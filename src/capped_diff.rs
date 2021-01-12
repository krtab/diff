use std::marker::PhantomData;

use crate::Diff;

trait Cap {}

struct CappedDif<D, C>
where
    D: Diff,
{
    inner: D,
    _phantom: PhantomData<C>,
}

struct Cap01;

impl Cap for Cap01 {}

impl<D: Diff> Diff for CappedDif<D, Cap01> {
    type ValueType = D::ValueType;

    type ForwardDiff = D::ValueType;

    fn val(&self) -> Self::ValueType {
        self.inner.val()
    }

    fn forward_diff<UID: crate::AsVariableUID>(&self, with_respect_to: UID) -> Self::ForwardDiff {
        self.inner.forward_diff(with_respect_to).val()
    }
}

macro_rules! capped_diff_gt1 {
    ($type:ident, $prevtype:ident) => {
        struct $type;

        impl Cap for $type {}

        impl<D: Diff> Diff for CappedDif<D, $type> {
            type ValueType = D::ValueType;

            type ForwardDiff = CappedDif<D::ForwardDiff, $prevtype>;

            fn val(&self) -> Self::ValueType {
                self.inner.val()
            }

            fn forward_diff<UID: crate::AsVariableUID>(
                &self,
                with_respect_to: UID,
            ) -> Self::ForwardDiff {
                CappedDif {
                    inner: self.inner.forward_diff(with_respect_to),
                    _phantom: PhantomData,
                }
            }
        }
    };
}

macro_rules! zip_capped_diffs {
    ($type:ident, $prevtype:ident$(,)? $($rest:ident),*) => {
        capped_diff_gt1!($type, $prevtype);
        zip_capped_diffs!($prevtype, $($rest),* );
    };
    ($type:ident, ) => {
    };
}

zip_capped_diffs!(Cap05, Cap04, Cap03, Cap02, Cap01);
