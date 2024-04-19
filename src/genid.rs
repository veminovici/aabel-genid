use crate::{High, KindId, Low, KIND16, KIND2, KIND4, KIND8};
use std::hash::Hash;
use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, Copy)]
pub struct GenId<const K: u32> {
    low: Low,
    high: High<K>,
}

impl<const K: u32> GenId<K> {
    #[inline(always)]
    pub const fn to_u64(&self) -> u64 {
        let high = self.high.inner();
        let low = self.low.inner();
        ((high as u64) << u32::BITS) | (low as u64)
    }

    #[inline(always)]
    pub(crate) const fn from_low(low: Low) -> Self {
        Self {
            low,
            high: High::with_default(),
        }
    }

    #[inline(always)]
    pub fn set_low(&mut self, low: Low) {
        self.low = low;
    }

    #[inline(always)]
    pub const fn low(&self) -> Low {
        self.low
    }

    #[inline(always)]
    pub fn set_kind(&mut self, kind: KindId<K>) {
        self.high.set_kind(kind);
    }

    #[inline(always)]
    pub const fn kind(&self) -> KindId<K> {
        self.high.kind()
    }

    #[inline(always)]
    pub fn set_high(&mut self, value: u32) {
        self.high.set_high(value)
    }

    #[inline(always)]
    pub const fn high(&self) -> u32 {
        self.high.high()
    }

    #[inline(always)]
    pub const fn unpack(self) -> (KindId<K>, u32, Low) {
        let (kind, high) = self.high.unpack();
        (kind, high, self.low)
    }

    #[inline(always)]
    pub fn incr(&mut self, value: u32) {
        self.high.incr(value)
    }
}

impl<const K: u32> PartialEq for GenId<K> {
    fn eq(&self, other: &Self) -> bool {
        self.to_u64() == other.to_u64()
    }
}

impl<const K: u32> Eq for GenId<K> {}

impl<const K: u32> PartialOrd for GenId<K> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const K: u32> Ord for GenId<K> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_u64().cmp(&other.to_u64())
    }
}

impl<const K: u32> Hash for GenId<K> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_u64().hash(state);
    }
}

impl<const K: u32> Add<u32> for GenId<K> {
    type Output = Self;

    fn add(self, value: u32) -> Self::Output {
        Self {
            low: self.low,
            high: self.high + value,
        }
    }
}

impl<const K: u32> AddAssign<u32> for GenId<K> {
    fn add_assign(&mut self, value: u32) {
        self.high += value;
    }
}

pub fn genid2_from_low(low: Low) -> GenId<KIND2> {
    GenId::from_low(low)
}

pub fn genid4_from_low(low: Low) -> GenId<KIND4> {
    GenId::from_low(low)
}

pub fn genid8_from_low(low: Low) -> GenId<KIND8> {
    GenId::from_low(low)
}

pub fn genid16_from_low(low: Low) -> GenId<KIND16> {
    GenId::from_low(low)
}
