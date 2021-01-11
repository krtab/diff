mod op_struct;
mod scalar;
mod variable;

use op_struct::*;
use variable::Context;
pub use variable::Variable;

static GLOBAL_CONTEXT: Context = Context::new();
type VariableUID = u64;

pub trait Diff: Sized {
    type ValueType: Copy;

    type ForwardDiff: Diff<ValueType = Self::ValueType>;
    fn val(&self) -> Self::ValueType;

    fn forward_diff(&self, with_respect_to: VariableUID) -> Self::ForwardDiff;

    fn add_diff<R>(self, rhs: R) -> Addition<Self, R, Self::ValueType>
    where
        R: Diff<ValueType = Self::ValueType>,
        Self::ValueType: std::ops::Add<Self::ValueType, Output = Self::ValueType>,
    {
        Addition::new(self, rhs)
    }

    fn mul_diff<R>(self, rhs: R) -> Multiplication<Self, R, Self::ValueType>
    where
        R: Diff<ValueType = Self::ValueType>,
        Self::ValueType: std::ops::Mul<Self::ValueType, Output = Self::ValueType>,
    {
        Multiplication::new(self, rhs)
    }
}

impl<'a, T: Diff> Diff for &'a T {
    type ValueType = T::ValueType;
    type ForwardDiff = T::ForwardDiff;

    fn val(&self) -> Self::ValueType {
        (*self).val()
    }

    fn forward_diff(&self, with_respect_to: VariableUID) -> Self::ForwardDiff {
        (*self).forward_diff(with_respect_to)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn addition_fwd() {
        let x = Variable::new(1.);
        let y = Variable::new(10.);
        let res = x.add_diff(y).add_diff(100.);
        let dx = res.forward_diff(0);
        let dy = res.forward_diff(1);
        let dxdy = dx.forward_diff(1);
        assert_eq!(res.val(), 111.);
        assert_eq!(dx.val(), 1.);
        assert_eq!(dy.val(), 1.);
        assert_eq!(dxdy.val(), 0.)
    }
}
