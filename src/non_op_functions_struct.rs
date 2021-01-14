use std::marker::PhantomData;

use crate::{op_struct::Multiplication, scalar::Scalar, AsVariableUID, Diff, Expr};

pub struct Exp<D, V> {
    arg: D,
    _value: PhantomData<V>,
}

impl<D, V> Exp<D, V> {
    pub fn new(arg: D) -> Self {
        Exp {
            arg,
            _value: PhantomData,
        }
    }
}

impl<D: Clone, V> Clone for Exp<D, V> {
    fn clone(&self) -> Self {
        Self {
            arg: self.arg.clone(),
            _value: PhantomData,
        }
    }
}

impl<D, V: Scalar> Expr for Exp<D, V> {
    type ValueType = V;
}

impl<D, V> Diff for Exp<D, V>
where
    V: Scalar,
    D: Diff<ValueType = V>,
    D::ForwardDiff: Copy,
    D::ForwardDiff: Diff<ValueType = V>,
    Self: Clone,
{
    type ForwardDiff = Multiplication<D::ForwardDiff, Self, V>;

    fn val(&self) -> Self::ValueType {
        self.arg.val().exp()
    }

    fn forward_diff<UID: AsVariableUID>(&self, with_respect_to: UID) -> Self::ForwardDiff {
        self.arg
            .forward_diff(with_respect_to.as_vuid())
            .mul_diff(self.clone())
    }
}
