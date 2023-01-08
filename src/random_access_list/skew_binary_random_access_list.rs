use std::sync::Arc;

use anyhow::{bail, Result};

use crate::{stack::Cell, List};

use super::impl_ralist;

#[derive(Clone)]
pub struct SkewBinaryRandomAccessList<T>(List<(usize, ATree<T>)>);

#[derive(Clone)]
struct ATree<T>(Arc<Tree<T>>);

#[derive(Clone)]
enum Tree<T> {
    Leaf(T),
    Node(T, ATree<T>, ATree<T>),
}

impl<T: Clone> SkewBinaryRandomAccessList<T> {
    pub fn empty() -> Self {
        Self(List::empty())
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn cons(x: T, xs: &Self) -> Self {
        match &*xs.0 {
            Cell::Cons((w1, t1), ts) => match &**ts {
                Cell::Cons((w2, t2), ts) if w1 == w2 => Self(List::cons(
                    (1 + w1 + w2, ATree::node(x, t1.clone(), t2.clone())),
                    ts.clone(),
                )),
                _ => Self(List::cons((1, ATree::leaf(x)), xs.0.clone())),
            },
            _ => Self(List::cons((1, ATree::leaf(x)), xs.0.clone())),
        }
    }
    pub fn head(&self) -> Option<T> {
        match &*self.0 {
            Cell::Nil => None,
            Cell::Cons((1, t), _) => match t.0.as_ref() {
                Tree::Leaf(x) => Some(x.clone()),
                _ => unreachable!(),
            },
            Cell::Cons((_, t), _) => match t.0.as_ref() {
                Tree::Node(x, ..) => Some(x.clone()),
                _ => unreachable!(),
            },
        }
    }
    pub fn tail(&self) -> Option<Self> {
        match &*self.0 {
            Cell::Nil => None,
            Cell::Cons((1, _), ts) => Some(Self(ts.clone())),
            Cell::Cons((w, t), ts) => match t.0.as_ref() {
                Tree::Node(_, t1, t2) => Some(Self(List::cons(
                    (w / 2, t1.clone()),
                    List::cons((w / 2, t2.clone()), ts.clone()),
                ))),
                _ => unreachable!(),
            },
        }
    }
    pub fn lookup(&self, i: usize) -> Option<T> {
        match &*self.0 {
            Cell::Nil => None,
            Cell::Cons((w, t), ts) => {
                if i < *w {
                    t.lookup(*w, i)
                } else {
                    Self(ts.clone()).lookup(i - w)
                }
            }
        }
    }
    pub fn update(&self, i: usize, y: T) -> Result<Self> {
        match &*self.0 {
            Cell::Nil => bail!("invalid index"),
            Cell::Cons((w, t), ts) => Ok(Self(if i < *w {
                List::cons((*w, t.update(*w, i, y)?), ts.clone())
            } else {
                List::cons((*w, t.clone()), Self(ts.clone()).update(i - *w, y)?.0)
            })),
        }
    }
}

impl<T: Clone> ATree<T> {
    fn leaf(x: T) -> Self {
        Self(Arc::new(Tree::Leaf(x)))
    }
    fn node(x: T, t1: ATree<T>, t2: ATree<T>) -> Self {
        Self(Arc::new(Tree::Node(x, t1, t2)))
    }
    fn lookup(&self, w: usize, i: usize) -> Option<T> {
        match (w, i, self.0.as_ref()) {
            (1, 0, Tree::Leaf(x)) => Some(x.clone()),
            (1, _, _) => None,
            (_, 0, Tree::Node(x, ..)) => Some(x.clone()),
            (w, i, Tree::Node(_, t1, t2)) => {
                if i <= w / 2 {
                    t1.lookup(w / 2, i - 1)
                } else {
                    t2.lookup(w / 2, i - 1 - w / 2)
                }
            }
            _ => unreachable!(),
        }
    }
    fn update(&self, w: usize, i: usize, y: T) -> Result<Self> {
        match (w, i, self.0.as_ref()) {
            (1, 0, Tree::Leaf(_)) => Ok(Self::leaf(y)),
            (1, _, Tree::Leaf(_)) => bail!("invalid index"),
            (_, 0, Tree::Node(_, t1, t2)) => Ok(Self::node(y, t1.clone(), t2.clone())),
            (w, i, Tree::Node(x, t1, t2)) => {
                if i <= w / 2 {
                    Ok(Self::node(
                        x.clone(),
                        t1.update(w / 2, i - 1, y)?,
                        t2.clone(),
                    ))
                } else {
                    Ok(Self::node(
                        x.clone(),
                        t1.clone(),
                        t2.update(w / 2, i - 1 - w / 2, y)?,
                    ))
                }
            }
            _ => unreachable!(),
        }
    }
}

impl_ralist!(SkewBinaryRandomAccessList);

#[cfg(test)]
mod tests {
    use super::SkewBinaryRandomAccessList as RAList;

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
