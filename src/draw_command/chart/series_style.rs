use rgb::RGBA;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SeriesStyle {
    pub color: RGBA<f32>,
}

impl std::default::Default for SeriesStyle {
    fn default() -> Self {
        Self {
            color: Default::default(),
        }
    }
}
