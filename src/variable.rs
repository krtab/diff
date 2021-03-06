use std::sync::atomic::{AtomicU64, Ordering};

use crate::{scalar::Scalar, AsVariableUID};

use super::{Diff, Expr, VariableUID, GLOBAL_CONTEXT};

pub(crate) struct Context {
    n_vars: AtomicU64,
}

impl Context {
    pub(crate) const fn new() -> Self {
        Context {
            n_vars: AtomicU64::new(0),
        }
    }

    pub(crate) fn variable<V>(&self, value: V) -> Variable<V> {
        let vuid = self.n_vars.fetch_add(1, Ordering::Relaxed);
        Variable {
            vuid: VariableUID(vuid),
            value,
        }
    }
}

pub struct Variable<ValueType> {
    pub(crate) vuid: VariableUID,
    pub(crate) value: ValueType,
}

impl<V> Variable<V> {
    pub fn new(value: V) -> Self {
        GLOBAL_CONTEXT.variable(value)
    }
    pub fn vuid(&self) -> VariableUID {
        self.vuid
    }
}
impl<'a, V> AsVariableUID for &'a Variable<V> {
    fn as_vuid(&self) -> VariableUID {
        self.vuid()
    }
}

impl<V> AsVariableUID for Variable<V> {
    fn as_vuid(&self) -> VariableUID {
        self.vuid()
    }
}

impl<'a, F: Scalar> Expr for &'a Variable<F> {
    type ValueType = F;
}

impl<'a, F> Diff for &'a Variable<F>
where
    F: Scalar,
{
    type ForwardDiff = F;

    fn val(&self) -> Self::ValueType {
        self.value
    }

    fn forward_diff<UID: AsVariableUID>(&self, with_respect_to: UID) -> Self::ForwardDiff {
        if with_respect_to.as_vuid() == self.vuid {
            F::one()
        } else {
            F::zero()
        }
    }
}
