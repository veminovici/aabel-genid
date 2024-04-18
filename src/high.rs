use std::ops::{Add, AddAssign, Mul};

use crate::{GenId, KindId, Low, KIND16, KIND2, KIND4, KIND8};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct High<const K: u32>(u32);

impl<const K: u32> High<K> {
    const KIND_MASK_SIZE: u32 = K;
    const HIGH_MASK: u32 = (((1 << (4 - Self::KIND_MASK_SIZE)) - 1) << 28) + 0x0FFF_FFFF;
    const KIND_MASK: u32 = !Self::HIGH_MASK;
    const KIND_MASK_MOVE: u32 = u32::BITS - Self::KIND_MASK_SIZE;

    pub(crate) const DEFAULT: u32 = 1;

    const fn from_raw(value: u32) -> Self {
        Self(value)
    }

    pub(crate) const fn with_default() -> Self {
        Self::from_raw(Self::DEFAULT)
    }

    #[inline(always)]
    const fn inner_pack_kind(high: u32, kind: u8) -> u32 {
        high | (kind as u32) << Self::KIND_MASK_MOVE
    }

    #[inline(always)]
    fn inner_pack_high(high: &Self, value: u32) -> Self {
        let kind = high.kind();
        let mut high = Self::from_raw(value);
        high.set_kind(kind);
        high
    }

    #[inline(always)]
    fn inner_incr(high: &Self, value: u32) -> Self {
        let kind = high.kind();
        let mut high = Self::from_raw(high.0 + value);
        high.set_kind(kind);
        high
    }

    /// Set the [`KindId`] value.
    #[inline(always)]
    pub fn set_kind(&mut self, kind: KindId<K>) {
        self.0 = Self::inner_pack_kind(self.0, kind.0);
    }

    #[inline(always)]
    pub const fn kind(&self) -> KindId<K> {
        KindId(((self.0 & Self::KIND_MASK) >> Self::KIND_MASK_MOVE) as u8)
    }

    #[inline(always)]
    pub fn set_high(&mut self, value: u32) {
        self.0 = Self::inner_pack_high(self, value).0;
    }

    #[inline(always)]
    pub const fn high(&self) -> u32 {
        self.0 & Self::HIGH_MASK
    }

    #[inline(always)]
    pub const fn unpack(self) -> (KindId<K>, u32) {
        let kind = self.kind();
        let high = self.high();
        (kind, high)
    }

    #[inline(always)]
    pub fn incr(&mut self, value: u32) {
        self.0 = Self::inner_incr(self, value).0;
    }
}

pub type High2 = High<KIND2>;
pub type High4 = High<KIND4>;
pub type High8 = High<KIND8>;
pub type High16 = High<KIND16>;

pub fn high<const K: u32>(value: u32) -> High<K> {
    High(value)
}

pub fn high_with_default<const K: u32>() -> High<K> {
    High::with_default()
}

pub fn high2(value: u32) -> High2 {
    High::<KIND2>::from_raw(value)
}

pub const fn high2_with_default() -> High2 {
    High::<KIND2>::with_default()
}

pub const fn high4(value: u32) -> High4 {
    High::<KIND4>::from_raw(value)
}

pub const fn high4_with_default() -> High4 {
    High::<KIND4>::with_default()
}

pub const fn high8(value: u32) -> High8 {
    High::<KIND8>::from_raw(value)
}

pub const fn high8_with_default() -> High8 {
    High::<KIND8>::with_default()
}

pub const fn high16(value: u32) -> High16 {
    High::<KIND16>::from_raw(value)
}

pub const fn high16_with_default() -> High16 {
    High::<KIND16>::with_default()
}

impl<const K: u32> Add<KindId<K>> for High<K> {
    type Output = Self;

    fn add(self, kind: KindId<K>) -> Self::Output {
        let value = Self::inner_pack_kind(self.0, kind.0);
        Self::from_raw(value)
    }
}

impl<const K: u32> AddAssign<KindId<K>> for High<K> {
    fn add_assign(&mut self, kind: KindId<K>) {
        self.set_kind(kind)
    }
}

impl<const K: u32> Add<u32> for High<K> {
    type Output = Self;

    fn add(self, value: u32) -> Self::Output {
        Self::inner_incr(&self, value)
    }
}

impl<const K: u32> AddAssign<u32> for High<K> {
    fn add_assign(&mut self, value: u32) {
        self.incr(value)
    }
}

impl<const K: u32> Mul<Low> for High<K> {
    type Output = GenId<K>;

    fn mul(self, low: Low) -> Self::Output {
        let mut gid = GenId::from_low(low);

        let (kind, high) = self.unpack();
        gid.set_high(high);
        gid.set_kind(kind);

        gid
    }
}

#[cfg(test)]
mod tests {
    use crate::kind;

    use super::*;

    #[test]
    fn constructors() {
        let hi = high2(10);
        assert_eq!(hi.0, 10);

        let hi = high4(10);
        assert_eq!(hi.0, 10);

        let hi = high8(10);
        assert_eq!(hi.0, 10);

        let hi = high16(10);
        assert_eq!(hi.0, 10);
    }

    #[test]
    fn with_default() {
        let hi = high2_with_default();
        assert_eq!(hi.0, 1);

        let hi = high4_with_default();
        assert_eq!(hi.0, 1);

        let hi = high8_with_default();
        assert_eq!(hi.0, 1);

        let hi = high16_with_default();
        assert_eq!(hi.0, 1);
    }

    #[test]
    fn test_kind() {
        let mut hi = high2(10);
        hi.set_kind(kind(1));
        assert_eq!(hi.kind().0, 1);

        let mut hi = high4(10);
        hi.set_kind(kind(3));
        assert_eq!(hi.kind().0, 3);

        let mut hi = high8(10);
        hi.set_kind(kind(7));
        assert_eq!(hi.kind().0, 7);

        let mut hi = high16(10);
        hi.set_kind(kind(15));
        assert_eq!(hi.kind().0, 15);
    }

    #[test]
    fn high() {
        let mut hi = high2(10);
        hi.set_kind(kind(1));
        hi.set_high(20);
        assert_eq!(hi.kind().0, 1);
        assert_eq!(hi.high(), 20);

        let mut hi = high4(10);
        hi.set_kind(kind(3));
        hi.set_high(20);
        assert_eq!(hi.kind().0, 3);
        assert_eq!(hi.high(), 20);

        let mut hi = high8(10);
        hi.set_kind(kind(7));
        hi.set_high(20);
        assert_eq!(hi.kind().0, 7);
        assert_eq!(hi.high(), 20);

        let mut hi = high16(10);
        hi.set_kind(kind(15));
        hi.set_high(20);
        assert_eq!(hi.kind().0, 15);
        assert_eq!(hi.high(), 20);
    }

    #[test]
    fn unpack() {
        let mut hi = high2(10);
        hi.set_kind(kind(1));
        hi.set_high(20);
        let (k, high) = hi.unpack();
        assert_eq!(k.0, 1);
        assert_eq!(high, 20);

        let mut hi = high4(10);
        hi.set_kind(kind(3));
        hi.set_high(20);
        let (k, high) = hi.unpack();
        assert_eq!(k.0, 3);
        assert_eq!(high, 20);

        let mut hi = high8(10);
        hi.set_kind(kind(7));
        hi.set_high(20);
        let (k, high) = hi.unpack();
        assert_eq!(k.0, 7);
        assert_eq!(high, 20);

        let mut hi = high16(10);
        hi.set_kind(kind(15));
        hi.set_high(20);
        let (kind, high) = hi.unpack();
        assert_eq!(kind.0, 15);
        assert_eq!(high, 20);
    }

    #[test]
    fn incr() {
        let mut hi = high2(10);
        hi.set_kind(kind(1));
        hi.set_high(20);
        hi.incr(10);
        let high = hi.high();
        assert_eq!(high, 30);

        let mut hi = high4(10);
        hi.set_kind(kind(3));
        hi.set_high(20);
        hi.incr(10);
        let high = hi.high();
        assert_eq!(high, 30);

        let mut hi = high8(10);
        hi.set_kind(kind(7));
        hi.set_high(20);
        hi.incr(10);
        let high = hi.high();
        assert_eq!(high, 30);

        let mut hi = high16(10);
        hi.set_kind(kind(15));
        hi.set_high(20);
        hi.incr(10);
        let high = hi.high();
        assert_eq!(high, 30);
    }

    #[test]
    fn add_value() {
        let mut hi = high2(10);
        hi.set_kind(kind(1));
        hi.set_high(20);
        let hi = hi + 10;
        let high = hi.high();
        assert_eq!(high, 30);

        let mut hi = high4(10);
        hi.set_kind(kind(3));
        hi.set_high(20);
        let hi = hi + 10;
        let high = hi.high();
        assert_eq!(high, 30);

        let mut hi = high8(10);
        hi.set_kind(kind(7));
        hi.set_high(20);
        let hi = hi + 10;
        let high = hi.high();
        assert_eq!(high, 30);

        let mut hi = high16(10);
        hi.set_kind(kind(15));
        hi.set_high(20);
        let hi = hi + 10;
        let high = hi.high();
        assert_eq!(high, 30);
    }

    #[test]
    fn add_assign_value() {
        let mut hi = high2(10);
        hi.set_kind(kind(1));
        hi.set_high(20);
        hi += 10;
        let high = hi.high();
        assert_eq!(high, 30);

        let mut hi = high4(10);
        hi.set_kind(kind(3));
        hi.set_high(20);
        hi += 10;
        let high = hi.high();
        assert_eq!(high, 30);

        let mut hi = high8(10);
        hi.set_kind(kind(7));
        hi.set_high(20);
        hi += 10;
        let high = hi.high();
        assert_eq!(high, 30);

        let mut hi = high16(10);
        hi.set_kind(kind(15));
        hi.set_high(20);
        hi += 10;
        let high = hi.high();
        assert_eq!(high, 30);
    }

    #[test]
    fn add_kind() {
        let hi = high2(10);
        let hi = hi + kind(1);
        let hi = hi + 10;
        assert_eq!(hi.high(), 20);

        let hi = high4(10);
        let hi = hi + kind(3);
        let hi = hi + 10;
        assert_eq!(hi.high(), 20);

        let hi = high8(10);
        let hi = hi + kind(7);
        let hi = hi + 10;
        assert_eq!(hi.high(), 20);

        let hi = high16(10);
        let hi = hi + kind(15);
        let hi = hi + 10;
        assert_eq!(hi.high(), 20);
    }

    #[test]
    fn add_assign_kind() {
        let mut hi = high2(10);
        hi += kind(1);
        let hi = hi + 10;
        assert_eq!(hi.high(), 20);

        let mut hi = high4(10);
        hi += kind(3);
        let hi = hi + 10;
        assert_eq!(hi.high(), 20);

        let mut hi = high8(10);
        hi += kind(7);
        let hi = hi + 10;
        assert_eq!(hi.high(), 20);

        let mut hi = high16(10);
        hi += kind(15);
        let hi = hi + 10;
        assert_eq!(hi.high(), 20);
    }
}
