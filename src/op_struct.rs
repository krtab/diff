use crate::{scalar::Scalar, AsVariableUID, Diff};

pub struct Addition<L, R, V>
where
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
    V: Scalar,
    L: Diff<ValueType = V>,
    R: Diff<ValueType = V>,
    L::ForwardDiff: Diff<ValueType=V>,
    R::ForwardDiff: Diff<ValueType=V>,
{
    type ValueType = L::ValueType;

    type ForwardDiff = Addition<L::ForwardDiff, R::ForwardDiff, V>;

    fn val(&self) -> Self::ValueType {
        self.value
    }

    fn forward_diff<UID: AsVariableUID>(&self, with_respect_to: UID) -> Self::ForwardDiff {
        self.left
            .forward_diff(with_respect_to.as_vuid())
            .add_diff(self.right.forward_diff(with_respect_to.as_vuid()))
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
    V: Scalar
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
    V: Scalar,
    V: Diff<ValueType = V>,
    L: Diff<ValueType = V>,
    R: Diff<ValueType = V>,
    L::ForwardDiff: Diff< ValueType = V >,
    R::ForwardDiff: Diff< ValueType = V >,
    <L::ForwardDiff as Diff>::ForwardDiff : Diff<ValueType = V >,
    <R::ForwardDiff as Diff>::ForwardDiff : Diff<ValueType = V >,
    Multiplication<L,R::ForwardDiff, V> : Diff<ValueType = V>,
    Multiplication<L::ForwardDiff,R, V> : Diff<ValueType = V>,

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
            .mul_diff(self.right.forward_diff(with_respect_to.as_vuid()));
        let rhs = self
            .right
            .mul_diff(self.left.forward_diff(with_respect_to.as_vuid()));
        lhs.add_diff(rhs)
    }
}
