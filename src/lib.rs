pub mod connection;
pub mod runtime;
pub mod widgets;
use std::{
    cell::RefCell,
    fmt::Debug,
    sync::{
        Arc, OnceLock,
        atomic::{AtomicUsize, Ordering},
    },
};

pub use bincode;
pub use iced;
pub use serde;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use tokio::{net::UnixStream, runtime::Runtime};
pub use widgets::element::Element;

use crate::{connection::Connection, runtime::Plugin, widgets::serde_types::Theme};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginEvent {
    Update,
    Message(Vec<u8>),
    Theme(Theme),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginRequest {
    View(Arc<Element>),
    Message(Vec<u8>),
}
#[derive(Debug, Clone)]
pub enum RuntimeMessage {
    New(Plugin, usize),
    Request(PluginRequest, usize),
    Shutdown(usize),
}

pub trait Application {
    type Message: Sync + Send + Serialize + DeserializeOwned + Debug + Clone;
    fn new() -> Self;
    fn update(&mut self, message: Self::Message);
    fn view(&self, theme: &Theme) -> Element;
}

pub fn run<A>(mut app: A)
where
    A: Application,
{
    tokio_runtime().block_on(async move {
        let stream = UnixStream::connect("/tmp/sock").await.unwrap();

        let mut connection = Connection::new(stream);
        let view_theme = RefCell::new(Theme::from(iced::Theme::default()));
        loop {
            if let Ok(Some(event)) = connection.read_frame().await {
                match event {
                    PluginEvent::Update => {

                        let element = app.view(&view_theme.borrow());
                        connection
                            .write_frame(PluginRequest::View(Arc::new(element)))
                            .await
                            .unwrap();
                    }
                    PluginEvent::Message(items) => {
                        let message: A::Message =
                            bincode::serde::decode_from_slice(&items, bincode::config::standard())
                                .unwrap()
                                .0;
                        let _ = app.update(message);

                        let element = app.view(&view_theme.borrow());
                        connection
                            .write_frame(PluginRequest::View(Arc::new(element)))
                            .await
                            .unwrap();
                    }
                    PluginEvent::Theme(theme) => {
                        *view_theme.borrow_mut() = theme;
                        let element = app.view(&view_theme.borrow());
                        connection
                            .write_frame(PluginRequest::View(Arc::new(element)))
                            .await
                            .unwrap();
                    }
                }
            }
        }
    });
}

pub(crate) fn tokio_runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."))
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);
fn unique_id() -> usize {
    COUNTER.fetch_add(1, Ordering::Relaxed)
}
