use std::{borrow::Borrow, ops::Mul};

use crate::{GenId, High, KindId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Low(pub(crate) u32);

impl Low {
    pub fn from_raw(value: u32) -> Self {
        Self(value)
    }
}

impl From<Low> for u32 {
    fn from(low: Low) -> Self {
        low.0
    }
}

impl AsRef<u32> for Low {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}

impl Borrow<u32> for Low {
    fn borrow(&self) -> &u32 {
        self.as_ref()
    }
}

impl<const K: u32> Mul<High<K>> for Low {
    type Output = GenId<K>;

    fn mul(self, high: High<K>) -> Self::Output {
        let mut gid = GenId::from_low(self);

        let (kind, high) = high.unpack();
        gid.set_kind(kind);
        gid.set_high(high);

        gid
    }
}

impl<const K: u32> Mul<KindId<K>> for Low {
    type Output = GenId<K>;

    fn mul(self, kind: KindId<K>) -> Self::Output {
        let mut gid = GenId::from_low(self);

        gid.set_kind(kind);
        gid.set_high(High::<K>::DEFAULT);

        gid
    }
}
