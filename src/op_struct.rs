use crate::AsVariableUID;

use super::Diff;

pub struct Addition<L, R, V>
where
    L: Diff<ValueType = V>,
    R: Diff<ValueType = V>,
{
    left: L,
    right: R,
    value: V,
}

impl<L, R, V> Addition<L, R, V>
where
    L: Diff<ValueType = V>,
    R: Diff<ValueType = V>,

    V: std::ops::Add<V, Output = V>,
{
    pub fn new(left: L, right: R) -> Self {
        Addition {
            value: left.val() + right.val(),
            left,
            right,
        }
    }
}

impl<L, R, V> Diff for Addition<L, R, V>
where
    V: Copy,
    L: Diff<ValueType = V>,
    R: Diff<ValueType = V>,
    L::ValueType: std::ops::Add<R::ValueType, Output = L::ValueType>,
{
    type ValueType = L::ValueType;

    type ForwardDiff = Addition<L::ForwardDiff, R::ForwardDiff, V>;

    fn val(&self) -> Self::ValueType {
        self.value
    }

    fn forward_diff<UID: AsVariableUID>(&self, with_respect_to: UID) -> Self::ForwardDiff {
        self.left
            .forward_diff(with_respect_to)
            .add_diff(self.right.forward_diff(with_respect_to))
    }
}

pub struct Multiplication<L, R, V>
where
    L: Diff<ValueType = V>,
    R: Diff<ValueType = V>,
{
    left: L,
    right: R,
    value: V,
}

impl<L, R, V> Multiplication<L, R, V>
where
    L: Diff<ValueType = V>,
    R: Diff<ValueType = V>,
    V: std::ops::Mul<V, Output = V>,
{
    pub fn new(left: L, right: R) -> Self {
        Multiplication {
            value: left.val() * right.val(),
            left,
            right,
        }
    }
}

impl<L, R, V> Diff for Multiplication<L, R, V>
where
    V: Copy,
    V: Diff<ValueType = V>,
    L: Diff<ValueType = V>,
    R: Diff<ValueType = V>,
    V: std::ops::Add<V, Output = V>,
    V: std::ops::Mul<V, Output = V>,
{
    type ValueType = L::ValueType;

    type ForwardDiff =
        Addition<Multiplication<V, R::ForwardDiff, V>, Multiplication<V, L::ForwardDiff, V>, V>;

    fn val(&self) -> Self::ValueType {
        self.value
    }

    fn forward_diff<UID: AsVariableUID>(&self, with_respect_to: UID) -> Self::ForwardDiff {
        let lhs = self
            .left
            .val()
            .mul_diff(self.right.forward_diff(with_respect_to));
        let rhs = self
            .right
            .val()
            .mul_diff(self.left.forward_diff(with_respect_to));
        lhs.add_diff(rhs)
    }
}
