//! Text widgets display information through writing.
//!
//! # Example
//! ```no_run
//! # mod iced { pub mod widget { pub fn text<T>(t: T) -> iced_core::widget::Text<'static, iced_core::Theme, ()> { unimplemented!() } }
//! #            pub use iced_core::color; }
//! # pub type State = ();
//! # pub type Element<'a, Message> = iced_core::Element<'a, Message, iced_core::Theme, ()>;
//! use iced::widget::text;
//! use iced::color;
//!
//! enum Message {
//!     // ...
//! }
//!
//! fn view(state: &State) -> Element<'_, Message> {
//!     text("Hello, this is iced!")
//!         .size(20)
//!         .color(color!(0x0000ff))
//!         .into()
//! }
//! ```
use iced::{Color, Length, Pixels, Size, Theme, alignment};

pub use iced::widget::text::{self, LineHeight, Shaping, Wrapping};
use serde::{Deserialize, Serialize};

use crate::{
    PluginRequest, Element,
    widgets::{
        serde_types::{
            Font, HorizontalDef, LengthDef, LineHeightDef, ShapingDef, VerticalDef,
            WrappingDef, opt_color, opt_pixels,
        },
        widget::Widget,
    },
};

/// A bunch of text.
///
/// # Example
/// ```no_run
/// # mod iced { pub mod widget { pub fn text<T>(t: T) -> iced_core::widget::Text<'static, iced_core::Theme, ()> { unimplemented!() } }
/// #            pub use iced_core::color; }
/// # pub type State = ();
/// # pub type Element<'a, Message> = iced_core::Element<'a, Message, iced_core::Theme, ()>;
/// use iced::widget::text;
/// use iced::color;
///
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> Element<'_, Message> {
///     text("Hello, this is iced!")
///         .size(20)
///         .color(color!(0x0000ff))
///         .into()
/// }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Text {
    fragment: String,
    #[serde(with = "opt_pixels")]
    size: Option<Pixels>,
    #[serde(with = "LineHeightDef")]
    line_height: LineHeight,
    #[serde(with = "LengthDef")]
    width: Length,
    #[serde(with = "LengthDef")]
    height: Length,
    #[serde(with = "HorizontalDef")]
    horizontal_alignment: alignment::Horizontal,
    #[serde(with = "VerticalDef")]
    vertical_alignment: alignment::Vertical,
    font: Option<Font>,
    #[serde(with = "ShapingDef")]
    shaping: Shaping,
    #[serde(with = "WrappingDef")]
    wrapping: Wrapping,
    class: Option<Style>,
}

impl Text {
    /// Create a new fragment of [`Text`] with the given contents.
    pub fn new(fragment: impl Into<String>) -> Self {
        Text {
            fragment: fragment.into(),
            size: None,
            line_height: LineHeight::default(),
            font: None,
            width: Length::Shrink,
            height: Length::Shrink,
            horizontal_alignment: alignment::Horizontal::Left,
            vertical_alignment: alignment::Vertical::Top,
            shaping: Shaping::default(),
            wrapping: Wrapping::default(),
            class: None,
        }
    }

    /// Sets the size of the [`Text`].
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = Some(size.into());
        self
    }

    /// Sets the [`LineHeight`] of the [`Text`].
    pub fn line_height(mut self, line_height: impl Into<LineHeight>) -> Self {
        self.line_height = line_height.into();
        self
    }

    /// Sets the [`Font`] of the [`Text`].
    ///
    /// [`Font`]: crate::text::Renderer::Font
    pub fn font(mut self, font: impl Into<Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// Sets the width of the [`Text`] boundaries.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Text`] boundaries.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Centers the [`Text`], both horizontally and vertically.
    pub fn center(self) -> Self {
        self.align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
    }

    /// Sets the [`alignment::Horizontal`] of the [`Text`].
    pub fn align_x(mut self, alignment: impl Into<alignment::Horizontal>) -> Self {
        self.horizontal_alignment = alignment.into();
        self
    }

    /// Sets the [`alignment::Vertical`] of the [`Text`].
    pub fn align_y(mut self, alignment: impl Into<alignment::Vertical>) -> Self {
        self.vertical_alignment = alignment.into();
        self
    }

    /// Sets the [`Shaping`] strategy of the [`Text`].
    pub fn shaping(mut self, shaping: Shaping) -> Self {
        self.shaping = shaping;
        self
    }

    /// Sets the [`Wrapping`] strategy of the [`Text`].
    pub fn wrapping(mut self, wrapping: Wrapping) -> Self {
        self.wrapping = wrapping;
        self
    }

    /// Sets the style of the [`Text`].
    #[must_use]
    pub fn style(mut self, style: Style) -> Self {
        self.class = Some(style);
        self
    }

    /// Sets the [`Color`] of the [`Text`].
    pub fn color(self, color: impl Into<Color>) -> Self {
        self.color_maybe(Some(color))
    }

    /// Sets the [`Color`] of the [`Text`], if `Some`.
    pub fn color_maybe(self, color: Option<impl Into<Color>>) -> Self {
        let color = color.map(Into::into);

        self.style(Style { color })
    }

    /// Sets the style class of the [`Text`].
    #[must_use]
    pub fn class(mut self, class: Style) -> Self {
        self.class = class.into();
        self
    }
}

#[typetag::serde]
impl Widget for Text {
    fn size_hint(&self) -> iced::Size<iced::Length> {
        Size::new(self.width, self.height)
    }

    fn data(&self) -> Vec<u8> {
        bincode::serde::encode_to_vec(self, bincode::config::standard()).unwrap()
    }
}

impl From<Text> for Element {
    fn from(value: Text) -> Element {
        Element::new(Box::new(value))
    }
}

impl<'a> From<Text> for iced::Element<'a, PluginRequest, iced::Theme, iced::Renderer> {
    fn from(value: Text) -> iced::Element<'a, PluginRequest, Theme, iced::Renderer> {
        let mut text = iced::widget::Text::new(value.fragment)
            .align_x(value.horizontal_alignment)
            .align_y(value.vertical_alignment)
            .shaping(value.shaping)
            .line_height(value.line_height)
            .width(value.width)
            .height(value.height)
            .wrapping(value.wrapping)
            .style(move |theme| if let Some(class) = value.class {
                class.into()
            } else {
                iced::widget::text::default(theme)
            });
        if let Some(font) = value.font {
            text = text.font(font)
        }
        if let Some(size) = value.size {
            text = text.size(size)
        }
        text.into()
    }
}

impl<'a> From<&'a str> for Text {
    fn from(content: &'a str) -> Self {
        Self::new(content)
    }
}

impl<'a> From<&'a str> for Element {
    fn from(content: &'a str) -> Self {
        Text::from(content).into()
    }
}

/// The appearance of some text.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Style {
    /// The [`Color`] of the text.
    ///
    /// The default, `None`, means using the inherited color.
    #[serde(with = "opt_color")]
    pub color: Option<Color>,
}

impl From<Style> for text::Style {
    fn from(value: Style) -> Self {
        Self { color: value.color }
    }
}
/// The theme catalog of a [`Text`].
pub trait Catalog: Sized {
    /// The item class of this [`Catalog`].
    type Class<'a>;

    /// The default class produced by this [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, item: &Self::Class<'_>) -> Style;
}

/// A styling function for a [`Text`].
///
/// This is just a boxed closure: `Fn(&Theme, Status) -> Style`.
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|_theme| Style::default())
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

/// The default text styling; color is inherited.
pub fn default(_theme: &Theme) -> Style {
    Style { color: None }
}

/// Text with the default base color.
pub fn base(theme: &Theme) -> Style {
    Style {
        color: Some(theme.palette().text),
    }
}

/// Text conveying some important information, like an action.
pub fn primary(theme: &Theme) -> Style {
    Style {
        color: Some(theme.palette().primary),
    }
}

/// Text conveying some secondary information, like a footnote.
pub fn secondary(theme: &Theme) -> Style {
    Style {
        color: Some(theme.extended_palette().secondary.strong.color),
    }
}

/// Text conveying some positive information, like a successful event.
pub fn success(theme: &Theme) -> Style {
    Style {
        color: Some(theme.palette().success),
    }
}

/// Text conveying some negative information, like an error.
pub fn danger(theme: &Theme) -> Style {
    Style {
        color: Some(theme.palette().danger),
    }
}
