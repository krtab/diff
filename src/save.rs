use std::{collections::HashMap, iter::FromIterator, marker::PhantomData, rc::Rc};

use crate::{scalar::Scalar, AsVariableUID, Diff, VariableUID};

pub trait SafeIndex {
    type ValueType;
    fn get(&self, vuid: VariableUID) -> Option<&Self::ValueType>;
    fn empty() -> Self;
}

impl<V> SafeIndex for HashMap<VariableUID, V> {
    type ValueType = V;

    fn get(&self, vuid: VariableUID) -> Option<&Self::ValueType> {
        self.get(&vuid)
    }

    fn empty() -> Self {
        HashMap::new()
    }
}

pub trait Set {
    fn contains(&self, vuid: VariableUID) -> bool;
}

impl<T> Set for Rc<T>
where
    T: SafeIndex,
{
    fn contains(&self, vuid: VariableUID) -> bool {
        self.get(vuid).is_some()
    }
}

pub struct Save0<V, S> {
    v0: V,
    saved_vuids: S,
}

impl<V, S> Save0<V, S> {
    pub fn new<D: Diff<ValueType = V>>(d: D, with_variables: S) -> Self {
        Self {
            v0: d.val(),
            saved_vuids: with_variables,
        }
    }
}

impl<V, S> Diff for Save0<V, S>
where
    V: Scalar,
    S: Set,
{
    type ValueType = V;

    type ForwardDiff = V;

    fn val(&self) -> Self::ValueType {
        self.v0
    }

    fn forward_diff<UID: crate::AsVariableUID>(&self, with_respect_to: UID) -> Self::ForwardDiff {
        if self.saved_vuids.contains(with_respect_to.as_vuid()) {
            panic!("Derivative of that order were not saved.");
        } else {
            V::zero()
        }
    }
}

pub struct Save1<V, Env> {
    v0: V,
    diffs: Rc<Env>,
}

impl<V, Env> Save1<V, Env> {
    pub fn new<D, UID>(d: D, with_variables: &[UID]) -> Self
    where
        D: Diff<ValueType = V>,
        UID: AsVariableUID,
        Env: FromIterator<(VariableUID, V)>,
    {
        Self {
            v0: d.val(),
            diffs: Rc::new(
                with_variables
                    .iter()
                    .map(|vuid| {
                        let vuid = vuid.as_vuid();
                        (vuid, d.forward_diff(vuid).val())
                    })
                    .collect(),
            ),
        }
    }

    pub fn constant(v: V) -> Self
    where
        V: Scalar,
        Env: FromIterator<(VariableUID, V)>,
    {
        Self::new::<_,VariableUID>(v, &[])
    }
}

impl<V, Env> Default for Save1<V, Env>
where
    V: Scalar,
    Env: SafeIndex,
{
    fn default() -> Self {
        Self {
            diffs: Rc::new(Env::empty()),
            v0: V::zero(),
        }
    }
}

impl<V, Env> Clone for Save1<V, Env>
where
    V: Scalar,
{
    fn clone(&self) -> Self {
        Self {
            v0: self.v0,
            diffs: self.diffs.clone(),
        }
    }
}

impl<V, Env> Diff for Save1<V, Env>
where
    V: Scalar,
    Env: SafeIndex<ValueType = V>,
{
    type ValueType = V;

    type ForwardDiff = Save0<V, Rc<Env>>;

    fn val(&self) -> Self::ValueType {
        self.v0
    }

    fn forward_diff<UID: crate::AsVariableUID>(&self, with_respect_to: UID) -> Self::ForwardDiff {
        Save0 {
            v0: self
                .diffs
                .get(with_respect_to.as_vuid())
                .map(|&x| x)
                .unwrap_or(V::zero()),
            saved_vuids: self.diffs.clone(),
        }
    }
}

pub struct Save2<V, Env, SubEnv> {
    v0: V,
    diffs: Env,
    _phantom: PhantomData<SubEnv>,
}

impl<V, Env, SubEnv> Save2<V, Env, SubEnv> {
    pub fn new<D, UID>(d: D, with_variables: &[UID]) -> Self
    where
        D: Diff<ValueType = V>,
        UID: AsVariableUID,
        Env: FromIterator<(VariableUID, Save1<V, SubEnv>)>,
        SubEnv: FromIterator<(VariableUID, V)>,
    {
        Self {
            v0: d.val(),
            diffs: with_variables
                .iter()
                .map(|vuid| {
                    let vuid = vuid.as_vuid();
                    (vuid, Save1::new(d.forward_diff(vuid), with_variables))
                })
                .collect(),
            _phantom: PhantomData,
        }
    }
}

impl<V, Env, SubEnv> Diff for Save2<V, Env, SubEnv>
where
    V: Scalar,
    Env: SafeIndex<ValueType = Save1<V, Rc<SubEnv>>>,
    Rc<SubEnv>: SafeIndex<ValueType = V> + Copy,
    SubEnv: Default,
    Env: Copy,
{
    type ValueType = V;

    type ForwardDiff = Save1<V, Rc<SubEnv>>;

    fn val(&self) -> Self::ValueType {
        self.v0
    }

    fn forward_diff<UID: crate::AsVariableUID>(&self, with_respect_to: UID) -> Self::ForwardDiff {
        self.diffs
            .get(with_respect_to.as_vuid())
            .cloned()
            .unwrap_or_default()
    }
}
