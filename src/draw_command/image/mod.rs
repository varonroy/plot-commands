use std::io::Cursor;

use float_ord::FloatOrd;
use image::{DynamicImage, ImageBuffer};

#[cfg(feature = "builder")]
pub mod builder;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ImageFormat {
    /// No compression, raw pixel data
    Buffer,
    Jpeg,
    Png,
}

impl From<image::ImageFormat> for ImageFormat {
    fn from(value: image::ImageFormat) -> Self {
        match value {
            image::ImageFormat::Jpeg => Self::Jpeg,
            image::ImageFormat::Png => Self::Png,
            _ => panic!("unsupported {:?}", value),
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImageStyle {
    pub draw_axes: bool,
}

impl std::default::Default for ImageStyle {
    fn default() -> Self {
        Self { draw_axes: false }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Image {
    pub title: String,
    pub buffer: Vec<u8>,
    pub format: ImageFormat,
    pub rows: usize,
    pub cols: usize,
    pub style: ImageStyle,
}

// #[cfg(feature = "builder")]
// pub use conversions::*;

// #[cfg(feature = "builder")]
// mod conversions {
//     use super::builder::ImageBuilder;
//     use super::Image;
//
//     impl From<ImageBuilder> for Image {
//         fn from(value: ImageBuilder) -> Self {
//             value.build()
//         }
//     }
// }

impl From<DynamicImage> for Image {
    fn from(value: DynamicImage) -> Self {
        let mut buffer: Vec<u8> = Vec::new();
        let format = image::ImageFormat::Png;
        value
            .write_to(&mut Cursor::new(&mut buffer), format)
            .unwrap();

        Self {
            title: "".to_string(),
            buffer,
            format: format.into(),
            rows: value.height() as _,
            cols: value.width() as _,
            style: Default::default(),
        }
    }
}

impl<P, C> From<ImageBuffer<P, C>> for Image
where
    P: image::Pixel + image::PixelWithColorType,
    [P::Subpixel]: image::EncodableLayout,
    C: std::ops::Deref<Target = [P::Subpixel]>,
    DynamicImage: From<ImageBuffer<P, C>>,
{
    fn from(value: ImageBuffer<P, C>) -> Self {
        let img = image::DynamicImage::from(value).to_rgb8();

        let mut buffer: Vec<u8> = Vec::new();
        let format = image::ImageFormat::Png;
        img.write_to(&mut Cursor::new(&mut buffer), format).unwrap();

        Self {
            title: "".to_string(),
            buffer,
            format: format.into(),
            rows: img.height() as _,
            cols: img.width() as _,
            style: Default::default(),
        }
    }
}

pub trait PixelCoreType: Clone + Copy + Send + Sync + Sized + PartialEq {
    type OrdType: Ord;

    fn normalize(self, min: Self, max: Self) -> u8;

    fn into_ord(self) -> Self::OrdType;
}

impl PixelCoreType for f32 {
    type OrdType = FloatOrd<Self>;

    fn normalize(self, min: Self, max: Self) -> u8 {
        ((self - min) / (max - min) * 255.0) as u8
    }

    fn into_ord(self) -> Self::OrdType {
        FloatOrd(self)
    }
}

impl PixelCoreType for f64 {
    type OrdType = FloatOrd<Self>;

    fn normalize(self, min: Self, max: Self) -> u8 {
        ((self - min) / (max - min) * 255.0) as u8
    }

    fn into_ord(self) -> Self::OrdType {
        FloatOrd(self)
    }
}

impl PixelCoreType for u8 {
    type OrdType = Self;

    fn normalize(self, _min: Self, _max: Self) -> u8 {
        self
    }

    fn into_ord(self) -> Self::OrdType {
        self
    }
}

impl PixelCoreType for i32 {
    type OrdType = Self;

    fn normalize(self, min: Self, max: Self) -> u8 {
        let x = (self - min) as f64;
        let max = (max - min) as f64;
        x.normalize(0.0, max)
    }

    fn into_ord(self) -> Self::OrdType {
        self
    }
}

impl PixelCoreType for i64 {
    type OrdType = Self;

    fn normalize(self, min: Self, max: Self) -> u8 {
        let x = (self - min) as f64;
        let max = (max - min) as f64;
        x.normalize(0.0, max)
    }

    fn into_ord(self) -> Self::OrdType {
        self
    }
}

pub trait IntoPixel: Clone + Copy + Send + Sync + Sized {
    type CoreType: PixelCoreType;

    fn iter(&self) -> impl Iterator<Item = Self::CoreType>;

    fn into_pixel(self, min: Self::CoreType, max: Self::CoreType) -> [u8; 3] {
        let mut it = self.iter();
        let first = it.next().unwrap().normalize(min, max);
        match it.next() {
            None => [first, first, first],
            Some(second) => {
                let third = it.next().unwrap();
                [first, second.normalize(min, max), third.normalize(min, max)]
            }
        }
    }
}

impl<T: PixelCoreType> IntoPixel for T {
    type CoreType = T;

    fn iter(&self) -> impl Iterator<Item = Self::CoreType> {
        std::iter::once(*self)
    }
}

impl<T: PixelCoreType> IntoPixel for [T; 3] {
    type CoreType = T;

    fn iter(&self) -> impl Iterator<Item = Self::CoreType> {
        self.into_iter().copied()
    }
}

impl<T: PixelCoreType> IntoPixel for (T, T, T) {
    type CoreType = T;

    fn iter(&self) -> impl Iterator<Item = Self::CoreType> {
        [self.0, self.1, self.2].into_iter()
    }
}

impl<P: IntoPixel> From<Vec<Vec<P>>> for Image {
    fn from(value: Vec<Vec<P>>) -> Self {
        let min = value
            .iter()
            .flatten()
            .flat_map(|p| p.iter())
            .min_by_key(|x| x.into_ord())
            .unwrap();
        let max = value
            .iter()
            .flatten()
            .flat_map(|p| p.iter())
            .max_by_key(|x| x.into_ord())
            .unwrap();
        let min = min;
        let max = max;

        let rows = value.len();
        let cols = value.iter().map(|row| row.len()).max().unwrap();

        let mut imgbuf = image::ImageBuffer::new(cols as _, rows as _);

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let col = x as usize;
            let row = y as usize;
            let color = value
                .get(row)
                .and_then(|row| row.get(col))
                .map(|x| x.into_pixel(min, max))
                .unwrap_or([0, 0, 0]);
            *pixel = image::Rgb(color);
        }

        let image: DynamicImage = imgbuf.into();

        Self::from(image)
    }
}

impl Image {
    pub fn to_image_dynamic_image(&self) -> image::DynamicImage {
        image::io::Reader::new(Cursor::new(&self.buffer))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap()
    }
}
