pub mod connection;
pub mod widgets;
pub mod runtime;
use std::{
    cell::RefCell, fmt::Debug, sync::{atomic::{AtomicUsize, Ordering}, Arc, OnceLock}
};

pub use bincode;
pub use serde;
pub use iced;
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
                        // let _ = app.update();

                        let element = app.view(&view_theme.borrow());
                        connection
                            .write_frame(PluginRequest::View(Arc::new(element)))
                            .await
                            .unwrap();
                    }
                    PluginEvent::Message(items) => {
                        let message: A::Message = bincode::serde::decode_from_slice(&items, bincode::config::standard()).unwrap().0;
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

pub fn unique_id() -> usize {
    AtomicUsize::new(0).fetch_add(1, Ordering::Relaxed)
}