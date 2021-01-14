use crate::{Diff, Expr, VariableUID};

#[derive(Clone)]
pub struct ForwardDiffIter<I> {
    inner: I,
    wrt: VariableUID,
}

impl<I> Iterator for ForwardDiffIter<I>
where
    I: Iterator,
    I::Item: Diff,
{
    type Item = <I::Item as Diff>::ForwardDiff;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|d| d.forward_diff(self.wrt))
    }
}

#[derive(Clone)]
pub struct Sum<I> {
    inner: I,
}

impl<I> Sum<I>
where
    I: Iterator,
{
    pub fn new(x: I) -> Self {
        Self { inner: x }
    }
}

impl<I> Expr for Sum<I>
where
    I: Iterator,
    I::Item: Expr,
{
    type ValueType = <I::Item as Expr>::ValueType;
}

impl<I> Diff for Sum<I>
where
    I: Iterator + Clone,
    I::Item: Diff,
    Self::ValueType: std::iter::Sum,
{
    type ForwardDiff = Sum<ForwardDiffIter<I>>;

    fn val(&self) -> Self::ValueType {
        self.inner.clone().map(|d| d.val()).sum()
    }

    fn forward_diff<UID: crate::AsVariableUID>(&self, with_respect_to: UID) -> Self::ForwardDiff {
        Sum {
            inner: ForwardDiffIter {
                inner: self.inner.clone(),
                wrt: with_respect_to.as_vuid(),
            },
        }
    }
}
