use std::{iter, ops};

/// ApterTree is a tree data structure that stores elements of type `T`.
///
/// ```rust
/// use apter::ApterTree;
/// let mut tree = ApterTree::new();
/// tree.insert("root", usize::MAX);
/// tree.insert("a", 0);
/// tree.insert("b", 0);
/// assert_eq!(tree.len(), 3);
/// ```
#[derive(Clone, Debug)]
pub struct ApterTree<T> {
    pub d: Vec<T>,
    pub p: Vec<usize>,
}

impl<T> ApterTree<T> {
    /// Create a new empty Apter Tree
    pub const fn new() -> Self {
        Self {
            d: vec![],
            p: vec![],
        }
    }

    /// Returns the total number of elements in the tree.
    pub fn len(&self) -> usize {
        self.p.len()
    }

    /// Returns `true` if the tree contains no elements.
    pub fn is_empty(&self) -> bool {
        self.p.is_empty()
    }

    /// Returns an iterator over all item indices in the tree.
    pub fn keys(&self) -> ops::Range<usize> {
        0..self.len()
    }

    /// Insert a new item into the tree, with the given parent index. By
    /// convention, the root node has a parent index of `usize::MAX`.
    pub fn insert(&mut self, v: T, parent_idx: usize) {
        self.d.push(v);
        self.p.push(parent_idx);
    }

    /// Returns the parent index of the given child index.
    pub fn parent_of(&self, child_idx: usize) -> usize {
        self.p[child_idx]
    }

    /// Returns a reference to the item at the given index.
    pub fn get(&self, idx: usize) -> Option<&T> {
        self.d.get(idx)
    }

    /// Returns a mutable reference to the item at the given index.
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        self.d.get_mut(idx)
    }

    /// Iterates through all items in the tree in insertion order.
    pub fn iter(&self) -> impl Iterator<Item = (usize, &T)> {
        self.keys().map(move |idx| (idx, &self.d[idx]))
    }

    /// Searches for an item in the tree and returns its index if found.
    pub fn find(&self, v: &T) -> Option<usize>
    where
        T: PartialEq,
    {
        self.d.iter().position(|x| x == v)
    }

    /// Returns an iterator through all children of the given parent index.
    pub fn children(&self, parent_idx: usize) -> impl Iterator<Item = usize> + '_ {
        self.keys().filter(move |idx| self.p[*idx] == parent_idx)
    }

    /// Returns `true` if the item at `idx` is a leaf node.
    pub fn is_leaf(&self, idx: usize) -> bool {
        self.children(idx).next().is_none()
    }

    /// Returns an iterator through all leaf nodes in the tree.
    pub fn leaves(&self) -> impl Iterator<Item = usize> + '_ {
        self.keys()
            .filter(move |idx| self.children(*idx).next().is_none())
    }

    /// Returns an iterator through all ancestors of the given index.
    pub fn ancestors(&self, idx: usize) -> impl Iterator<Item = usize> + '_ {
        let parent = self.parent_of(idx);
        let len = self.len();
        iter::successors(Some(parent), |&idx| {
            self.get(idx).map(|_| self.parent_of(idx))
        })
        .take_while(move |&idx| idx < len)
    }

    /// Delete the node at the given index. This is an O(n) operation since all
    /// indices after the deleted node must be shifted down by one. The node
    /// being deleted should not have any child elements, otherwise they will
    /// point at the wrong parent index.
    pub fn delete(&mut self, idx: usize) -> Option<T> {
        if idx >= self.len() {
            return None;
        }

        let v = self.d.remove(idx);
        self.p.remove(idx);

        for i in 0..self.len() {
            if self.p[i] > idx {
                self.p[i] -= 1;
            }
        }

        Some(v)
    }
}

impl<T> Default for ApterTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ops::Index<usize> for ApterTree<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.d[idx]
    }
}

impl<T> ops::IndexMut<usize> for ApterTree<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.d[idx]
    }
}
