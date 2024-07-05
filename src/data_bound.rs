use std::ops::Range;

pub trait Extendable {
    fn extend(self, bound: DataBound) -> DataBound;
}

impl Extendable for f32 {
    fn extend(self, bound: DataBound) -> DataBound {
        DataBound(self.min(bound.0), self.max(bound.1))
    }
}

impl Extendable for DataBound {
    fn extend(self, bound: DataBound) -> DataBound {
        DataBound(self.0.min(bound.0), self.1.max(bound.1))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DataBound(f32, f32);

impl From<f32> for DataBound {
    fn from(value: f32) -> Self {
        Self(value, value)
    }
}

impl From<(f32, f32)> for DataBound {
    fn from((a, b): (f32, f32)) -> Self {
        Self(a, b)
    }
}

impl DataBound {
    pub fn zero() -> Self {
        Self(0.0, 0.0)
    }

    pub fn max(self, other: Self) -> Self {
        Self(self.0.max(other.0), self.1.max(other.1))
    }

    pub fn extend_to_include(self, other: impl Extendable) -> Self {
        other.extend(self)
    }

    pub fn as_tuple(self) -> (f32, f32) {
        (self.0, self.1)
    }

    pub fn as_range(self) -> Range<f32> {
        self.0..self.1
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DataBound2D {
    pub x: DataBound,
    pub y: DataBound,
}

impl From<(f32, f32)> for DataBound2D {
    fn from((x, y): (f32, f32)) -> Self {
        Self {
            x: DataBound::from(x),
            y: DataBound::from(y),
        }
    }
}

impl From<(DataBound, DataBound)> for DataBound2D {
    fn from((x, y): (DataBound, DataBound)) -> Self {
        Self { x, y }
    }
}

impl DataBound2D {
    pub fn zero() -> Self {
        Self {
            x: DataBound::zero(),
            y: DataBound::zero(),
        }
    }

    pub fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    pub fn extend_to_include_bound(self, x: impl Into<DataBound>, y: impl Into<DataBound>) -> Self {
        Self {
            x: self.x.extend_to_include(x.into()),
            y: self.y.extend_to_include(y.into()),
        }
    }

    pub fn extend_to_include_bound2d(self, other: impl Into<DataBound2D>) -> Self {
        let other = other.into();
        Self {
            x: self.x.extend_to_include(other.x),
            y: self.y.extend_to_include(other.y),
        }
    }
}
