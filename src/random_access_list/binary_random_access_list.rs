use anyhow::{bail, Result};
use std::sync::Arc;

use crate::{stack::Cell, List};

use super::impl_ralist;

#[derive(Clone)]
pub struct BinaryRandomAccessList<T>(List<Digit<T>>);

#[derive(Clone)]
enum Digit<T> {
    Zero,
    One(ATree<T>),
}

#[derive(Clone)]
enum Tree<T> {
    Leaf(T),
    Node(usize, ATree<T>, ATree<T>),
}

#[derive(Clone)]
struct ATree<T>(Arc<Tree<T>>);

impl<T: Clone> BinaryRandomAccessList<T> {
    pub fn empty() -> Self {
        Self(List::empty())
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    fn cons_tree(t: ATree<T>, ts: Self) -> Self {
        Self(match &*ts.0 {
            Cell::Nil => List::cons(Digit::One(t), List::empty()),
            Cell::Cons(Digit::Zero, ts) => List::cons(Digit::One(t), ts.clone()),
            Cell::Cons(Digit::One(tp), ts) => {
                List::cons(Digit::Zero, Self::cons_tree(t.link(tp), Self(ts.clone())).0)
            }
        })
    }
    fn uncons_tree(&self) -> Result<(ATree<T>, Self)> {
        Ok(match &*self.0 {
            Cell::Nil => bail!("empty"),
            Cell::Cons(Digit::One(t), ts) => match &**ts {
                Cell::Nil => (t.clone(), Self::empty()),
                _ => (t.clone(), Self(List::cons(Digit::Zero, ts.clone()))),
            },
            Cell::Cons(Digit::Zero, ts) => {
                let (t, ts) = Self(ts.clone()).uncons_tree()?;
                match t.0.as_ref() {
                    Tree::Node(_, t1, t2) => {
                        (t1.clone(), Self(List::cons(Digit::One(t2.clone()), ts.0)))
                    }
                    _ => unreachable!(),
                }
            }
        })
    }
    pub fn cons(x: T, xs: &Self) -> Self {
        Self::cons_tree(ATree(Arc::new(Tree::Leaf(x))), xs.clone())
    }
    pub fn head(&self) -> Option<T> {
        match self.uncons_tree().ok()?.0 .0.as_ref() {
            Tree::Leaf(x) => Some(x.clone()),
            _ => unreachable!(),
        }
    }
    pub fn tail(&self) -> Option<Self> {
        Some(self.uncons_tree().ok()?.1)
    }
    pub fn lookup(&self, i: usize) -> Option<T> {
        match &*self.0 {
            Cell::Nil => None,
            Cell::Cons(Digit::Zero, ts) => Self(ts.clone()).lookup(i),
            Cell::Cons(Digit::One(t), ts) => {
                if i < t.size() {
                    t.lookup(i)
                } else {
                    Self(ts.clone()).lookup(i - t.size())
                }
            }
        }
    }
    pub fn update(&self, i: usize, y: T) -> Result<Self> {
        match &*self.0 {
            Cell::Nil => bail!("invalid index"),
            Cell::Cons(Digit::Zero, ts) => Ok(Self(List::cons(
                Digit::Zero,
                Self(ts.clone()).update(i, y)?.0,
            ))),
            Cell::Cons(Digit::One(t), ts) => Ok(Self(if i < t.size() {
                List::cons(Digit::One(t.update(i, y)?), ts.clone())
            } else {
                List::cons(
                    Digit::One(t.clone()),
                    Self(ts.clone()).update(i - t.size(), y)?.0,
                )
            })),
        }
    }
}

impl<T: Clone> ATree<T> {
    fn size(&self) -> usize {
        match self.0.as_ref() {
            Tree::Leaf(_) => 1,
            Tree::Node(n, ..) => *n,
        }
    }
    fn link(&self, other: &Self) -> Self {
        Self(Arc::new(Tree::Node(
            self.size() + other.size(),
            self.clone(),
            other.clone(),
        )))
    }
    fn lookup(&self, i: usize) -> Option<T> {
        match (self.0.as_ref(), i) {
            (Tree::Leaf(x), 0) => Some(x.clone()),
            (Tree::Leaf(_), _) => None,
            (Tree::Node(w, t1, t2), i) => {
                if i < w / 2 {
                    t1.lookup(i)
                } else {
                    t2.lookup(i - w / 2)
                }
            }
        }
    }
    fn update(&self, i: usize, y: T) -> Result<Self> {
        match (self.0.as_ref(), i) {
            (Tree::Leaf(_), 0) => Ok(Self(Arc::new(Tree::Leaf(y)))),
            (Tree::Leaf(_), _) => bail!("invalid index"),
            (Tree::Node(w, t1, t2), i) => Ok(Self(Arc::new(if i < w / 2 {
                Tree::Node(*w, t1.update(i, y)?, t2.clone())
            } else {
                Tree::Node(*w, t1.clone(), t2.update(i - w / 2, y)?)
            }))),
        }
    }
}

impl_ralist!(BinaryRandomAccessList);

#[cfg(test)]
mod tests {
    use super::BinaryRandomAccessList as RAList;

    #[test]
    fn test() {
        let xs = [5, 2, 7];
        let r = xs
            .iter()
            .rev()
            .fold(RAList::empty(), |l, &x| RAList::cons(x, &l));

        assert_eq!(r.head(), Some(5));
        assert_eq!(r.lookup(0), Some(5));
        assert_eq!(r.lookup(1), Some(2));
        assert_eq!(r.lookup(2), Some(7));
        assert_eq!(r.lookup(3), None);

        let r = r.tail().unwrap();
        assert_eq!(r.head(), Some(2));
        assert_eq!(r.lookup(0), Some(2));
        assert_eq!(r.lookup(1), Some(7));
        assert_eq!(r.lookup(2), None);

        let r = r.update(1, 4).unwrap();
        assert_eq!(r.head(), Some(2));
        assert_eq!(r.lookup(0), Some(2));
        assert_eq!(r.lookup(1), Some(4));
        assert_eq!(r.lookup(2), None);

        assert!(r.update(3, 0).is_err());

        let r = RAList::cons(8, &r);
        assert_eq!(r.head(), Some(8));
        assert_eq!(r.lookup(0), Some(8));
        assert_eq!(r.lookup(1), Some(2));
        assert_eq!(r.lookup(2), Some(4));
        assert_eq!(r.lookup(3), None);
    }
}
