use std::ops::{Add, Mul};

use num_traits::{One, Zero};

use crate::op_struct::{Addition, Multiplication};

use super::{Diff, Variable};

impl<T, V> Add<T> for Variable<V>
where
    V: Copy,
    V: Diff<ValueType = V>,
    V: One,
    V: Zero,
    T: Diff<ValueType = V>,
{
    type Output = Addition<Self, T, V>;

    fn add(self, rhs: T) -> Self::Output {
        self.add_diff(rhs)
    }
}

impl<T, V> Mul<T> for Variable<V>
where
    V: Copy,
    V: Diff<ValueType = V>,
    V: One,
    V: Zero,
    T: Diff<ValueType = V>,
{
    type Output = Multiplication<Self, T, V>;

    fn mul(self, rhs: T) -> Self::Output {
        self.mul_diff(rhs)
    }
}

impl<T, L, R, V> Add<T> for Addition<L, R, V>
where
    V: Copy,
    V: Diff<ValueType = V>,
    V: One,
    V: Zero,
    T: Diff<ValueType = V>,
    L: Diff<ValueType = V>,
    R: Diff<ValueType = V>,
{
    type Output = Addition<Self, T, V>;

    fn add(self, rhs: T) -> Self::Output {
        self.add_diff(rhs)
    }
}

impl<T, L, R, V> Mul<T> for Addition<L, R, V>
where
    V: Copy,
    V: Diff<ValueType = V>,
    V: One,
    V: Zero,
    T: Diff<ValueType = V>,
    L: Diff<ValueType = V>,
    R: Diff<ValueType = V>,
{
    type Output = Multiplication<Self, T, V>;

    fn mul(self, rhs: T) -> Self::Output {
        self.mul_diff(rhs)
    }
}

impl<T, L, R, V> Add<T> for Multiplication<L, R, V>
where
    V: Copy,
    V: Diff<ValueType = V>,
    V: One,
    V: Zero,
    T: Diff<ValueType = V>,
    L: Diff<ValueType = V>,
    R: Diff<ValueType = V>,
{
    type Output = Addition<Self, T, V>;

    fn add(self, rhs: T) -> Self::Output {
        self.add_diff(rhs)
    }
}

impl<T, L, R, V> Mul<T> for Multiplication<L, R, V>
where
    V: Copy,
    V: Diff<ValueType = V>,
    V: One,
    V: Zero,
    T: Diff<ValueType = V>,
    L: Diff<ValueType = V>,
    R: Diff<ValueType = V>,
{
    type Output = Multiplication<Self, T, V>;

    fn mul(self, rhs: T) -> Self::Output {
        self.mul_diff(rhs)
    }
}

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
