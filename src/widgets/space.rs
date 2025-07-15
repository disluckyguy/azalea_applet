use crate::{widgets::{
    element::Element,
    widget::Widget,
}, PluginRequest};
use iced::{Length, Size};
use serde::{Deserialize, Serialize};

use super::serde_types::LengthDef;

#[derive(Debug, Serialize, Deserialize)]
pub struct Space {
    #[serde(with = "LengthDef")]
    pub width: Length,
    #[serde(with = "LengthDef")]
    pub height: Length,
}

impl Space {
    /// Creates an amount of empty [`Space`] with the given width and height.
    pub fn new(width: impl Into<Length>, height: impl Into<Length>) -> Self {
        Space {
            width: width.into(),
            height: height.into(),
        }
    }

    /// Creates an amount of horizontal [`Space`].
    pub fn with_width(width: impl Into<Length>) -> Self {
        Space {
            width: width.into(),
            height: Length::Shrink,
        }
    }

    /// Creates an amount of vertical [`Space`].
    pub fn with_height(height: impl Into<Length>) -> Self {
        Space {
            width: Length::Shrink,
            height: height.into(),
        }
    }

    /// Sets the width of the [`Space`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Space`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }
}

#[typetag::serde]
impl Widget for Space {
    fn size_hint(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn data(&self) -> Vec<u8> {
        bincode::serde::encode_to_vec(self, bincode::config::standard()).unwrap()
    }
}

impl From<Space> for Element {
    fn from(value: Space) -> Self {
        Element::new(Box::new(value))
    }
}

impl<'a, Renderer> From<Space> for iced::Element<'a, PluginRequest, iced::Theme, Renderer>
    where Renderer: iced_core::Renderer + 'a
{
    fn from(value: Space) -> Self {
        iced::widget::Space::new(value.width, value.height).into()
    }
}
