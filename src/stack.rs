use anyhow::{bail, Result};
use std::{ops::Deref, sync::Arc};

use crate::{lazy, lazy::Stream};

#[derive(Clone)]
pub struct Stack<T>(Arc<Cell<T>>);

pub enum Cell<T> {
    Nil,
    Cons(T, Stack<T>),
}

impl<T> Deref for Stack<T> {
    type Target = Cell<T>;
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl<T> From<Cell<T>> for Stack<T> {
    fn from(x: Cell<T>) -> Self {
        Self(Arc::new(x))
    }
}

impl<T: Clone> Stack<T> {
    pub fn empty() -> Self {
        Cell::Nil.into()
    }
    pub fn is_empty(&self) -> bool {
        matches!(self.0.as_ref(), Cell::Nil)
    }
    pub fn cons(head: T, tail: Self) -> Self {
        Cell::Cons(head, tail).into()
    }
    pub fn head(&self) -> Option<T> {
        match self.0.as_ref() {
            Cell::Nil => None,
            Cell::Cons(x, _) => Some(x.clone()),
        }
    }
    pub fn tail(&self) -> Option<Self> {
        match self.0.as_ref() {
            Cell::Nil => None,
            Cell::Cons(_, xs) => Some(xs.clone()),
        }
    }
    pub fn update(&self, idx: usize, new: T) -> Result<Self> {
        match (self.0.as_ref(), idx) {
            (Cell::Nil, _) => bail!("Invalid index"),
            (Cell::Cons(_, tail), 0) => Ok(Self::cons(new, tail.clone())),
            (Cell::Cons(head, tail), idx) => {
                Ok(Self::cons(head.clone(), tail.update(idx - 1, new)?))
            }
        }
    }
    /// Exercise 2.1
    pub fn suffixes(&self) -> Stack<Self> {
        match &**self {
            Cell::Nil => Stack::cons(Self::empty(), Stack::empty()),
            Cell::Cons(_, tail) => Stack::cons(self.clone(), tail.clone().suffixes()),
        }
    }
    pub fn extend(&self, other: Self) -> Self {
        if self.is_empty() {
            other
        } else {
            Self::cons(self.head().unwrap(), self.tail().unwrap().extend(other))
        }
    }
    pub fn map<U: Clone, F: Fn(T) -> U>(&self, f: F) -> Stack<U> {
        if self.is_empty() {
            Stack::empty()
        } else {
            Stack::cons(f(self.head().unwrap()), self.tail().unwrap().map(f))
        }
    }
    pub fn reverse(&self) -> Self {
        self.reverse_rec(Self::empty())
    }
    fn reverse_rec(&self, dest: Self) -> Self {
        match self.0.as_ref() {
            Cell::Nil => dest,
            Cell::Cons(x, xs) => xs.reverse_rec(Self::cons(x.clone(), dest)),
        }
    }
    pub fn take(&self, n: usize) -> Self {
        match (self.0.as_ref(), n) {
            (_, 0) => Self::empty(),
            (Cell::Nil, _) => Self::empty(),
            (Cell::Cons(x, xs), n) => Self::cons(x.clone(), xs.take(n - 1)),
        }
    }
    pub fn drop(&self, n: usize) -> Self {
        match (self.0.as_ref(), n) {
            (_, 0) => self.clone(),
            (Cell::Nil, _) => Self::empty(),
            (Cell::Cons(_, xs), n) => xs.drop(n - 1),
        }
    }
    pub fn size(&self) -> usize {
        fn s<T>(xs: &Stack<T>, n: usize) -> usize {
            match xs.0.as_ref() {
                Cell::Nil => n,
                Cell::Cons(_, xs) => s(xs, n + 1),
            }
        }

        s(self, 0)
    }
}

impl<T: Clone + 'static> Stack<T> {
    pub fn to_stream(&self) -> Stream<T> {
        match &*self.0 {
            Cell::Nil => Stream::empty(),
            Cell::Cons(x, xs) => {
                let x = x.clone();
                let xs = xs.clone();
                Stream::cons(lazy!((x, xs.to_stream())))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let s = Stack::<()>::empty();
        assert!(s.is_empty());
    }

    #[test]
    fn test_cons() {
        let x = 10;
        let s = Stack::<i64>::empty();
        let t = Stack::cons(x, s);

        assert_eq!(x, t.head().unwrap());
        assert!(t.tail().unwrap().is_empty());
    }

    #[test]
    fn test_update() {
        let mut s = Stack::<i64>::empty();
        for x in [10, 20, 30] {
            s = Stack::cons(x, s);
        }

        assert!(s.update(5, 40).is_err());
        assert_eq!(
            s.update(2, 50)
                .unwrap()
                .tail()
                .unwrap()
                .tail()
                .unwrap()
                .head()
                .unwrap(),
            50
        );
    }
}
