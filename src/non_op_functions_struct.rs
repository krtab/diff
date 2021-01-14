use crate::{AsVariableUID, Diff, op_struct::Multiplication, scalar::Scalar};

pub struct Exp<D, V>
where
    D: Diff<ValueType = V>,
{
    arg: D,
    value: V,
}

impl<D, V> Exp<D, V>
where
    D: Diff<ValueType = V>,
    V: Scalar

{
    pub fn new(arg: D) -> Self {
        Exp {
            value : arg.val().exp(),
            arg
        }
    }
}

impl<D, V> Diff for Exp<D, V>
where
    V: Scalar,
    D: Diff<ValueType = V>,
    D::ForwardDiff: Copy,
    D::ForwardDiff: Diff<ValueType = V>,
    Self: Copy,
{
    type ValueType = V;

    type ForwardDiff = Multiplication<D::ForwardDiff,Self,V>;

    fn val(&self) -> Self::ValueType {
        self.value
    }

    fn forward_diff<UID: AsVariableUID>(&self, with_respect_to: UID) -> Self::ForwardDiff {
        self.arg.forward_diff(with_respect_to.as_vuid()).mul_diff(*self)
    }
}
