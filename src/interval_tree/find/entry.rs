use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct Entry<'a, K: Clone + Ord, V> {
    #[deprecated(note = "use `entry.key()` instead")]
    pub key: &'a RangeInclusive<K>,
    #[deprecated(note = "use `entry.get()` instead")]
    pub value: &'a V,
}

impl<'a, K: Clone + Ord, V> Entry<'a, K, V> {
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
