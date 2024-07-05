use rgb::RGBA;

#[derive(Debug, Clone, Copy)]
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
