use crate::{lazy, lazy::Stream, lazy::StreamCell, List};

use super::impl_queue;

#[derive(Clone)]
pub struct RealTimeQueue<T>(Stream<T>, List<T>, Stream<T>);

impl<T: Clone + 'static> RealTimeQueue<T> {
    pub fn empty() -> Self {
        Self(Stream::empty(), List::empty(), Stream::empty())
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty().force()
    }
    fn rotate(&self) -> Stream<T> {
        let this = self.clone();
        match self.0.force() {
            StreamCell::Nil => Stream::cons(lazy!((this.1.head().unwrap(), this.2))),
            StreamCell::Cons(x, xs) => Stream::cons(lazy!((
                x,
                Self(
                    xs,
                    this.1.tail().unwrap(),
                    Stream::cons(lazy!((this.1.head().unwrap(), this.2))),
                )
                .rotate(),
            ))),
        }
    }
    fn exec(&self) -> Self {
        match self.2.force() {
            StreamCell::Nil => {
                let f = self.rotate();
                Self(f.clone(), List::empty(), f)
            }
            StreamCell::Cons(_, s) => Self(self.0.clone(), self.1.clone(), s),
        }
    }
    pub fn snoc(&self, x: T) -> Self {
        Self(
            self.0.clone(),
            List::cons(x, self.1.clone()),
            self.2.clone(),
        )
        .exec()
    }
    pub fn head(&self) -> Option<T> {
        match self.0.force() {
            StreamCell::Nil => None,
            StreamCell::Cons(x, _) => Some(x),
        }
    }
    pub fn tail(&self) -> Option<Self> {
        match self.0.force() {
            StreamCell::Nil => None,
            StreamCell::Cons(_, xs) => Some(Self(xs, self.1.clone(), self.2.clone()).exec()),
        }
    }
}

impl_queue!(RealTimeQueue);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let xs = [1, 3, 2, 0, 4];
        let q = xs.iter().fold(RealTimeQueue::empty(), |q, &x| q.snoc(x));

        assert_eq!(q.head(), Some(1));
        let q = q.tail().unwrap();
        assert_eq!(q.head(), Some(3));
        let q = q.tail().unwrap();
        assert_eq!(q.head(), Some(2));
        let q = q.tail().unwrap();
        assert_eq!(q.head(), Some(0));
        let q = q.tail().unwrap();
        assert_eq!(q.head(), Some(4));
        let q = q.tail().unwrap();
        assert!(q.is_empty());
    }
}
