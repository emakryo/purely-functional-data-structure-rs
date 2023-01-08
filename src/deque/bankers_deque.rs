use crate::{
    lazy,
    lazy::{Stream, StreamCell},
};

use super::impl_deque;

#[derive(Clone)]
pub struct BankersDeque<T> {
    lenf: usize,
    f: Stream<T>,
    lenr: usize,
    r: Stream<T>,
}

impl<T: Clone + 'static> BankersDeque<T> {
    pub fn empty() -> Self {
        Self {
            lenf: 0,
            f: Stream::empty(),
            lenr: 0,
            r: Stream::empty(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.lenf + self.lenr == 0
    }
    fn check(self) -> Self {
        let c = 2;
        if self.lenf > c * self.lenr + 1 {
            let i = (self.lenf + self.lenr) / 2;
            let j = self.lenf + self.lenr - i;
            let f = self.f.take(i);
            let r = self.r.extend(&self.f.drop(i).reverse());
            Self {
                lenf: i,
                f,
                lenr: j,
                r,
            }
        } else if self.lenr > c * self.lenf + 1 {
            let j = (self.lenf + self.lenr) / 2;
            let i = self.lenf + self.lenr - j;
            let r = self.r.take(j);
            let f = self.f.extend(&self.r.drop(j).reverse());
            Self {
                lenf: i,
                f,
                lenr: j,
                r,
            }
        } else {
            self
        }
    }
    pub fn cons(x: T, xs: &Self) -> Self {
        let f = xs.f.clone();
        Self {
            lenf: xs.lenf + 1,
            f: Stream::cons(lazy!((x, f))),
            lenr: xs.lenr,
            r: xs.r.clone(),
        }
        .check()
    }
    pub fn head(&self) -> Option<T> {
        match (self.f.force(), self.r.force()) {
            (StreamCell::Nil, StreamCell::Nil) => None,
            (StreamCell::Nil, StreamCell::Cons(x, _)) => Some(x),
            (StreamCell::Cons(x, _), _) => Some(x),
        }
    }
    pub fn tail(&self) -> Option<Self> {
        match (self.f.force(), self.r.force()) {
            (StreamCell::Nil, StreamCell::Nil) => None,
            (StreamCell::Nil, StreamCell::Cons(..)) => Some(Self::empty()),
            (StreamCell::Cons(_, f), _) => Some(
                Self {
                    lenf: self.lenf - 1,
                    f,
                    lenr: self.lenr,
                    r: self.r.clone(),
                }
                .check(),
            ),
        }
    }
    pub fn snoc(&self, x: T) -> Self {
        let r = self.r.clone();
        Self {
            lenf: self.lenf,
            f: self.f.clone(),
            lenr: self.lenr + 1,
            r: Stream::cons(lazy!((x, r))),
        }
        .check()
    }
    pub fn last(&self) -> Option<T> {
        match (self.f.force(), self.r.force()) {
            (StreamCell::Nil, StreamCell::Nil) => None,
            (StreamCell::Cons(x, _), StreamCell::Nil) => Some(x),
            (_, StreamCell::Cons(x, _)) => Some(x),
        }
    }
    pub fn init(&self) -> Option<Self> {
        match (self.f.force(), self.r.force()) {
            (StreamCell::Nil, StreamCell::Nil) => None,
            (StreamCell::Cons(..), StreamCell::Nil) => Some(Self::empty()),
            (_, StreamCell::Cons(_, r)) => Some(
                Self {
                    lenf: self.lenf,
                    f: self.f.clone(),
                    lenr: self.lenr - 1,
                    r,
                }
                .check(),
            ),
        }
    }
}

impl_deque!(BankersDeque);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let deq = BankersDeque::empty();
        let deq = BankersDeque::cons(5, &deq); // [5]
        assert_eq!(deq.head(), Some(5));
        assert_eq!(deq.last(), Some(5));

        let deq = BankersDeque::cons(7, &deq); // [7, 5]
        assert_eq!(deq.head(), Some(7));
        assert_eq!(deq.last(), Some(5));

        let deq = BankersDeque::snoc(&deq, 2); // [7, 5, 2]
        assert_eq!(deq.head(), Some(7));
        assert_eq!(deq.last(), Some(2));

        let deq = deq.tail().unwrap(); // [5, 2]
        assert_eq!(deq.head(), Some(5));
        assert_eq!(deq.last(), Some(2));

        let deq = BankersDeque::cons(4, &deq); // [4, 5, 2]
        assert_eq!(deq.head(), Some(4));
        assert_eq!(deq.last(), Some(2));

        let deq = deq.init().unwrap(); // [4, 5]
        assert_eq!(deq.head(), Some(4));
        assert_eq!(deq.last(), Some(5));
    }
}
