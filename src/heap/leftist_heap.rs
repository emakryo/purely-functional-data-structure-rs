use std::sync::Arc;

use anyhow::{bail, Result};

use super::impl_heap;

#[derive(Clone)]
pub struct LeftistHeap<T>(Arc<Tree<T>>);

#[derive(Clone)]
enum Tree<T> {
    Empty,
    Node(usize, T, LeftistHeap<T>, LeftistHeap<T>),
}

impl<T> From<Tree<T>> for LeftistHeap<T> {
    fn from(x: Tree<T>) -> Self {
        Self(Arc::new(x))
    }
}

impl<T: Clone + PartialOrd> LeftistHeap<T> {
    pub fn empty() -> Self {
        Tree::Empty.into()
    }
    pub fn is_empty(&self) -> bool {
        matches!(self.0.as_ref(), Tree::Empty)
    }
    pub fn insert(&self, val: T) -> Self {
        let e = Self::empty();
        self.merge(&Self::make_node(val, e.clone(), e))
    }
    pub fn merge(&self, other: &Self) -> Self {
        match (self.0.as_ref(), other.0.as_ref()) {
            (_, Tree::Empty) => self.clone(),
            (Tree::Empty, _) => other.clone(),
            (Tree::Node(_, x, a1, b1), Tree::Node(_, y, a2, b2)) => {
                if x <= y {
                    Self::make_node(x.clone(), a1.clone(), b1.merge(other))
                } else {
                    Self::make_node(y.clone(), a2.clone(), self.merge(b2))
                }
            }
        }
    }
    pub fn find_min(&self) -> Option<T> {
        match self.0.as_ref() {
            Tree::Empty => None,
            Tree::Node(_, x, _, _) => Some(x.clone()),
        }
    }
    pub fn delete_min(&self) -> Result<Self> {
        match self.0.as_ref() {
            Tree::Empty => bail!("Empty"),
            Tree::Node(_, _, a, b) => Ok(a.merge(b)),
        }
    }

    fn rank(&self) -> usize {
        match self.0.as_ref() {
            Tree::Empty => 0,
            Tree::Node(r, ..) => *r,
        }
    }
    fn make_node(x: T, a: Self, b: Self) -> Self {
        if a.rank() >= b.rank() {
            Tree::Node(b.rank() + 1, x, a, b).into()
        } else {
            Tree::Node(a.rank() + 1, x, b, a).into()
        }
    }
    /// exercise 3.2
    pub fn insert2(&self, x: T) -> Self {
        let e = Self::empty();
        match self.0.as_ref() {
            Tree::Empty => Self::make_node(x, e.clone(), e),
            Tree::Node(_, y, a, b) => {
                if &x <= y {
                    Self::make_node(x, a.clone().insert2(y.clone()), b.clone())
                } else {
                    Self::make_node(y.clone(), a.clone().insert2(x), b.clone())
                }
            }
        }
    }
}

impl_heap!(LeftistHeap);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{heap::Heap, List};

    #[test]
    fn test_heap() {
        let h = LeftistHeap::empty();
        let h = h.insert(10);
        let h = h.insert(0);
        let h = h.insert(20);

        assert_eq!(h.find_min(), Some(0));
        let h = h.delete_min().unwrap();
        assert_eq!(h.find_min(), Some(10));
    }

    #[test]
    fn test_heap_insert2() {
        let h = LeftistHeap::empty();
        let h = h.insert2(10);
        let h = h.insert2(0);
        let h = h.insert2(20);

        assert_eq!(h.find_min(), Some(0));
        let h = h.delete_min().unwrap();
        assert_eq!(h.find_min(), Some(10));
    }

    #[test]
    fn test_from_list() {
        let xs = [1, 3, 2, 4];
        let mut l = List::empty();
        for &x in &xs {
            l = List::cons(x, l);
        }
        let h: LeftistHeap<_> = LeftistHeap::from_list(l);
        assert_eq!(h.find_min(), Some(1));
        let h = h.delete_min().unwrap();
        assert_eq!(h.find_min(), Some(2));
        let h = h.delete_min().unwrap();
        assert_eq!(h.find_min(), Some(3));
        let h = h.delete_min().unwrap();
        assert_eq!(h.find_min(), Some(4));
    }
}
