use std::ops::RangeInclusive;

pub(crate) struct Node<K: Clone + Ord, V> {
    pub(crate) key: RangeInclusive<K>,
    pub(crate) value: V,
    pub(crate) max: K,
    pub(crate) height: u32,
    pub(crate) left: Option<Box<Node<K, V>>>,
    pub(crate) right: Option<Box<Node<K, V>>>,
}

impl<K: Clone + Ord, V> Node<K, V> {
    pub(crate) fn new(key: RangeInclusive<K>, value: V) -> Self {
        let max = key.end().clone();

        Self {
            key,
            value,
            max,
            height: 1,
            left: None,
            right: None,
        }
    }
}
