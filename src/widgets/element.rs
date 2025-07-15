use serde::{Deserialize, Serialize};

use crate::{widgets::{button::Button, column::Column, container::Container, space::Space, stack::Stack, text::Text, widget::Widget}, PluginRequest};

#[derive(Serialize, Deserialize, Debug)]
pub struct Element {
    widget: Box<dyn Widget>,
}

impl Element {
    pub fn new(widget: Box<dyn Widget>) -> Self {
        Element { widget }
    }
    pub fn as_widget(&self) -> &dyn Widget {
        self.widget.as_ref()
    }
}

pub fn to_element<'a>(
    element: &Element,
) -> iced::Element<'a, PluginRequest, iced::Theme, iced::Renderer>
{
    let slice = element.as_widget().data();
    let type_name = &element.as_widget().typetag_name().to_string();
    match type_name.as_str() {
        "Container" => {
            let (state, _): (Container, _) =
                bincode::serde::decode_from_slice(&slice, bincode::config::standard()).unwrap();
            state.into()
        }
        "Space" => {
            let (state, _): (Space, _) =
                bincode::serde::decode_from_slice(&slice, bincode::config::standard()).unwrap();
            state.into()
        }
        "Stack" => {
            let (state, _): (Stack, _) =
                bincode::serde::decode_from_slice(&slice, bincode::config::standard()).unwrap();
            state.into()
        }
        "Button" => {
            let (state, _): (Button, _) =
                bincode::serde::decode_from_slice(&slice, bincode::config::standard()).unwrap();
            state.into()
        }
        "Text" => {
            let (state, _): (Text, _) =
                bincode::serde::decode_from_slice(&slice, bincode::config::standard()).unwrap();
            state.into()
        }
        "Column" => {
            let (state, _): (Column, _) =
                bincode::serde::decode_from_slice(&slice, bincode::config::standard()).unwrap();
            state.into()
        }
        _ => todo!(),
    }
}
