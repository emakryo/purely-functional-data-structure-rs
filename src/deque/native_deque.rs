use crate::List;

use super::impl_deque;

#[derive(Clone)]
pub struct NaiveDeque<T>(List<T>, List<T>, usize, usize);

impl<T: Clone> NaiveDeque<T> {
    pub fn empty() -> Self {
        let e = List::empty();
        Self(e.clone(), e, 0, 0)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn check(self) -> Self {
        if self.2 == 0 {
            Self(
                self.1.drop(self.3 / 2).reverse(),
                self.1.take(self.3 / 2),
                (self.3 + 1) / 2,
                self.3 / 2,
            )
        } else if self.3 == 0 {
            Self(
                self.0.take(self.2 / 2),
                self.0.drop(self.2 / 2).reverse(),
                self.2 / 2,
                (self.2 + 1) / 2,
            )
        } else {
            self
        }
    }

    pub fn cons(head: T, tail: &Self) -> Self {
        Self(
            List::cons(head, tail.0.clone()),
            tail.1.clone(),
            tail.2 + 1,
            tail.3,
        )
        .check()
    }

    pub fn head(&self) -> Option<T> {
        if self.0.is_empty() {
            self.1.head()
        } else {
            self.0.head()
        }
    }

    pub fn tail(&self) -> Option<Self> {
        Some(Self(self.0.clone().tail()?, self.1.clone(), self.2 - 1, self.3).check())
    }

    pub fn snoc(init: &Self, last: T) -> Self {
        Self(
            init.0.clone(),
            List::cons(last, init.1.clone()),
            init.2,
            init.3 + 1,
        )
        .check()
    }

    pub fn last(&self) -> Option<T> {
        if self.1.is_empty() {
            self.0.head()
        } else {
            self.1.head()
        }
    }

    pub fn init(&self) -> Option<Self> {
        if self.1.is_empty() {
            self.tail()
        } else {
            Some(Self(self.0.clone(), self.1.tail()?, self.2, self.3 - 1).check())
        }
    }
}

impl_deque!(NaiveDeque);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let deq = NaiveDeque::empty();
        let deq = NaiveDeque::cons(5, &deq); // [5]
        assert_eq!(deq.head(), Some(5));
        assert_eq!(deq.last(), Some(5));

        let deq = NaiveDeque::cons(7, &deq); // [7, 5]
        assert_eq!(deq.head(), Some(7));
        assert_eq!(deq.last(), Some(5));

        let deq = NaiveDeque::snoc(&deq, 2); // [7, 5, 2]
        assert_eq!(deq.head(), Some(7));
        assert_eq!(deq.last(), Some(2));

        let deq = deq.tail().unwrap(); // [5, 2]
        assert_eq!(deq.head(), Some(5));
        assert_eq!(deq.last(), Some(2));

        let deq = NaiveDeque::cons(4, &deq); // [4, 5, 2]
        assert_eq!(deq.head(), Some(4));
        assert_eq!(deq.last(), Some(2));

        let deq = deq.init().unwrap(); // [4, 5]
        assert_eq!(deq.head(), Some(4));
        assert_eq!(deq.last(), Some(5));
    }
}
