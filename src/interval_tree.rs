mod find;
mod node;

pub use self::find::Find;

use std::{cmp, ops::RangeInclusive};

use self::node::Node;

/// A self-balancing binary search tree optimized to hold interval-value pairs.
#[derive(Default)]
pub struct IntervalTree<K: Clone + Ord, V> {
    root: Option<Box<Node<K, V>>>,
}

impl<K: Clone + Ord, V> IntervalTree<K, V> {
    /// Creates an empty interval tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use interval_tree::IntervalTree;
    /// let _tree: IntervalTree<u64, &str> = IntervalTree::new();
    /// ```
    pub fn new() -> Self {
        Self { root: None }
    }

    /// Adds an interval-value pair into the tree.
    ///
    /// Upon a collision, a new node is added as the left child of the existing node.
    ///
    /// # Examples
    ///
    /// ```
    /// use interval_tree::IntervalTree;
    ///
    /// let mut tree = IntervalTree::new();
    ///
    /// tree.insert(2..=6, "elm");
    /// tree.insert(7..=13, "ash");
    /// tree.insert(7..=13, "walnut");
    /// ```
    pub fn insert(&mut self, key: RangeInclusive<K>, value: V) {
        self.root = if let Some(root) = self.root.take() {
            Some(insert(root, key, value))
        } else {
            Some(Box::new(Node::new(key, value)))
        };
    }

    /// Returns an iterator visiting nodes that intersect the given key.
    ///
    /// # Examples
    ///
    /// ```
    /// use interval_tree::IntervalTree;
    ///
    /// let mut tree = IntervalTree::new();
    ///
    /// tree.insert(2..=6, "elm");
    /// tree.insert(7..=13, "ash");
    /// tree.insert(3..=9, "walnut");
    ///
    /// let mut iter = tree.find(8..=10);
    ///
    /// let entry = iter.next().unwrap();
    /// assert_eq!(entry.key(), &(3..=9));
    /// assert_eq!(entry.get(), &"walnut");
    ///
    /// let entry = iter.next().unwrap();
    /// assert_eq!(entry.key(), &(7..=13));
    /// assert_eq!(entry.get(), &"ash");
    ///
    /// assert!(iter.next().is_none());
    /// ```
    pub fn find(&self, key: RangeInclusive<K>) -> Find<K, V> {
        let nodes = self.root.iter().map::<&Node<K, V>, _>(|n| n).collect();
        Find::new(nodes, key)
    }
}

fn height<K: Clone + Ord, V>(root: &Option<Box<Node<K, V>>>) -> u32 {
    root.as_ref().map_or(0, |n| n.height)
}

enum BalanceFactor {
    LeftHeavy,
    Balanced,
    RightHeavy,
}

fn balance_factor<K: Clone + Ord, V>(root: &Node<K, V>) -> BalanceFactor {
    let left_height = height(&root.left) as i32;
    let right_height = height(&root.right) as i32;

    if left_height > right_height && left_height - right_height >= 2 {
        BalanceFactor::LeftHeavy
    } else if left_height < right_height && right_height - left_height >= 2 {
        BalanceFactor::RightHeavy
    } else {
        BalanceFactor::Balanced
    }
}

fn update_height<K: Clone + Ord, V>(root: &mut Node<K, V>) {
    let left_height = height(&root.left);
    let right_height = height(&root.right);
    root.height = cmp::max(left_height, right_height) + 1;
}

fn update_max<K: Clone + Ord, V>(root: &mut Node<K, V>) {
    root.max = root.key.end().clone();

    if let Some(ref left) = root.left {
        if left.max > root.max {
            root.max = left.max.clone();
        }
    }

    if let Some(ref right) = root.right {
        if right.max > root.max {
            root.max = right.max.clone();
        }
    }
}

fn rotate_left<K: Clone + Ord, V>(mut root: Box<Node<K, V>>) -> Box<Node<K, V>> {
    let mut new_root = root.right.take().expect("invalid tree");

    root.right = new_root.left.take();
    update_height(&mut root);
    update_max(&mut root);

    new_root.left = Some(root);
    update_height(&mut new_root);
    update_max(&mut new_root);

    new_root
}

fn balance_left_heavy_tree<K: Clone + Ord, V>(mut root: Box<Node<K, V>>) -> Box<Node<K, V>> {
    let left = root.left.take().expect("invalid tree");

    if height(&left.left) < height(&left.right) {
        let new_left = rotate_left(left);
        root.left = Some(new_left);
        update_height(&mut root);
        update_max(&mut root);
    } else {
        root.left = Some(left);
    }

    rotate_right(root)
}

fn rotate_right<K: Clone + Ord, V>(mut root: Box<Node<K, V>>) -> Box<Node<K, V>> {
    let mut new_root = root.left.take().expect("invalid tree");

    root.left = new_root.right.take();
    update_height(&mut root);
    update_max(&mut root);

    new_root.right = Some(root);
    update_height(&mut new_root);
    update_max(&mut new_root);

    new_root
}

fn balance_right_heavy_tree<K: Clone + Ord, V>(mut root: Box<Node<K, V>>) -> Box<Node<K, V>> {
    let right = root.right.take().expect("invalid tree");

    if height(&right.left) > height(&right.right) {
        let new_right = rotate_right(right);
        root.right = Some(new_right);
        update_height(&mut root);
        update_max(&mut root);
    } else {
        root.right = Some(right);
    }

    rotate_left(root)
}

fn balance<K: Clone + Ord, V>(root: Box<Node<K, V>>) -> Box<Node<K, V>> {
    match balance_factor(&root) {
        BalanceFactor::LeftHeavy => balance_left_heavy_tree(root),
        BalanceFactor::Balanced => root,
        BalanceFactor::RightHeavy => balance_right_heavy_tree(root),
    }
}

fn insert<K, V>(mut root: Box<Node<K, V>>, key: RangeInclusive<K>, value: V) -> Box<Node<K, V>>
where
    K: Clone + Ord,
{
    if key.start() <= root.key.start() {
        root.left = if let Some(left) = root.left.take() {
            Some(insert(left, key, value))
        } else {
            Some(Box::new(Node::new(key, value)))
        }
    } else if key.start() > root.key.start() {
        root.right = if let Some(right) = root.right.take() {
            Some(insert(right, key, value))
        } else {
            Some(Box::new(Node::new(key, value)))
        }
    }

    update_height(&mut root);
    update_max(&mut root);

    balance(root)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_tree() -> IntervalTree<i32, i32> {
        //          15..18
        //        /        \
        //    5..8          17..19
        //   /    \         /    \
        // 4..8  7..10  16..22  21..24
        let mut tree = IntervalTree::new();

        tree.insert(17..=19, 0);
        tree.insert(5..=8, 1);
        tree.insert(21..=24, 2);
        tree.insert(4..=8, 3);
        tree.insert(15..=18, 4);
        tree.insert(7..=10, 5);
        tree.insert(16..=22, 6);

        tree
    }

    #[test]
    fn test_insert() {
        let tree = build_tree();

        let root = tree.root.as_ref().unwrap();
        assert_eq!(root.key, 15..=18);
        assert_eq!(root.value, 4);
        assert_eq!(root.max, 24);
        assert_eq!(root.height, 3);

        let node = root.left.as_ref().unwrap();
        assert_eq!(node.key, 5..=8);
        assert_eq!(node.value, 1);
        assert_eq!(node.max, 10);
        assert_eq!(node.height, 2);

        let node = root
            .left
            .as_ref()
            .and_then(|node| node.left.as_ref())
            .unwrap();
        assert_eq!(node.key, 4..=8);
        assert_eq!(node.value, 3);
        assert_eq!(node.max, 8);
        assert_eq!(node.height, 1);

        let node = root
            .left
            .as_ref()
            .and_then(|node| node.right.as_ref())
            .unwrap();
        assert_eq!(node.key, 7..=10);
        assert_eq!(node.value, 5);
        assert_eq!(node.max, 10);
        assert_eq!(node.height, 1);

        let node = root.right.as_ref().unwrap();
        assert_eq!(node.key, 17..=19);
        assert_eq!(node.value, 0);
        assert_eq!(node.max, 24);
        assert_eq!(node.height, 2);

        let node = root
            .right
            .as_ref()
            .and_then(|node| node.left.as_ref())
            .unwrap();
        assert_eq!(node.key, 16..=22);
        assert_eq!(node.value, 6);
        assert_eq!(node.max, 22);
        assert_eq!(node.height, 1);

        let node = root
            .right
            .as_ref()
            .and_then(|node| node.right.as_ref())
            .unwrap();
        assert_eq!(node.key, 21..=24);
        assert_eq!(node.value, 2);
        assert_eq!(node.max, 24);
        assert_eq!(node.height, 1);
    }

    #[test]
    fn test_find() {
        let tree = build_tree();
        let entries: Vec<_> = tree.find(7..=20).collect();

        assert_eq!(entries.len(), 6);

        assert_eq!(entries[0].key(), &(15..=18));
        assert_eq!(entries[0].get(), &4);

        assert_eq!(entries[1].key(), &(17..=19));
        assert_eq!(entries[1].get(), &0);

        assert_eq!(entries[2].key(), &(16..=22));
        assert_eq!(entries[2].get(), &6);

        assert_eq!(entries[3].key(), &(5..=8));
        assert_eq!(entries[3].get(), &1);

        assert_eq!(entries[4].key(), &(7..=10));
        assert_eq!(entries[4].get(), &5);

        assert_eq!(entries[5].key(), &(4..=8));
        assert_eq!(entries[5].get(), &3);
    }
}
