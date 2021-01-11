use std::sync::atomic::{AtomicU64, Ordering};

use super::{Diff, VariableUID, GLOBAL_CONTEXT};

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
        Variable { vuid, value }
    }
}

pub struct Variable<ValueType> {
    vuid: VariableUID,
    value: ValueType,
}

impl<V> Variable<V> {
    pub fn new(value: V) -> Self {
        GLOBAL_CONTEXT.variable(value)
    }
}

impl<F> Diff for Variable<F>
where
    F: Copy,
    F: Diff<ValueType = F>,
    F: num_traits::One,
    F: num_traits::Zero,
{
    type ValueType = F;
    type ForwardDiff = F;

    fn val(&self) -> Self::ValueType {
        self.value
    }

    fn forward_diff(&self, with_respect_to: VariableUID) -> Self::ForwardDiff {
        if with_respect_to == self.vuid {
            F::one()
        } else {
            F::zero()
        }
    }
}
