use std::sync::Arc;

use anyhow::{bail, Result};

use super::impl_heap;

#[derive(Clone)]
enum Tree<T> {
    Empty,
    Node(T, SplayHeap<T>, SplayHeap<T>),
}

#[derive(Clone)]
pub struct SplayHeap<T>(Arc<Tree<T>>);

impl<T> From<Tree<T>> for SplayHeap<T> {
    fn from(x: Tree<T>) -> Self {
        Self(Arc::new(x))
    }
}

impl<T> From<Arc<Tree<T>>> for SplayHeap<T> {
    fn from(x: Arc<Tree<T>>) -> Self {
        Self(x)
    }
}

impl<T: Clone + PartialOrd> SplayHeap<T> {
    pub fn empty() -> Self {
        Self(Arc::new(Tree::Empty))
    }

    pub fn is_empty(&self) -> bool {
        matches!(self.0.as_ref(), Tree::Empty)
    }

    fn node(x: T, a: Self, b: Self) -> Self {
        Self(Arc::new(Tree::Node(x, a, b)))
    }

    fn partition(&self, pivot: T) -> (Self, Self) {
        match self.0.as_ref() {
            Tree::Empty => (Self::empty(), Self::empty()),
            Tree::Node(x, a, b) => {
                if x <= &pivot {
                    match b.0.as_ref() {
                        Tree::Empty => (self.clone(), Self::empty()),
                        Tree::Node(y, b1, b2) => {
                            if y <= &pivot {
                                let (small, big) = b2.partition(pivot);
                                (
                                    Self::node(
                                        y.clone(),
                                        Self::node(x.clone(), a.clone(), b1.clone()),
                                        small,
                                    ),
                                    big,
                                )
                            } else {
                                let (small, big) = b1.partition(pivot);
                                (
                                    Self::node(x.clone(), a.clone(), small),
                                    Self::node(y.clone(), big, b2.clone()),
                                )
                            }
                        }
                    }
                } else {
                    match a.0.as_ref() {
                        Tree::Empty => (Self::empty(), self.clone()),
                        Tree::Node(y, a1, a2) => {
                            if y <= &pivot {
                                let (small, big) = a2.partition(pivot);
                                (
                                    Self::node(y.clone(), a1.clone(), small),
                                    Self::node(x.clone(), big, b.clone()),
                                )
                            } else {
                                let (small, big) = a1.partition(pivot);
                                (
                                    small,
                                    Self::node(
                                        y.clone(),
                                        big,
                                        Self::node(x.clone(), a2.clone(), b.clone()),
                                    ),
                                )
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn insert(&self, x: T) -> Self {
        let (a, b) = self.partition(x.clone());
        Self::node(x, a, b)
    }

    pub fn merge(&self, other: &Self) -> Self {
        match self.0.as_ref() {
            Tree::Empty => other.clone(),
            Tree::Node(x, a, b) => {
                let (ta, tb) = other.partition(x.clone());
                Self::node(x.clone(), ta.merge(a), tb.merge(b))
            }
        }
    }

    pub fn find_min(&self) -> Option<T> {
        match self.0.as_ref() {
            Tree::Empty => None,
            Tree::Node(x, a, _) => match a.0.as_ref() {
                Tree::Empty => Some(x.clone()),
                _ => a.find_min(),
            },
        }
    }

    pub fn delete_min(&self) -> Result<Self> {
        match self.0.as_ref() {
            Tree::Empty => bail!("empty"),
            Tree::Node(y, a, c) => Ok(match a.0.as_ref() {
                Tree::Empty => c.clone(),
                Tree::Node(x, a, b) => match a.0.as_ref() {
                    Tree::Empty => Self::node(y.clone(), b.clone(), c.clone()),
                    Tree::Node(..) => Self::node(
                        x.clone(),
                        a.delete_min()?,
                        Self::node(y.clone(), b.clone(), c.clone()),
                    ),
                },
            }),
        }
    }
}

impl_heap!(SplayHeap);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let h = [3, 1, 4, 0, 2]
            .into_iter()
            .fold(SplayHeap::empty(), |h, x| h.insert(x));

        assert_eq!(h.find_min(), Some(0));
        let h = h.delete_min().unwrap();
        assert_eq!(h.find_min(), Some(1));
        let h = h.delete_min().unwrap();
        assert_eq!(h.find_min(), Some(2));
        let h = h.delete_min().unwrap();
        assert_eq!(h.find_min(), Some(3));
        let h = h.delete_min().unwrap();
        assert_eq!(h.find_min(), Some(4));
        let h = h.delete_min().unwrap();
        assert!(h.find_min().is_none());
    }
}
