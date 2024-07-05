pub trait IntoData {
    fn into_data(self) -> Vec<(f32, f32)>;
}

pub fn data_from_iterator<I: IntoDataIterItem, IT: Iterator<Item = I>>(it: IT) -> Vec<(f32, f32)> {
    it.enumerate()
        .map(|(i, item)| item.into_data_item(i))
        .collect()
}

impl IntoData for Vec<f32> {
    fn into_data(self) -> Vec<(f32, f32)> {
        data_from_iterator(self.into_iter())
    }
}

impl IntoData for Vec<(f32, f32)> {
    fn into_data(self) -> Vec<(f32, f32)> {
        self
    }
}

impl<const N: usize> IntoData for [f32; N] {
    fn into_data(self) -> Vec<(f32, f32)> {
        data_from_iterator(self.into_iter())
    }
}

impl<const N: usize> IntoData for [(f32, f32); N] {
    fn into_data(self) -> Vec<(f32, f32)> {
        self.to_vec()
    }
}

impl<const N: usize> IntoData for &[f32; N] {
    fn into_data(self) -> Vec<(f32, f32)> {
        data_from_iterator(self.into_iter())
    }
}

impl<const N: usize> IntoData for &[(f32, f32); N] {
    fn into_data(self) -> Vec<(f32, f32)> {
        self.to_vec()
    }
}

impl IntoData for &[f32] {
    fn into_data(self) -> Vec<(f32, f32)> {
        data_from_iterator(self.into_iter())
    }
}

impl IntoData for &[(f32, f32)] {
    fn into_data(self) -> Vec<(f32, f32)> {
        self.to_vec()
    }
}

macro_rules! impl_data_for_primitive {
    ($ty:ty) => {
        impl IntoData for Vec<$ty> {
            fn into_data(self) -> Vec<(f32, f32)> {
                data_from_iterator(self.into_iter())
            }
        }

        impl IntoData for Vec<($ty, $ty)> {
            fn into_data(self) -> Vec<(f32, f32)> {
                data_from_iterator(self.into_iter())
            }
        }

        impl<const N: usize> IntoData for [$ty; N] {
            fn into_data(self) -> Vec<(f32, f32)> {
                data_from_iterator(self.into_iter())
            }
        }

        impl<const N: usize> IntoData for [($ty, $ty); N] {
            fn into_data(self) -> Vec<(f32, f32)> {
                data_from_iterator(self.into_iter())
            }
        }

        impl<const N: usize> IntoData for &[$ty; N] {
            fn into_data(self) -> Vec<(f32, f32)> {
                data_from_iterator(self.into_iter())
            }
        }

        impl<const N: usize> IntoData for &[($ty, $ty); N] {
            fn into_data(self) -> Vec<(f32, f32)> {
                data_from_iterator(self.into_iter())
            }
        }

        impl IntoData for &[$ty] {
            fn into_data(self) -> Vec<(f32, f32)> {
                data_from_iterator(self.into_iter())
            }
        }

        impl IntoData for &[($ty, $ty)] {
            fn into_data(self) -> Vec<(f32, f32)> {
                data_from_iterator(self.into_iter())
            }
        }
    };
}

// impl_data_for_primitive!(f32, Byf32); // defined more simply above
impl_data_for_primitive!(f64);
impl_data_for_primitive!(i8);
impl_data_for_primitive!(i32);
impl_data_for_primitive!(i64);
impl_data_for_primitive!(u8);
impl_data_for_primitive!(u32);
impl_data_for_primitive!(u64);

pub trait IntoDataIterItem {
    fn into_data_item(self, idx: usize) -> (f32, f32);
}

macro_rules! impl_item_for_primitive {
    ($ty:ty) => {
        impl IntoDataIterItem for &$ty {
            fn into_data_item(self, idx: usize) -> (f32, f32) {
                (idx as _, *self as _)
            }
        }

        impl IntoDataIterItem for $ty {
            fn into_data_item(self, idx: usize) -> (f32, f32) {
                (idx as _, self as _)
            }
        }

        impl IntoDataIterItem for &($ty, $ty) {
            fn into_data_item(self, _idx: usize) -> (f32, f32) {
                (self.0 as _, self.1 as _)
            }
        }

        impl IntoDataIterItem for ($ty, $ty) {
            fn into_data_item(self, _idx: usize) -> (f32, f32) {
                (self.0 as _, self.1 as _)
            }
        }
    };
}

impl_item_for_primitive!(f32);
impl_item_for_primitive!(f64);
impl_item_for_primitive!(i8);
impl_item_for_primitive!(i32);
impl_item_for_primitive!(i64);
impl_item_for_primitive!(u8);
impl_item_for_primitive!(u32);
impl_item_for_primitive!(u64);
