use std::ops::{Add, Mul};

use crate::{
    op_struct::{Addition, Multiplication},
    scalar::Scalar,
};

use super::{Diff, Variable};

impl<T, V> Add<T> for Variable<V>
where
    V: Scalar,
    T: Diff<ValueType = V>,
{
    type Output = Addition<Self, T, V>;

    fn add(self, rhs: T) -> Self::Output {
        self.add_diff(rhs)
    }
}

impl<T, V> Mul<T> for Variable<V>
where
    V: Scalar,
    T: Diff<ValueType = V>,
{
    type Output = Multiplication<Self, T, V>;

    fn mul(self, rhs: T) -> Self::Output {
        self.mul_diff(rhs)
    }
}

macro_rules! impl_std_ops {
    (<$($typearg:ident),*>,$type:ident) => {
        impl<T, V, $($typearg),* > Add<T> for $type<$($typearg),*, V>
        where
            V: Scalar,
            T: Diff<ValueType = V>,
            $($typearg: Diff<ValueType = V>),*
        {
            type Output = Addition<Self, T, V>;

            fn add(self, rhs: T) -> Self::Output {
                self.add_diff(rhs)
            }
        }

        impl<T, V, $($typearg),* > Mul<T> for $type<$($typearg),*, V>
        where
            V: Scalar,
            T: Diff<ValueType = V>,
            $($typearg: Diff<ValueType = V>),*
        {
            type Output = Multiplication<Self, T, V>;

            fn mul(self, rhs: T) -> Self::Output {
                self.mul_diff(rhs)
            }
        }
        
    };
}

impl_std_ops!(<L, R>, Multiplication);
impl_std_ops!(<L, R>, Addition);


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_mul() {
        let x = Variable::new(1.);
        let xid = x.vuid();
        let y = Variable::new(10.);
        let yid = y.vuid();
        let res = (x + y) * 3.;
        let dx = res.forward_diff(xid);
        let dy = res.forward_diff(yid);
        let dxdy = dx.forward_diff(yid);
        assert_eq!(res.val(), 33.);
        assert_eq!(dx.val(), 3.);
        assert_eq!(dy.val(), 3.);
        assert_eq!(dxdy.val(), 0.)
    }
}
