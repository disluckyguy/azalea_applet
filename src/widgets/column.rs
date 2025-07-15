use iced::{Alignment, Length, Padding, Pixels, Size, alignment};
use serde::{Deserialize, Serialize};

use crate::{
    widgets::{
        element::to_element,
        serde_types::{AlignmentDef, LengthDef, PaddingDef},
        widget::Widget,
    }, PluginRequest, Element
};

/// A container that distributes its contents vertically.
///
/// # Example
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } }
/// # pub type State = ();
/// # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
/// use iced::widget::{button, column};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> Element<'_, Message> {
///     column![
///         "I am on top!",
///         button("I am in the center!"),
///         "I am below.",
///     ].into()
/// }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Column {
    spacing: f32,
    #[serde(with = "PaddingDef")]
    padding: Padding,
    #[serde(with = "LengthDef")]
    width: Length,
    #[serde(with = "LengthDef")]
    height: Length,
    max_width: f32,
    #[serde(with = "AlignmentDef")]
    align: Alignment,
    clip: bool,
    children: Vec<Element>,
}

impl Column {
    /// Creates an empty [`Column`].
    pub fn new() -> Self {
        Self::from_vec(Vec::new())
    }

    /// Creates a [`Column`] with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self::from_vec(Vec::with_capacity(capacity))
    }

    /// Creates a [`Column`] with the given elements.
    pub fn with_children(children: impl IntoIterator<Item = Element>) -> Self {
        let iterator = children.into_iter();

        Self::with_capacity(iterator.size_hint().0).extend(iterator)
    }

    /// Creates a [`Column`] from an already allocated [`Vec`].
    ///
    /// Keep in mind that the [`Column`] will not inspect the [`Vec`], which means
    /// it won't automatically adapt to the sizing strategy of its contents.
    ///
    /// If any of the children have a [`Length::Fill`] strategy, you will need to
    /// call [`Column::width`] or [`Column::height`] accordingly.
    pub fn from_vec(children: Vec<Element>) -> Self {
        Self {
            spacing: 0.0,
            padding: Padding::ZERO,
            width: Length::Shrink,
            height: Length::Shrink,
            max_width: f32::INFINITY,
            align: Alignment::Start,
            clip: false,
            children,
        }
    }

    /// Sets the vertical spacing _between_ elements.
    ///
    /// Custom margins per element do not exist in iced. You should use this
    /// method instead! While less flexible, it helps you keep spacing between
    /// elements consistent.
    pub fn spacing(mut self, amount: impl Into<Pixels>) -> Self {
        self.spacing = amount.into().0;
        self
    }

    /// Sets the [`Padding`] of the [`Column`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the width of the [`Column`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Column`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the maximum width of the [`Column`].
    pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
        self.max_width = max_width.into().0;
        self
    }

    /// Sets the horizontal alignment of the contents of the [`Column`] .
    pub fn align_x(mut self, align: impl Into<alignment::Horizontal>) -> Self {
        self.align = Alignment::from(align.into());
        self
    }

    /// Sets whether the contents of the [`Column`] should be clipped on
    /// overflow.
    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    /// Adds an element to the [`Column`].
    pub fn push(mut self, child: impl Into<Element>) -> Self {
        let child = child.into();
        let child_size = child.as_widget().size_hint();

        self.width = self.width.enclose(child_size.width);
        self.height = self.height.enclose(child_size.height);

        self.children.push(child);
        self
    }

    /// Adds an element to the [`Column`], if `Some`.
    pub fn push_maybe(self, child: Option<impl Into<Element>>) -> Self {
        if let Some(child) = child {
            self.push(child)
        } else {
            self
        }
    }

    /// Extends the [`Column`] with the given children.
    pub fn extend(self, children: impl IntoIterator<Item = Element>) -> Self {
        children.into_iter().fold(self, Self::push)
    }
}

impl Default for Column {
    fn default() -> Self {
        Self::new()
    }
}

impl FromIterator<Element> for Column {
    fn from_iter<T: IntoIterator<Item = Element>>(iter: T) -> Self {
        Self::with_children(iter)
    }
}

#[typetag::serde]
impl Widget for Column {
    fn size_hint(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }
    fn data(&self) -> Vec<u8> {
        bincode::serde::encode_to_vec(self, bincode::config::standard()).unwrap()
    }
}

impl From<Column> for Element {
    fn from(column: Column) -> Self {
        Self::new(Box::new(column))
    }
}

impl<'a> From<Column> for iced::Element<'a, PluginRequest, iced::Theme, iced::Renderer>
{
    fn from(value: Column) -> Self {
        iced::widget::Column::with_children(value.children.iter().map(|c| to_element(c)))
            .align_x(value.align)
            .clip(value.clip)
            .width(value.width)
            .height(value.height)
            .max_width(value.max_width)
            .into()
    }
}
