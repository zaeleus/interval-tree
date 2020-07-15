use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct Entry<'a, K: Clone + Ord, V> {
    key: &'a RangeInclusive<K>,
    value: &'a V,
}

impl<'a, K: Clone + Ord, V> Entry<'a, K, V> {
    pub(crate) fn new(key: &'a RangeInclusive<K>, value: &'a V) -> Self {
        Self { key, value }
    }

    /// Returns a reference to the key in the entry.
    pub fn key(&self) -> &RangeInclusive<K> {
        #[allow(deprecated)]
        self.key
    }

    /// Returns a reference to the value in the entry.
    pub fn get(&self) -> &V {
        #[allow(deprecated)]
        self.value
    }
}
