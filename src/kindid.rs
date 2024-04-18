use crate::{GenId, High, Low, KIND16, KIND2, KIND4, KIND8};
use std::{
    fmt::Display,
    ops::{Add, Mul},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct KindId<const K: u32>(pub(crate) u8);

impl<const K: u32> Display for KindId<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.0, K)
    }
}

pub const fn kind<const K: u32>(value: u8) -> KindId<K> {
    KindId(value)
}

pub const fn kind2(value: u8) -> KindId<KIND2> {
    KindId(value)
}

pub const fn kind4(value: u8) -> KindId<KIND4> {
    KindId(value)
}

pub const fn kind8(value: u8) -> KindId<KIND8> {
    KindId(value)
}

pub const fn kind16(value: u8) -> KindId<KIND16> {
    KindId(value)
}

impl<const K: u32> Add<High<K>> for KindId<K> {
    type Output = High<K>;

    fn add(self, high: High<K>) -> Self::Output {
        high + self
    }
}

impl<const K: u32> Mul<Low> for KindId<K> {
    type Output = GenId<K>;

    fn mul(self, low: Low) -> Self::Output {
        low * self
    }
}

#[cfg(test)]
mod tests {
    use crate::high2;

    use super::*;

    #[test]
    fn contructors() {
        let kind = kind2(10);
        assert_eq!(kind.0, 10);

        let kind = kind4(10);
        assert_eq!(kind.0, 10);

        let kind = kind8(10);
        assert_eq!(kind.0, 10);

        let kind = kind16(10);
        assert_eq!(kind.0, 10);
    }

    #[test]
    fn add_high() {
        let hi = high2(10);
        let kind = kind2(1);
        let hi = kind + hi;
        let (kind, high) = hi.unpack();
        assert_eq!(kind.0, 1);
        assert_eq!(high, 10);
    }

    #[test]
    fn mul_low() {
        let low = Low::from_raw(10);
        let kind = kind2(1);
        let gid = kind * low;
        let (kind, high, low) = gid.unpack();
        assert_eq!(kind.0, 1);
        assert_eq!(high, High::<KIND2>::DEFAULT);
        assert_eq!(low.0, 10);
    }
}
