use crate::{Diff, Variable, op_struct::{Addition, Multiplication}, scalar::Scalar};

pub enum DynDiff<V: Scalar> {
    Value(V),
    Variable(Variable<V>),
    Addition(Box<Addition<Self,Self,V>>),
    Multiplication(Box<Multiplication<Self,Self,V>>),
}

impl<V> Diff for DynDiff<V> 
where
    V: Scalar,
{
    type ValueType = V;

    type ForwardDiff = Self;

    fn val(&self) -> Self::ValueType {
        match self {
            DynDiff::Value(x) => x.val(),
            DynDiff::Variable(x) => x.val(), 
            DynDiff::Addition(x) => x.val(),
            DynDiff::Multiplication(x) => x.val(),
        }
    }

    fn forward_diff<UID: crate::AsVariableUID>(&self, with_respect_to: UID) -> Self::ForwardDiff {
        match &self {
            DynDiff::Value(x) => x.forward_diff(with_respect_to).to_dyndiff(),
            DynDiff::Variable(x) => x.forward_diff(with_respect_to).to_dyndiff(),
            DynDiff::Addition(x) => x.forward_diff(with_respect_to).to_dyndiff(),
            DynDiff::Multiplication(x) => x.forward_diff(with_respect_to).to_dyndiff(),
        }
    }

    fn to_dyndiff(&self) -> DynDiff<Self::ValueType> {
        match self {
            DynDiff::Value(v) => v.to_dyndiff(),
            DynDiff::Variable(v) => v.to_dyndiff(),
            DynDiff::Addition(v) => v.to_dyndiff(),
            DynDiff::Multiplication(v) => v.to_dyndiff(),
        }
    }
}