//! This crate provides the `TreeMultiSet` data structure, which is an implementation of a multi-set using a BTree (BTreeMap) in Rust.
//!
//! # Examples
//!
//! ```
//! use tree_multi_set::TreeMultiSet;
//!
//! let mut set = TreeMultiSet::new();
//!
//! // Inserting elements
//! set.insert(1);
//! set.insert(2);
//! set.insert(2);
//!
//! // Counting occurrences
//! assert_eq!(set.count(&1), 1);
//! assert_eq!(set.count(&2), 2);
//!
//! // Removing elements
//! set.remove_one(&2);
//! assert_eq!(set.count(&2), 1);
//!
//! // Iterating over elements
//! for elem in set.iter() {
//!     println!("{}", elem);
//! }
//! ```
//!
//! The `TreeMultiSet` allows for efficient insertion, removal, and counting of elements, making it suitable for scenarios where elements need to be stored along with their counts.
//!

use std::collections::BTreeMap;

/// A data structure representing a multi-set implemented using a BTreeMap.
/// 
/// A `TreeMultiSet` stores elements of type `T` along with their counts,
/// allowing for efficient insertion, removal, and retrieval of elements.
pub struct TreeMultiSet<T> {
    mp: BTreeMap<T, usize>,
    count: usize,
}

impl<T: std::cmp::Ord + Clone> TreeMultiSet<T> {
    /// Constructs a new, empty `TreeMultiSet`.
    pub fn new() -> Self {
        Self {
            mp: BTreeMap::new(),
            count: 0,
        }
    }

    /// Removes all elements from the `TreeMultiSet`, leaving it empty.
    pub fn clear(&mut self) {
        self.mp.clear();
        self.count = 0;
    }

    /// Returns `true` if the `TreeMultiSet` is empty, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Returns the number of elements in the `TreeMultiSet`.
    pub fn len(&self) -> usize {
        self.count
    }

    /// Returns the count of occurrences of a specified element in the `TreeMultiSet`.
    /// # Complexity
    /// O(log n)
    pub fn count(&self, k: &T) -> usize {
        *self.mp.get(k).unwrap_or(&0)
    }

    /// Returns `true` if the `TreeMultiSet` contains a specified element, `false` otherwise.
    /// # Complexity
    /// O(log n)
    pub fn contains(&self, k: &T) -> bool {
        self.mp.contains_key(k)
    }

    /// Returns an immutable reference to the first (minimum) element in the `TreeMultiSet`, or `None` if it is empty.
    /// # Complexity
    /// O(log n)
    pub fn first(&self) -> Option<&T> {
        self.mp.first_key_value().map(|(k, _)| k)
    }

    /// Returns an immutable reference to the last (maximum) element in the `TreeMultiSet`, or `None` if it is empty.
    /// # Complexity
    /// O(log n)
    pub fn last(&self) -> Option<&T> {
        self.mp.last_key_value().map(|(k, _)| k)
    }

    /// Removes one occurrence of the first (minimum) element in the `TreeMultiSet` and returns it, or `None` if the set is empty.
    /// # Complexity
    /// O(log n)
    pub fn pop_first(&mut self) -> Option<T> {
        let first_key = self.mp.first_key_value().map(|kv| kv.0.clone())?;
        self.remove_one(&first_key)
    }

    /// Removes one occurrence of the last (maximum) element in the `TreeMultiSet` and returns it, or `None` if the set is empty.
    /// # Complexity
    /// O(log n)
    pub fn pop_last(&mut self) -> Option<T> {
        let last_key = self.mp.last_key_value().map(|kv|kv.0.clone())?;
        self.remove_one(&last_key)
    }

    /// Inserts an element into the `TreeMultiSet`.
    /// # Complexity
    /// O(log n)
    pub fn insert(&mut self, k: T) {
        self.count += 1;
        *self.mp.entry(k).or_insert(0) += 1;
    }

    /// Removes one occurrence of a specified element from the `TreeMultiSet` and returns it, or `None` if the element is not present.
    /// # Complexity
    /// O(log n)
    pub fn remove_one(&mut self, k: &T) -> Option<T> {
        let Some(v) = self.mp.get_mut(k) else { return None; };
        *v -= 1;
        self.count -= 1;
        if *v == 0 {
            self.mp.remove(k);
        }

        Some(k.clone())
    }

    /// Removes all occurrences of a specified element from the `TreeMultiSet` and returns it, or `None` if the element is not present.
    /// # Complexity
    /// O(log n)
    pub fn remove_all(&mut self, k: &T) -> Option<T>{
        if let Some(v) = self.mp.get_mut(k) {
            self.count -= *v;
            *v = 0;
            self.mp.remove(k);
            return Some(k.clone());
        }
        None
    }

    /// Returns an iterator over the elements of the `TreeMultiSet`.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.mp.iter().flat_map(|(k , &v)| (0..v).map(move |_| k))
    }

    /// Returns an iterator over the elements of the `TreeMultiSet` within a specified range.
    pub fn range<R>(&self, rng: R) -> impl Iterator<Item = &T> + DoubleEndedIterator
    where R: std::ops::RangeBounds<T> {
        self.mp.range(rng).flat_map(|(k , &v)| (0..v).map(move |_| k))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_count() {
        let mut set: TreeMultiSet<i32> = TreeMultiSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);
        set.insert(4);
        set.insert(5);
        set.insert(3);

        assert_eq!(set.count(&1), 1);
        assert_eq!(set.count(&2), 1);
        assert_eq!(set.count(&3), 2);
        assert_eq!(set.count(&4), 2);
        assert_eq!(set.count(&5), 1);
        assert_eq!(set.count(&6), 0);
    }

    #[test]
    fn test_empty_test() {
        let mut set: TreeMultiSet<i32> = TreeMultiSet::new();
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
        assert_eq!(set.first(), None);
        assert_eq!(set.last(), None);
        assert_eq!(set.pop_first(), None);
        assert_eq!(set.pop_last(), None);
        assert_eq!(set.iter().next(), None);
        assert_eq!(set.range(0..10).next(), None);
        assert_eq!(set.range(..).next(), None);
    }

    #[test]
    fn test_insert_remove() {
        let mut set: TreeMultiSet<i32> = TreeMultiSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);
        set.insert(4);
        set.insert(5);
        set.insert(3);

        assert_eq!(set.len(), 7);
        assert_eq!(set.first(), Some(&1));
        assert_eq!(set.last(), Some(&5));
        assert_eq!(set.pop_first(), Some(1));
        assert_eq!(set.first(), Some(&2));
        assert_eq!(set.len(), 6);
        assert_eq!(set.pop_last(), Some(5));
        assert_eq!(set.last(), Some(&4));
        assert_eq!(set.len(), 5);

        assert_eq!(set.remove_one(&3), Some(3));
        assert_eq!(set.len(), 4);
        assert_eq!(set.remove_one(&3), Some(3));
        assert_eq!(set.len(), 3);
        assert_eq!(set.remove_one(&3), None);
        assert_eq!(set.len(), 3);

        assert_eq!(set.remove_all(&4), Some(4));
        assert_eq!(set.len(), 1);
        assert_eq!(set.remove_all(&4), None);

        assert_eq!(set.iter().collect::<Vec<_>>(), vec![&2]);
    }

    #[test]
    fn test_contains() {
        let mut set: TreeMultiSet<i32> = TreeMultiSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);
        set.insert(4);
        set.insert(5);

        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
        assert!(set.contains(&4));
        assert!(set.contains(&5));

        assert!(!set.contains(&6));
        assert!(!set.contains(&7));
    }

    #[test]
    fn test_iter() {
        let mut set = TreeMultiSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(3);
        set.insert(4);
        set.insert(5);
        set.insert(5);

        assert_eq!(set.iter().collect::<Vec<_>>(), vec![&1, &2, &3, &3, &4, &5, &5]);
    }

    #[test]
    fn test_rev_iter() {
        let mut set = TreeMultiSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(3);
        set.insert(4);
        set.insert(5);
        set.insert(5);
    }

    #[test]
    fn test_range() {
        let mut set = TreeMultiSet::new();
        for i in 1..=4 {
            for _ in 0..i {
                set.insert(i);
            }
        }

        assert_eq!(set.range(1..=4).collect::<Vec<_>>(), vec![&1, &2, &2, &3, &3, &3, &4, &4, &4, &4]);
        assert_eq!(set.range(1..3).collect::<Vec<_>>(), vec![&1, &2, &2]);
        assert_eq!(set.range(..3).collect::<Vec<_>>(), vec![&1, &2, &2]);
        assert_eq!(set.range(..=3).collect::<Vec<_>>(), vec![&1, &2, &2, &3, &3, &3]);
        assert_eq!(set.range(..=3).rev().collect::<Vec<_>>(), vec![&3, &3, &3, &2, &2, &1]);
        assert_eq!(set.range(2..).collect::<Vec<_>>(), vec![&2, &2, &3, &3, &3, &4, &4, &4, &4]);
        assert_eq!(set.range(2..=4).rev().collect::<Vec<_>>(), vec![&4, &4, &4, &4, &3, &3, &3, &2, &2]);
    }
}
