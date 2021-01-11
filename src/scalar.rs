use super::{AsVariableUID, Diff};
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

            fn forward_diff<UID : AsVariableUID>(&self, _with_respect_to: UID) -> Self::ForwardDiff {
                <$f>::zero()
            }
        }

        impl_diff_for_scalar!($($fs),*);
    };
}

impl_diff_for_scalar!(f32, f64);
