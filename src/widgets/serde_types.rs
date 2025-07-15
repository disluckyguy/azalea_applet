use std::borrow::Cow;

use iced::{
    widget::text::{LineHeight, Shaping, Wrapping},
    alignment::{Horizontal, Vertical, Alignment}, border::Radius, font::{Stretch, Style, Weight}, gradient::{ColorStop, Linear}, Background, Border, Color, Gradient, Length, Padding, Pixels, Radians, Shadow, Vector,
    theme::{Palette, palette::{Extended, Primary, Pair, Secondary, Success, Danger, self}}
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(remote = "Length")]
pub enum LengthDef {
    /// Fill all the remaining space
    Fill,

    /// Fill a portion of the remaining space relative to other elements.
    ///
    /// Let's say we have two elements: one with `FillPortion(2)` and one with
    /// `FillPortion(3)`. The first will get 2 portions of the available space,
    /// while the second one would get 3.
    ///
    /// `Length::Fill` is equivalent to `Length::FillPortion(1)`.
    FillPortion(u16),

    /// Fill the least amount of space
    Shrink,

    /// Fill a fixed amount of space
    Fixed(f32),
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Padding")]
pub struct PaddingDef {
    /// Top padding
    pub top: f32,
    /// Right padding
    pub right: f32,
    /// Bottom padding
    pub bottom: f32,
    /// Left padding
    pub left: f32,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Vertical")]
pub enum VerticalDef {
    /// Align top
    Top,

    /// Vertically centered
    Center,

    /// Align bottom
    Bottom,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Horizontal")]
pub enum HorizontalDef {
    /// Align left
    Left,

    /// Horizontally centered
    Center,

    /// Align right
    Right,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Shadow")]
pub struct ShadowDef {
    /// The color of the shadow.
    #[serde(with = "ColorDef")]
    pub color: Color,

    /// The offset of the shadow.
    #[serde(with = "VectorDef")]
    pub offset: Vector,

    /// The blur radius of the shadow.
    pub blur_radius: f32,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Vector")]
pub struct VectorDef<T = f32> {
    /// The X component of the [`Vector`]
    pub x: T,

    /// The Y component of the [`Vector`]
    pub y: T,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Color")]
pub struct ColorDef {
    /// Red component, 0.0 - 1.0
    pub r: f32,
    /// Green component, 0.0 - 1.0
    pub g: f32,
    /// Blue component, 0.0 - 1.0
    pub b: f32,
    /// Transparency, 0.0 - 1.0
    pub a: f32,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Border")]
pub struct BorderDef {
    /// The color of the border. Str
    #[serde(with = "ColorDef")]
    pub color: Color,

    /// The width of the border.
    pub width: f32,

    /// The [`Radius`] of the border.
    #[serde(with = "RadiusDef")]
    pub radius: Radius,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Radius")]
pub struct RadiusDef {
    /// Top left radius
    pub top_left: f32,
    /// Top right radius
    pub top_right: f32,
    /// Bottom right radius
    pub bottom_right: f32,
    /// Bottom left radius
    pub bottom_left: f32,
}

pub mod opt_color {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(value: &Option<Color>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "ColorDef")] &'a Color);

        value.as_ref().map(Helper).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Color>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper(#[serde(with = "ColorDef")] Color);

        let helper = Option::deserialize(deserializer)?;
        Ok(helper.map(|Helper(external)| external))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Background")]
pub enum BackgroundDef {
    /// A solid color.
    #[serde(with = "ColorDef")]
    Color(Color),
    /// Linearly interpolate between several colors.
    #[serde(with = "GradientDef")]
    Gradient(Gradient),
    // TODO: Add image variant
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Gradient")]
pub enum GradientDef {
    /// A linear gradient interpolates colors along a direction at a specific angle.
    #[serde(with = "LinearDef")]
    Linear(Linear),
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Radians")]
pub struct RadiansDef(pub f32);

#[derive(Serialize, Deserialize)]
#[serde(remote = "ColorStop")]
pub struct ColorStopDef {
    /// Offset along the gradient vector.
    pub offset: f32,

    /// The color of the gradient at the specified [`offset`].
    ///
    /// [`offset`]: Self::offset
    #[serde(with = "ColorDef")]
    pub color: Color,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Linear")]
pub struct LinearDef {
    /// How the [`Gradient`] is angled within its bounds.
    #[serde(with = "RadiansDef")]
    pub angle: Radians,
    /// [`ColorStop`]s along the linear gradient path.
    #[serde(with = "slice_opt_color_stop")]
    pub stops: [Option<ColorStop>; 8],
}

pub mod opt_background {
    use super::*;
    use iced::Background;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(value: &Option<Background>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "BackgroundDef")] &'a Background);

        value.as_ref().map(Helper).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Background>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper(#[serde(with = "BackgroundDef")] Background);

        let helper = Option::deserialize(deserializer)?;
        Ok(helper.map(|Helper(external)| external))
    }
}

pub mod opt_color_stop {
    use super::*;
    use iced::gradient::ColorStop;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(value: &Option<ColorStop>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "ColorStopDef")] &'a ColorStop);

        value.as_ref().map(Helper).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<ColorStop>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper(#[serde(with = "ColorStopDef")] ColorStop);

        let helper = Option::deserialize(deserializer)?;
        Ok(helper.map(|Helper(external)| external))
    }
}

pub mod slice_opt_color_stop {
    use super::*;
    use iced::gradient::ColorStop;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(value: &[Option<ColorStop>; 8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "opt_color_stop")] &'a Option<ColorStop>);

        value
            .iter()
            .map(Helper)
            .collect::<Vec<_>>()
            .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<[Option<ColorStop>; 8], D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper(#[serde(with = "opt_color_stop")] Option<ColorStop>);

        let helper = Vec::deserialize(deserializer)?;
        let vec = helper
            .into_iter()
            .map(|Helper(external)| external)
            .collect::<Vec<_>>();
        Ok([
            vec[0], vec[1], vec[2], vec[3], vec[4], vec[5], vec[6], vec[7],
        ])
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Id {
    Unique,
    Custom(Cow<'static, str>),
}

impl From<Id> for iced::advanced::widget::Id {
    fn from(value: Id) -> Self {
        match value {
            Id::Unique => iced::advanced::widget::Id::unique(),
            Id::Custom(id) => iced::advanced::widget::Id::new(id),
        }
    }
}

impl Id {
    /// Creates a custom [`Id`].
    pub fn new(id: impl Into<std::borrow::Cow<'static, str>>) -> Self {
        Self::Custom(id.into())
    }

    /// Creates a unique [`Id`].
    ///
    /// This function produces a different [`Id`] every time it is called.
    pub fn unique() -> Self {
        Self::Unique
    }
}


/// A font family.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum Family {
    /// The name of a font family of choice.
    Name(String),

    /// Serif fonts represent the formal text style for a script.
    Serif,

    /// Glyphs in sans-serif fonts, as the term is used in CSS, are generally low
    /// contrast and have stroke endings that are plain — without any flaring,
    /// cross stroke, or other ornamentation.
    SansSerif,

    /// Glyphs in cursive fonts generally use a more informal script style, and
    /// the result looks more like handwritten pen or brush writing than printed
    /// letterwork.
    Cursive,

    /// Fantasy fonts are primarily decorative or expressive fonts that contain
    /// decorative or expressive representations of characters.
    Fantasy,

    /// The sole criterion of a monospace font is that all glyphs have the same
    /// fixed width.
    Monospace,
}

impl From<Family> for iced::font::Family {
    fn from(value: Family) -> Self {
        match value {
            Family::Name(name) => iced::font::Family::Name(Box::leak(name.into_boxed_str())),
            Family::Serif => iced::font::Family::Serif,
            Family::SansSerif => iced::font::Family::SansSerif,
            Family::Cursive => iced::font::Family::Cursive,
            Family::Fantasy => iced::font::Family::Fantasy,
            Family::Monospace => iced::font::Family::Monospace,
        }
    }
} 

/// The weight of some text.
#[allow(missing_docs)]
#[derive(Serialize, Deserialize)]
#[serde(remote = "Weight")]
pub enum WeightDef {
    Thin,
    ExtraLight,
    Light,
    Normal,
    Medium,
    Semibold,
    Bold,
    ExtraBold,
    Black,
}

/// The width of some text.
#[allow(missing_docs)]
#[derive(Serialize, Deserialize)]
#[serde(remote = "Stretch")]
pub enum StretchDef {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

/// The style of some text.
#[allow(missing_docs)]
#[derive(Serialize, Deserialize)]
#[serde(remote = "Style")]
pub enum StyleDef {
    Normal,
    Italic,
    Oblique,
}

/// A font.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Font {
    /// The [`Family`] of the [`Font`].
    pub family: Family,
    /// The [`Weight`] of the [`Font`].
    #[serde(with = "WeightDef")]
    pub weight: Weight,
    /// The [`Stretch`] of the [`Font`].
    #[serde(with = "StretchDef")]
    pub stretch: Stretch,
    /// The [`Style`] of the [`Font`].
    #[serde(with = "StyleDef")]
    pub style: Style,
}

impl From<Font> for iced::Font {
    fn from(value: Font) -> Self {
        iced::Font {
            family: value.family.into(),
            weight: value.weight,
            stretch: value.stretch,
            style: value.style,
        }
    }
}

/// The shaping strategy of some text.
#[derive(Serialize, Deserialize)]
#[serde(remote = "Shaping")]
pub enum ShapingDef {
    /// No shaping and no font fallback.
    ///
    /// This shaping strategy is very cheap, but it will not display complex
    /// scripts properly nor try to find missing glyphs in your system fonts.
    ///
    /// You should use this strategy when you have complete control of the text
    /// and the font you are displaying in your application.
    ///
    /// This is the default.
    Basic,
    /// Advanced text shaping and font fallback.
    ///
    /// You will need to enable this flag if the text contains a complex
    /// script, the font used needs it, and/or multiple fonts in your system
    /// may be needed to display all of the glyphs.
    ///
    /// Advanced shaping is expensive! You should only enable it when necessary.
    Advanced,
}

/// The wrapping strategy of some text.
#[derive(Serialize, Deserialize)]
#[serde(remote = "Wrapping")]
pub enum WrappingDef {
    /// No wrapping.
    None,
    /// Wraps at the word level.
    ///
    /// This is the default.
    Word,
    /// Wraps at the glyph level.
    Glyph,
    /// Wraps at the word level, or fallback to glyph level if a word can't fit on a line by itself.
    WordOrGlyph,
}

/// The height of a line of text in a paragraph.
#[derive(Serialize, Deserialize)]
#[serde(remote = "LineHeight")]
pub enum LineHeightDef {
    /// A factor of the size of the text.
    Relative(f32),

    /// An absolute height in logical pixels.
    #[serde(with = "PixelsDef")]
    Absolute(Pixels),
}

/// An amount of logical pixels.
///
/// Normally used to represent an amount of space, or the size of something.
///
/// This type is normally asked as an argument in a generic way
/// (e.g. `impl Into<Pixels>`) and, since `Pixels` implements `From` both for
/// `f32` and `u16`, you should be able to provide both integers and float
/// literals as needed.
#[derive(Serialize, Deserialize)]
#[serde(remote = "Pixels")]
pub struct PixelsDef(pub f32);

pub mod opt_pixels {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(value: &Option<Pixels>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "PixelsDef")] &'a Pixels);

        value.as_ref().map(Helper).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Pixels>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper(#[serde(with = "PixelsDef")] Pixels);

        let helper = Option::deserialize(deserializer)?;
        Ok(helper.map(|Helper(external)| external))
    }
}

/// Alignment on the axis of a container.
#[derive(Serialize, Deserialize)]
#[serde(remote = "Alignment")]
pub enum AlignmentDef {
    /// Align at the start of the axis.
    Start,

    /// Align at the center of the axis.
    Center,

    /// Align at the end of the axis.
    End,
}

/// A [`Theme`] with a customized [`Palette`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Theme {
    #[serde(with = "PaletteDef")]
    pub palette: Palette,
    #[serde(with = "ExtendedDef")]
    pub extended: palette::Extended,
}

impl From<iced::Theme> for Theme {
    fn from(value: iced::Theme) -> Self {
        Self {
            palette: value.palette(),
            extended: value.extended_palette().clone()
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Palette")]
/// A color palette.
pub struct PaletteDef {
    /// The background [`Color`] of the [`Palette`].
    #[serde(with = "ColorDef")]
    pub background: Color,
    /// The text [`Color`] of the [`Palette`].
    #[serde(with = "ColorDef")]
    pub text: Color,
    /// The primary [`Color`] of the [`Palette`].
    #[serde(with = "ColorDef")]
    pub primary: Color,
    /// The success [`Color`] of the [`Palette`].
    #[serde(with = "ColorDef")]
    pub success: Color,
    /// The danger [`Color`] of the [`Palette`].
    #[serde(with = "ColorDef")]
    pub danger: Color,
}

/// An extended set of colors generated from a [`Palette`].
#[derive(Serialize, Deserialize)]
#[serde(remote = "Extended")]
pub struct ExtendedDef {
    /// The set of background colors.
    #[serde(with = "ThemeBackgroundDef")]
    pub background: palette::Background,
    /// The set of primary colors.
    #[serde(with = "PrimaryDef")]
    pub primary: Primary,
    /// The set of secondary colors.
    #[serde(with = "SecondaryDef")]
    pub secondary: Secondary,
    /// The set of success colors.
    #[serde(with = "SuccessDef")]
    pub success: Success,
    /// The set of danger colors.
    #[serde(with = "DangerDef")]
    pub danger: Danger,
    /// Whether the palette is dark or not.
    pub is_dark: bool,
}

/// A pair of background and text colors.
#[derive(Serialize, Deserialize)]
#[serde(remote = "Pair")]
pub struct PairDef {
    /// The background color.
    #[serde(with = "ColorDef")]
    pub color: Color,

    /// The text color.
    ///
    /// It's guaranteed to be readable on top of the background [`color`].
    ///
    /// [`color`]: Self::color
    #[serde(with = "ColorDef")]
    pub text: Color,
}

/// A set of background colors.
#[derive(Serialize, Deserialize)]
#[serde(remote = "palette::Background")]

pub struct ThemeBackgroundDef {
    /// The base background color.
    #[serde(with = "PairDef")]
    pub base: Pair,
    /// A weaker version of the base background color.
    #[serde(with = "PairDef")]
    pub weak: Pair,
    /// A stronger version of the base background color.
    #[serde(with = "PairDef")]
    pub strong: Pair,
}

/// A set of primary colors.
#[derive(Serialize, Deserialize)]
#[serde(remote = "Primary")]
pub struct PrimaryDef {
    /// The base primary color.
    #[serde(with = "PairDef")]
    pub base: Pair,
    /// A weaker version of the base primary color.
    #[serde(with = "PairDef")]
    pub weak: Pair,
    /// A stronger version of the base primary color.
    #[serde(with = "PairDef")]
    pub strong: Pair,
}

/// A set of secondary colors.
#[derive(Serialize, Deserialize)]
#[serde(remote = "Secondary")]
pub struct SecondaryDef {
    /// The base secondary color.
    #[serde(with = "PairDef")]
    pub base: Pair,
    /// A weaker version of the base secondary color.
    #[serde(with = "PairDef")]
    pub weak: Pair,
    /// A stronger version of the base secondary color.
    #[serde(with = "PairDef")]
    pub strong: Pair,
}

/// A set of success colors.
#[derive(Serialize, Deserialize)]
#[serde(remote = "Success")]
pub struct SuccessDef {
    /// The base success color.
    #[serde(with = "PairDef")]
    pub base: Pair,
    /// A weaker version of the base success color.
    #[serde(with = "PairDef")]
    pub weak: Pair,
    /// A stronger version of the base success color.
    #[serde(with = "PairDef")]
    pub strong: Pair,
}

/// A set of danger colors.
#[derive(Serialize, Deserialize)]
#[serde(remote = "Danger")]
pub struct DangerDef {
    /// The base danger color.
    #[serde(with = "PairDef")]
    pub base: Pair,
    /// A weaker version of the base danger color.
    #[serde(with = "PairDef")]
    pub weak: Pair,
    /// A stronger version of the base danger color.
    #[serde(with = "PairDef")]
    pub strong: Pair,
}

