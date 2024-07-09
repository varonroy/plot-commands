use super::{Image, ImageStyle};

pub struct ImageBuilder(Image);

impl From<Image> for ImageBuilder {
    fn from(value: Image) -> Self {
        Self(value)
    }
}

impl ImageBuilder {
    pub fn style(mut self, style: impl Into<ImageStyle>) -> Self {
        self.0.style = style.into();
        self
    }

    pub fn axes(mut self) -> Self {
        self.0.style.draw_axes = true;
        self
    }
    pub fn build(self) -> Image {
        self.0
    }
}
