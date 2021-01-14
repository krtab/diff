use std::marker::PhantomData;

use crate::{scalar::Scalar, AsVariableUID, Diff, Expr};

pub struct Addition<L, R, V> {
    left: L,
    right: R,
    _value: PhantomData<V>,
}

impl<L, R, V> Addition<L, R, V> {
    pub fn new(left: L, right: R) -> Self {
        Addition {
            _value: PhantomData,
            left,
            right,
        }
    }
}

impl<L: Clone, R: Clone, V> Clone for Addition<L, R, V> {
    fn clone(&self) -> Self {
        Self {
            left: self.left.clone(),
            right: self.right.clone(),
            _value: PhantomData,
        }
    }
}

impl<L, R, V: Scalar> Expr for Addition<L, R, V> {
    type ValueType = V;
}

impl<L, R, V> Diff for Addition<L, R, V>
where
    V: Scalar,
    L: Diff<ValueType = V>,
    R: Diff<ValueType = V>,
{
    type ForwardDiff = Addition<L::ForwardDiff, R::ForwardDiff, V>;

    fn val(&self) -> Self::ValueType {
        self.left.val() + self.right.val()
    }

    fn forward_diff<UID: AsVariableUID>(&self, with_respect_to: UID) -> Self::ForwardDiff {
        self.left
            .forward_diff(with_respect_to.as_vuid())
            .add_diff(self.right.forward_diff(with_respect_to.as_vuid()))
    }
}

pub struct Multiplication<L, R, V> {
    left: L,
    right: R,
    _value: PhantomData<V>,
}

impl<L, R, V> Multiplication<L, R, V>
where
    V: Scalar,
{
    pub fn new(left: L, right: R) -> Self {
        Multiplication {
            _value: PhantomData,
            left,
            right,
        }
    }
}

impl<L: Clone, R: Clone, V> Clone for Multiplication<L, R, V> {
    fn clone(&self) -> Self {
        Self {
            left: self.left.clone(),
            right: self.right.clone(),
            _value: PhantomData,
        }
    }
}

impl<L, R, V: Scalar> Expr for Multiplication<L, R, V> {
    type ValueType = V;
}

impl<L, R, V> Diff for Multiplication<L, R, V>
where
    V: Scalar,
    L: Diff<ValueType = V> + Clone,
    R: Diff<ValueType = V> + Clone,
{
    type ForwardDiff =
        Addition<Multiplication<L, R::ForwardDiff, V>, Multiplication<R, L::ForwardDiff, V>, V>;

    fn val(&self) -> Self::ValueType {
        self.left.val() * self.right.val()
    }

    fn forward_diff<UID: AsVariableUID>(&self, with_respect_to: UID) -> Self::ForwardDiff {
        let lhs = self
            .left
            .clone()
            .mul_diff(self.right.forward_diff(with_respect_to.as_vuid()));
        let rhs = self
            .right
            .clone()
            .mul_diff(self.left.forward_diff(with_respect_to.as_vuid()));
        lhs.add_diff(rhs)
    }
}
