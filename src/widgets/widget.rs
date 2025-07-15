use std::fmt::Debug;

use iced::{Length, Size};

#[typetag::serde(tag = "type")]
pub trait Widget: Debug + Sync + Send {
    fn size_hint(&self) -> Size<Length>;

    fn data(&self) -> Vec<u8>;
}
