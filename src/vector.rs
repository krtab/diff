use std::marker::PhantomData;

use crate::{scalar::Scalar, Diff, VariableUID};

pub trait DiffIter {
    type ValueType: Scalar;
    type DiffType: Diff<ValueType = Self::ValueType>;
}

pub struct ForwardDiffIter<I, D> {
    inner: I,
    wrt: VariableUID,
    _phantom: PhantomData<D>,
}

impl<I, D> Clone for ForwardDiffIter<I, D>
where
    I: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            wrt: self.wrt,
            _phantom: PhantomData,
        }
    }
}

impl<D, I> Iterator for ForwardDiffIter<I, D>
where
    D: Diff,
    I: Iterator<Item = D>,
{
    type Item = D::ForwardDiff;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|d| d.forward_diff(self.wrt))
    }
}

pub struct Sum<I, D, V> {
    inner: I,
    _phantom: PhantomData<(D, V)>,
}

impl<I, D, V> Sum<I, D, V> {
    pub fn new<T: IntoIterator<IntoIter = I, Item = D>>(x: T) -> Self {
        Self {
            inner: x.into_iter(),
            _phantom: PhantomData,
        }
    }
}

impl<I, D, V> Diff for Sum<I, D, V>
where
    I: Iterator<Item = D> + Clone,
    D: Diff<ValueType = V>,
    V: Scalar,
    V: std::iter::Sum,
{
    type ValueType = V;

    type ForwardDiff = Sum<ForwardDiffIter<I, D>, D::ForwardDiff, V>;

    fn val(&self) -> Self::ValueType {
        self.inner.clone().map(|d| d.val()).sum()
    }

    fn forward_diff<UID: crate::AsVariableUID>(&self, with_respect_to: UID) -> Self::ForwardDiff {
        // todo!()
        Sum {
            inner: ForwardDiffIter {
                inner: self.inner.clone(),
                wrt: with_respect_to.as_vuid(),
                _phantom: PhantomData,
            },
            _phantom: PhantomData,
        }
    }
}
