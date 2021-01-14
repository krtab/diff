pub mod non_op_functions_struct;
pub mod op_struct;
pub mod save;
pub mod scalar;
mod std_ops;
pub mod variable;
pub mod vector;

use std::borrow::Borrow;

use non_op_functions_struct::Exp;
use op_struct::*;
use scalar::Scalar;
use variable::Context;
pub use variable::Variable;

static GLOBAL_CONTEXT: Context = Context::new();

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct VariableUID(u64);

pub trait AsVariableUID {
    fn as_vuid(&self) -> VariableUID;
}

impl<T: Borrow<VariableUID>> AsVariableUID for T {
    fn as_vuid(&self) -> VariableUID {
        *self.borrow()
    }
}

pub trait Diff: Expr {
    type ForwardDiff: Expr<ValueType = Self::ValueType>;

    fn val(&self) -> Self::ValueType;

    fn forward_diff<UID: AsVariableUID>(&self, with_respect_to: UID) -> Self::ForwardDiff;
}

pub trait Expr: Sized {
    type ValueType: Scalar;

    fn add_diff<R>(self, rhs: R) -> Addition<Self, R, Self::ValueType> {
        Addition::new(self, rhs)
    }

    fn mul_diff<R>(self, rhs: R) -> Multiplication<Self, R, Self::ValueType> {
        Multiplication::new(self, rhs)
    }

    fn exp_diff(self) -> Exp<Self, Self::ValueType> {
        Exp::new(self)
    }
}

impl<'a, T: Expr> Expr for &'a T {
    type ValueType = T::ValueType;
}

impl<'a, T: Diff> Diff for &'a T {
    type ForwardDiff = T::ForwardDiff;

    fn val(&self) -> Self::ValueType {
        (*self).val()
    }

    fn forward_diff<UID: AsVariableUID>(&self, with_respect_to: UID) -> Self::ForwardDiff {
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
        let yid = y.vuid();
        let res = x.add_diff(&y).add_diff(100.);
        let dx = res.forward_diff(&x);
        let dy = res.forward_diff(yid);
        let dxdy = dx.forward_diff(yid);
        assert_eq!(res.val(), 111.);
        assert_eq!(dx.val(), 1.);
        assert_eq!(dy.val(), 1.);
        assert_eq!(dxdy.val(), 0.)
    }
}
