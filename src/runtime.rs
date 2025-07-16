use std::sync::{Arc};

use tokio::sync::mpsc::UnboundedSender;

use crate::{Element, PluginEvent, PluginRequest, RuntimeMessage, widgets::element::to_element};

#[derive(Debug, Default, Clone)]
pub struct PluginRuntime {
    pub plugins: Vec<Plugin>,
}

impl PluginRuntime {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn handle_plugin_message(&mut self, message: RuntimeMessage, theme: iced::Theme) {
        match message {
            RuntimeMessage::Request(message, id) => match message {
                PluginRequest::Message(items) => {
                    if let Some(sender) =
                        self.plugins.iter().find(|p| p.id == id).map(|p| &p.sender)
                    {
                        sender.send(PluginEvent::Message(items)).unwrap();
                    }
                }
                PluginRequest::View(element) => {
                    if let Some(plugin) = self.plugins.iter_mut().find(|p| p.id == id) {
                        plugin.view = Some(element)
                    }
                }
            },
            RuntimeMessage::New(plugin, id) => {
                plugin
                    .sender
                    .send(PluginEvent::Theme(theme.into()))
                    .unwrap();
                self.plugins.insert(id, plugin);
            }
            RuntimeMessage::Shutdown(id) => {
                self.plugins.retain(|p| p.id != id);
            }
        }
    }

    pub fn views(&self) -> Vec<iced::Element<RuntimeMessage, iced::Theme, iced::Renderer>> {
        self.plugins
            .iter()
            .filter_map(|p| {
                p.view
                    .clone()
                    .map(|v| to_element(&v).map(|m| RuntimeMessage::Request(m, p.id)))
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct Plugin {
    pub id: usize,
    pub sender: UnboundedSender<PluginEvent>,
    pub view: Option<Arc<Element>>,
}

impl Plugin {
    pub fn shutdown() {}

    pub fn restart() {}
}
