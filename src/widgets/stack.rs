use iced::{Length, Size};
use serde::{Deserialize, Serialize};

use crate::{widgets::{element::{to_element, Element}, serde_types::*, widget::Widget}, PluginRequest};

/// A container that displays children on top of each other.
///
/// The first [`Element`] dictates the intrinsic [`Size`] of a [`Stack`] and
/// will be displayed as the base layer. Every consecutive [`Element`] will be
/// renderer on top; on its own layer.
///
/// Keep in mind that too much layering will normally produce bad UX as well as
/// introduce certain rendering overhead. Use this widget sparingly!
#[derive(Serialize, Deserialize, Debug)]
pub struct Stack {
    #[serde(with = "LengthDef")]
    pub width: Length,
    #[serde(with = "LengthDef")]
    pub height: Length,
    pub children: Vec<Element>,
}

impl Stack {
    /// Creates an empty [`Stack`].
    pub fn new() -> Self {
        Self::from_vec(Vec::new())
    }

    /// Creates a [`Stack`] with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self::from_vec(Vec::with_capacity(capacity))
    }

    /// Creates a [`Stack`] with the given elements.
    pub fn with_children(children: impl IntoIterator<Item = Element>) -> Self {
        let iterator = children.into_iter();

        Self::with_capacity(iterator.size_hint().0).extend(iterator)
    }

    /// Creates a [`Stack`] from an already allocated [`Vec`].
    ///
    /// Keep in mind that the [`Stack`] will not inspect the [`Vec`], which means
    /// it won't automatically adapt to the sizing strategy of its contents.
    ///
    /// If any of the children have a [`Length::Fill`] strategy, you will need to
    /// call [`Stack::width`] or [`Stack::height`] accordingly.
    pub fn from_vec(children: Vec<Element>) -> Self {
        Self {
            width: Length::Shrink,
            height: Length::Shrink,
            children,
        }
    }

    /// Sets the width of the [`Stack`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Stack`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Adds an element to the [`Stack`].
    pub fn push(mut self, child: impl Into<Element>) -> Self {
        let child = child.into();

        if self.children.is_empty() {
            let child_size = child.as_widget().size_hint();

            self.width = self.width.enclose(child_size.width);
            self.height = self.height.enclose(child_size.height);
        }

        self.children.push(child);
        self
    }

    /// Adds an element to the [`Stack`], if `Some`.
    pub fn push_maybe(self, child: Option<impl Into<Element>>) -> Self {
        if let Some(child) = child {
            self.push(child)
        } else {
            self
        }
    }

    /// Extends the [`Stack`] with the given children.
    pub fn extend(self, children: impl IntoIterator<Item = Element>) -> Self {
        children.into_iter().fold(self, Self::push)
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

#[typetag::serde]
impl Widget for Stack {
    fn size_hint(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn data(&self) -> Vec<u8> {
        bincode::serde::encode_to_vec(self, bincode::config::standard()).unwrap()
    }
}

impl From<Stack> for Element {
    fn from(stack: Stack) -> Self {
        Self::new(Box::new(stack))
    }
}

impl<'a> From<Stack> for iced::Element<'a , PluginRequest, iced::Theme, iced::Renderer>

{
    fn from(value: Stack) -> Self {
        iced::widget::Stack::from_vec(value.children.into_iter().map(|e| to_element(&e)).collect())
            .width(value.width)
            .height(value.height)
            .into()
    }
}
