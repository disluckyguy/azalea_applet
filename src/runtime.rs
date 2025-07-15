use std::{
    collections::HashMap,
    sync::{Arc, mpsc::Sender},
};

use crate::{Element, PluginEvent, PluginRequest, RuntimeMessage};

#[derive(Debug, Default, Clone)]
pub struct PluginRuntime {
    pub plugins: HashMap<usize, Plugin>,
}

impl PluginRuntime {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    pub fn handle_plugin_message(&mut self, message: RuntimeMessage, theme: iced::Theme) {
        match message {
            RuntimeMessage::Request(message, id) => match message {
                PluginRequest::Message(items) => {
                    if let Some(sender) = self.plugins.get(&id).map(|p| &p.sender) {
                        println!("sending");
                        sender.send(PluginEvent::Message(items)).unwrap();
                    }
                }
                PluginRequest::View(element) => {
                    println!("view");
                    if let Some(plugin) = self.plugins.get_mut(&id) {
                        plugin.view = Some(element)
                    }
                }
            },

            RuntimeMessage::New(plugin, id) => {
                plugin.sender.send(PluginEvent::Theme(theme.into())).unwrap();
                self.plugins.insert(id, plugin);
            }
        }
    }

    pub fn views(&self) -> HashMap<usize, Arc<Element>> {
        self.plugins
            .iter()
            .filter_map(|(id, p)| p.view.clone().map(|v| (id.clone(), v)))
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct Plugin {
    pub sender: Sender<PluginEvent>,
    pub view: Option<Arc<Element>>,
}

impl Plugin {
    pub fn shutdown() {}

    pub fn restart() {}
}
