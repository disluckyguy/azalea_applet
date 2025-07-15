//! Buttons allow your users to perform actions by pressing them.
//!
//! # Example
//! ```no_run
//! # mod iced { pub mod widget { pub use iced_widget::*; } }
//! # pub type State = ();
//! # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
//! use iced::widget::button;
//!
//! #[derive(Clone)]
//! enum Message {
//!     ButtonPressed,
//! }
//!
//! fn view(state: &State) -> Element<'_, Message> {
//!     button("Press me!").on_press(Message::ButtonPressed).into()
//! }
//! ```
use std::fmt::Debug;

use crate::widgets::serde_types::{
    BorderDef, ColorDef, LengthDef, PaddingDef, ShadowDef, opt_background,
};
use iced::border::{self, Border};
use iced::theme::palette;
use iced::widget::button;
use iced::{Background, Color, Length, Padding, Shadow, Size, Theme};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::widgets::element::to_element;
use crate::widgets::widget::Widget;
use crate::{PluginRequest, Element};

/// A generic widget that produces a message when pressed.
///
/// # Example
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } }
/// # pub type State = ();
/// # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
/// use iced::widget::button;
///
/// #[derive(Clone)]
/// enum Message {
///     ButtonPressed,
/// }
///
/// fn view(state: &State) -> Element<'_, Message> {
///     button("Press me!").on_press(Message::ButtonPressed).into()
/// }
/// ```
///
/// If a [`Button::on_press`] handler is not set, the resulting [`Button`] will
/// be disabled:
///
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } }
/// # pub type State = ();
/// # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
/// use iced::widget::button;
///
/// #[derive(Clone)]
/// enum Message {
///     ButtonPressed,
/// }
///
/// fn view(state: &State) -> Element<'_, Message> {
///     button("I am disabled!").into()
/// }
/// ```

#[derive(Debug, Serialize, Deserialize)]
pub struct Button {
    content: Element,
    on_press: Option<Vec<u8>>,
    #[serde(with = "LengthDef")]
    width: Length,
    #[serde(with = "LengthDef")]
    height: Length,
    #[serde(with = "PaddingDef")]
    padding: Padding,
    clip: bool,
    class: Option<StateStyle>,
}

impl Button {
    /// Creates a new [`Button`] with the given content.
    pub fn new(content: impl Into<Element>) -> Self {
        let content = content.into();
        let size = content.as_widget().size_hint();

        Button {
            content,
            on_press: None,
            width: size.width.fluid(),
            height: size.height.fluid(),
            padding: DEFAULT_PADDING,
            clip: false,
            class: None,
        }
    }

    /// Sets the width of the [`Button`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Button`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the [`Padding`] of the [`Button`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the message that will be produced when the [`Button`] is pressed.
    ///
    /// Unless `on_press` is called, the [`Button`] will be disabled.
    pub fn on_press<Message>(mut self, on_press: Message) -> Self
    where
        Message: Send + Sync + Serialize + DeserializeOwned,
    {
        self.on_press =
            Some(bincode::serde::encode_to_vec(on_press, bincode::config::standard()).unwrap());
        self
    }

    /// Sets the message that will be produced when the [`Button`] is pressed.
    ///
    /// This is analogous to [`Button::on_press`], but using a closure to produce
    /// the message.
    ///
    /// This closure will only be called when the [`Button`] is actually pressed and,
    /// therefore, this method is useful to reduce overhead if creating the resulting
    /// message is slow.
    pub fn on_press_with<Message>(mut self, on_press: impl Fn() -> Message) -> Self
    where
        Message: Send + Sync + Serialize + DeserializeOwned,
    {
        self.on_press =
            Some(bincode::serde::encode_to_vec(on_press(), bincode::config::standard()).unwrap());
        self
    }

    /// Sets the message that will be produced when the [`Button`] is pressed,
    /// if `Some`.
    ///
    /// If `None`, the [`Button`] will be disabled.
    pub fn on_press_maybe<Message>(mut self, on_press: Option<Message>) -> Self
    where
        Message: Send + Sync + Serialize + DeserializeOwned,
    {
        self.on_press =
            Some(bincode::serde::encode_to_vec(on_press, bincode::config::standard()).unwrap());
        self
    }

    /// Sets whether the contents of the [`Button`] should be clipped on
    /// overflow.
    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    /// Sets the style of the [`Button`].
    #[must_use]
    pub fn style(mut self, style: StateStyle) -> Self {
        self.class = Some(style);
        self
    }

    /// Sets the style class of the [`Button`]
    pub fn class(mut self, class: StateStyle) -> Self {
        self.class = Some(class);
        self
    }
}

#[expect(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct State {
    is_pressed: bool,
}

#[typetag::serde]
impl Widget for Button {
    fn size_hint(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn data(&self) -> Vec<u8> {
        bincode::serde::encode_to_vec(self, bincode::config::standard()).unwrap()
    }
}

impl<'a> From<Button> for Element {
    fn from(value: Button) -> Self {
        Element::new(Box::new(value))
    }
}

impl<'a> From<Button> for iced::Element<'a, PluginRequest, iced::Theme, iced::Renderer>
{
    fn from(value: Button) -> Self {
        iced::widget::Button::new(to_element(&value.content))
            .on_press_maybe(value.on_press.map(|b| PluginRequest::Message(b)))
            .width(value.width)
            .height(value.height)
            .padding(value.padding)
            .style(move |theme, status| {
                if let Some(class) = value.class {
                    let style = match status {
                        button::Status::Active => class.active,
                        button::Status::Hovered => class.hovered,
                        button::Status::Pressed => class.pressed,
                        button::Status::Disabled => class.disabled,
                    };
                    style.into()
                } else {
                    button::primary(&theme, status)
                }
            })
            .clip(value.clip)
            .into()
    }
}

/// The default [`Padding`] of a [`Button`].
pub(crate) const DEFAULT_PADDING: Padding = Padding {
    top: 5.0,
    bottom: 5.0,
    right: 10.0,
    left: 10.0,
};

/// The possible status of a [`Button`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// The [`Button`] can be pressed.
    Active,
    /// The [`Button`] can be pressed and it is being hovered.
    Hovered,
    /// The [`Button`] is being pressed.
    Pressed,
    /// The [`Button`] cannot be pressed.
    Disabled,
}

/// The style of a button.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StateStyle {
    pub active: Style,
    pub hovered: Style,
    pub pressed: Style,
    pub disabled: Style,
}

/// The style of a button.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Style {
    /// The [`Background`] of the button.
    #[serde(with = "opt_background")]
    pub background: Option<Background>,
    /// The text [`Color`] of the button.
    #[serde(with = "ColorDef")]
    pub text_color: Color,
    /// The [`Border`] of the buton.
    #[serde(with = "BorderDef")]
    pub border: Border,
    /// The [`Shadow`] of the butoon.
    #[serde(with = "ShadowDef")]
    pub shadow: Shadow,
}

impl Style {
    /// Updates the [`Style`] with the given [`Background`].
    pub fn with_background(self, background: impl Into<Background>) -> Self {
        Self {
            background: Some(background.into()),
            ..self
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            background: None,
            text_color: Color::BLACK,
            border: Border::default(),
            shadow: Shadow::default(),
        }
    }
}

impl From<Style> for iced::widget::button::Style {
    fn from(value: Style) -> Self {
        Self {
            background: value.background,
            text_color: value.text_color,
            border: value.border,
            shadow: value.shadow,
        }
    }
}

/// The theme catalog of a [`Button`].
pub trait Catalog {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

/// A styling function for a [`Button`].
pub type StyleFn<Theme> = Box<dyn Fn(&Theme, Status) -> Style>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// A primary button; denoting a main action.
pub fn primary(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = styled(palette.primary.strong);

    match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => Style {
            background: Some(Background::Color(palette.primary.base.color)),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

/// A secondary button; denoting a complementary action.
pub fn secondary(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = styled(palette.secondary.base);

    match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => Style {
            background: Some(Background::Color(palette.secondary.strong.color)),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

/// A success button; denoting a good outcome.
pub fn success(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = styled(palette.success.base);

    match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => Style {
            background: Some(Background::Color(palette.success.strong.color)),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

/// A danger button; denoting a destructive action.
pub fn danger(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = styled(palette.danger.base);

    match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => Style {
            background: Some(Background::Color(palette.danger.strong.color)),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

/// A text button; useful for links.
pub fn text(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    let base = Style {
        text_color: palette.background.base.text,
        ..Style::default()
    };

    match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => Style {
            text_color: palette.background.base.text.scale_alpha(0.8),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

fn styled(pair: palette::Pair) -> Style {
    Style {
        background: Some(Background::Color(pair.color)),
        text_color: pair.text,
        border: border::rounded(2),
        ..Style::default()
    }
}

fn disabled(style: Style) -> Style {
    Style {
        background: style
            .background
            .map(|background| background.scale_alpha(0.5)),
        text_color: style.text_color.scale_alpha(0.5),
        ..style
    }
}
