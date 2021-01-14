use crate::{AsVariableUID, Diff, Expr};
use num_traits::{real::Real, Zero};

pub trait Scalar: Diff<ValueType = Self, ForwardDiff = Self> + Real {}

impl<T> Scalar for T
where
    T: Diff<ValueType = T, ForwardDiff = T>,
    T: Real,
{
}

macro_rules! impl_diff_for_scalar {
    () => {};
    ($f:ty $(, $fs:ty)*) => {
        impl Expr for $f {
            type ValueType = $f;
        }
        impl Diff for $f {
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
