mod entry;

pub use self::entry::Entry;

use std::ops::RangeInclusive;

use super::Node;

pub struct Find<'a, K: Clone + Ord + 'a, V: 'a> {
    nodes: Vec<&'a Node<K, V>>,
    key: RangeInclusive<K>,
}

impl<'a, K: Clone + Ord + 'a, V: 'a> Find<'a, K, V> {
    pub(crate) fn new(nodes: Vec<&'a Node<K, V>>, key: RangeInclusive<K>) -> Self {
        Self { nodes, key }
    }
}

impl<'a, K: Clone + Ord + 'a, V: 'a> Iterator for Find<'a, K, V> {
    type Item = Entry<'a, K, V>;

    fn next(&mut self) -> Option<Entry<'a, K, V>> {
        loop {
            let node = self.nodes.pop()?;

            if *self.key.start() >= node.max {
                continue;
            }

            if let Some(ref left) = node.left {
                self.nodes.push(left);
            }

            if self.key.end() <= node.key.start() {
                continue;
            }

            if let Some(ref right) = node.right {
                self.nodes.push(right);
            }

            if intersects(&self.key, &node.key) {
                #[allow(deprecated)]
                return Some(Entry {
                    key: &node.key,
                    value: &node.value,
                });
            }
        }
    }
}

fn intersects<K: Clone + Ord>(r: &RangeInclusive<K>, s: &RangeInclusive<K>) -> bool {
    r.start() < s.end() && s.start() < r.end()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersects() {
        assert!(intersects(&(0..=8), &(4..=8)));
        assert!(intersects(&(0..=8), &(-3..=17)));
        assert!(intersects(&(0..=8), &(-2..=2)));
        assert!(intersects(&(0..=8), &(5..=13)));
        assert!(!intersects(&(0..=8), &(-1..=0)));
        assert!(!intersects(&(0..=8), &(-9..=-2)));
        assert!(!intersects(&(0..=8), &(14..=20)));
        assert!(!intersects(&(0..=8), &(8..=9)));
    }
}
