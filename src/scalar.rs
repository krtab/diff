use super::{Diff, VariableUID};
use num_traits::Zero;

macro_rules! impl_diff_for_scalar {
    () => {};
    ($f:ty $(, $fs:ty)*) => {
        impl Diff for $f {
            type ValueType = Self;

            type ForwardDiff = Self;

            fn val(&self) -> Self::ValueType {
                *self
            }

            fn forward_diff(&self, _with_respect_to: VariableUID) -> Self::ForwardDiff {
                <$f>::zero()
            }
        }

        impl_diff_for_scalar!($($fs),*);
    };
}

impl_diff_for_scalar!(f32, f64);
