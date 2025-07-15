use std::borrow::Cow;

use iced::{
    self, Background, Border, Color, Gradient, Length, Padding, Pixels, Shadow, Size, Theme,
    alignment, border, color, gradient, widget::container,
};
use serde::{Deserialize, Serialize};

use crate::{
    PluginRequest,
    widgets::{
        element::{Element, to_element},
        serde_types::{
            BorderDef, HorizontalDef, LengthDef, PaddingDef, ShadowDef, VerticalDef,
            opt_background, opt_color,
        },
        widget::Widget,
    },
};

/// A widget that aligns its contents inside of its boundaries.
///
/// # Example
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } }
/// # pub type State = ();
/// # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
/// use iced::widget::container;
///
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> Element<'_, Message> {
///     container("This text is centered inside a rounded box!")
///         .padding(10)
///         .center(800)
///         .style(container::rounded_box)
///         .into()
/// }
/// ```
#[allow(missing_debug_implementations)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    max_width: f32,
    max_height: f32,
    id: Option<Id>,
    #[serde(with = "PaddingDef")]
    padding: Padding,
    #[serde(with = "LengthDef")]
    width: Length,
    #[serde(with = "LengthDef")]
    height: Length,
    #[serde(with = "HorizontalDef")]
    horizontal_alignment: alignment::Horizontal,
    #[serde(with = "VerticalDef")]
    vertical_alignment: alignment::Vertical,
    clip: bool,
    content: Element,
    class: Option<Style>,
}

impl Container {
    /// Creates a [`Container`] with the given content.
    pub fn new(content: impl Into<Element>) -> Self {
        let content = content.into();
        let size = content.as_widget().size_hint();

        Container {
            id: None,
            padding: Padding::ZERO,
            width: size.width.fluid(),
            height: size.height.fluid(),
            max_width: f32::INFINITY,
            max_height: f32::INFINITY,
            horizontal_alignment: alignment::Horizontal::Left,
            vertical_alignment: alignment::Vertical::Top,
            clip: false,
            class: None,
            content,
        }
    }

    /// Sets the [`Id`] of the [`Container`].
    pub fn id(mut self, id: Id) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the [`Padding`] of the [`Container`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the width of the [`Container`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Container`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the maximum width of the [`Container`].
    pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
        self.max_width = max_width.into().0;
        self
    }

    /// Sets the maximum height of the [`Container`].
    pub fn max_height(mut self, max_height: impl Into<Pixels>) -> Self {
        self.max_height = max_height.into().0;
        self
    }

    /// Sets the width of the [`Container`] and centers its contents horizontally.
    pub fn center_x(self, width: impl Into<Length>) -> Self {
        self.width(width).align_x(alignment::Horizontal::Center)
    }

    /// Sets the height of the [`Container`] and centers its contents vertically.
    pub fn center_y(self, height: impl Into<Length>) -> Self {
        self.height(height).align_y(alignment::Vertical::Center)
    }

    /// Centers the contents in both the horizontal and vertical axes of the
    /// [`Container`].
    ///
    /// This is equivalent to chaining [`center_x`] and [`center_y`].
    ///
    /// [`center_x`]: Self::center_x
    /// [`center_y`]: Self::center_y
    pub fn center(self, length: impl Into<Length>) -> Self {
        let length = length.into();
        self.center_x(length).center_y(length)
    }

    /// Aligns the contents of the [`Container`] to the left.
    pub fn align_left(self, width: impl Into<Length>) -> Self {
        self.width(width).align_x(alignment::Horizontal::Left)
    }

    /// Aligns the contents of the [`Container`] to the right.
    pub fn align_right(self, width: impl Into<Length>) -> Self {
        self.width(width).align_x(alignment::Horizontal::Right)
    }

    /// Aligns the contents of the [`Container`] to the top.
    pub fn align_top(self, height: impl Into<Length>) -> Self {
        self.height(height).align_y(alignment::Vertical::Top)
    }

    /// Aligns the contents of the [`Container`] to the bottom.
    pub fn align_bottom(self, height: impl Into<Length>) -> Self {
        self.height(height).align_y(alignment::Vertical::Bottom)
    }

    /// Sets the content alignment for the horizontal axis of the [`Container`].
    pub fn align_x(mut self, alignment: impl Into<alignment::Horizontal>) -> Self {
        self.horizontal_alignment = alignment.into();
        self
    }

    /// Sets the content alignment for the vertical axis of the [`Container`].
    pub fn align_y(mut self, alignment: impl Into<alignment::Vertical>) -> Self {
        self.vertical_alignment = alignment.into();
        self
    }

    /// Sets whether the contents of the [`Container`] should be clipped on
    /// overflow.
    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    /// Sets the class of the [`Container`].
    pub fn class(mut self, class: Style) -> Self {
        self.class = Some(class);
        self
    }

    /// Sets the style of the [`Container`].
    #[must_use]
    pub fn style(mut self, style: Style) -> Self {
        self.class = Some(style);
        self
    }
}

impl From<Container> for Element {
    fn from(value: Container) -> Self {
        Element::new(Box::new(value))
    }
}

impl<'a> From<Container> for iced::Element<'a, PluginRequest, iced::Theme, iced::Renderer>
{
    fn from(value: Container) -> Self {
        let mut container= container(to_element(&value.content))
            .width(value.width)
            .height(value.height)
            .style(move |theme| if let Some(class) = value.class {
                class.into()
            } else {
                iced::widget::container::transparent(theme)
            })
            .align_x(value.horizontal_alignment)
            .align_y(value.vertical_alignment)
            .clip(value.clip)
            .padding(value.padding);

        if let Some(id) = value.id {
            container = container.id(id.into())
        }

        container.into()
    }
}

#[typetag::serde]
impl Widget for Container {
    fn size_hint(&self) -> Size<Length> {
        self.content.as_widget().size_hint()
    }

    fn data(&self) -> Vec<u8> {
        bincode::serde::encode_to_vec(self, bincode::config::standard()).unwrap()
    }
}

/// The appearance of a container.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Style {
    /// The text [`Color`] of the container.
    #[serde(with = "opt_color")]
    pub text_color: Option<Color>,
    /// The [`Background`] of the container.
    #[serde(with = "opt_background")]
    pub background: Option<Background>,
    /// The [`Border`] of the container.
    #[serde(with = "BorderDef")]
    pub border: Border,
    /// The [`Shadow`] of the container.
    #[serde(with = "ShadowDef")]
    pub shadow: Shadow,
}

impl Style {
    /// Updates the text color of the [`Style`].
    pub fn color(self, color: impl Into<Color>) -> Self {
        Self {
            text_color: Some(color.into()),
            ..self
        }
    }

    /// Updates the border of the [`Style`].
    pub fn border(self, border: impl Into<Border>) -> Self {
        Self {
            border: border.into(),
            ..self
        }
    }

    /// Updates the background of the [`Style`].
    pub fn background(self, background: impl Into<Background>) -> Self {
        Self {
            background: Some(background.into()),
            ..self
        }
    }

    /// Updates the shadow of the [`Style`].
    pub fn shadow(self, shadow: impl Into<Shadow>) -> Self {
        Self {
            shadow: shadow.into(),
            ..self
        }
    }
}

impl From<Color> for Style {
    fn from(color: Color) -> Self {
        Self::default().background(color)
    }
}

impl From<Gradient> for Style {
    fn from(gradient: Gradient) -> Self {
        Self::default().background(gradient)
    }
}

impl From<gradient::Linear> for Style {
    fn from(gradient: gradient::Linear) -> Self {
        Self::default().background(gradient)
    }
}

impl From<Style> for iced::widget::container::Style {
    fn from(value: Style) -> Self {
        iced::widget::container::Style {
            text_color: value.text_color,
            background: value.background,
            border: value.border,
            shadow: value.shadow,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Id {
    Unique,
    Custom(Cow<'static, str>),
}

impl From<Id> for iced::widget::container::Id {
    fn from(value: Id) -> Self {
        match value {
            Id::Unique => iced::widget::container::Id::unique(),
            Id::Custom(id) => iced::widget::container::Id::new(id),
        }
    }
}

/// The theme catalog of a [`Container`].
pub trait Catalog {
    /// The item class of the [`Catalog`].
    type Class;

    /// The default class produced by the [`Catalog`].
    fn default() -> Self::Class;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class) -> Style;
}

/// A styling function for a [`Container`].
pub type StyleFn<Theme> = Box<dyn Fn(&Theme) -> Style>;

impl<Theme> From<Style> for StyleFn<Theme> {
    fn from(style: Style) -> Self {
        Box::new(move |_theme| style)
    }
}

impl Catalog for Theme {
    type Class = StyleFn<Self>;

    fn default() -> Self::Class {
        Box::new(transparent)
    }

    fn style(&self, class: &Self::Class) -> Style {
        class(self)
    }
}

/// A transparent [`Container`].
pub fn transparent<Theme>(_theme: &Theme) -> Style {
    Style::default()
}

/// A [`Container`] with the given [`Background`].
pub fn background(background: impl Into<Background>) -> Style {
    Style::default().background(background)
}

/// A rounded [`Container`] with a background.
pub fn rounded_box(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    Style {
        background: Some(palette.background.weak.color.into()),
        border: border::rounded(2),
        ..Style::default()
    }
}

/// A bordered [`Container`] with a background.
pub fn bordered_box(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    Style {
        background: Some(palette.background.weak.color.into()),
        border: Border {
            width: 1.0,
            radius: 0.0.into(),
            color: palette.background.strong.color,
        },
        ..Style::default()
    }
}

/// A [`Container`] with a dark background and white text.
pub fn dark(_theme: &Theme) -> Style {
    Style {
        background: Some(color!(0x111111).into()),
        text_color: Some(Color::WHITE),
        border: border::rounded(2),
        ..Style::default()
    }
}
