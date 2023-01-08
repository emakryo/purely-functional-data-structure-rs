use crate::{
    lazy,
    lazy::{Stream, StreamCell},
};

use super::impl_deque;

#[derive(Clone)]
pub struct RealTimeDeque<T> {
    lenf: usize,
    f: Stream<T>,
    sf: Stream<T>,
    lenr: usize,
    r: Stream<T>,
    sr: Stream<T>,
}

impl<T: Clone + 'static> RealTimeDeque<T> {
    fn new(
        lenf: usize,
        f: Stream<T>,
        sf: Stream<T>,
        lenr: usize,
        r: Stream<T>,
        sr: Stream<T>,
    ) -> Self {
        Self {
            lenf,
            f,
            sf,
            lenr,
            r,
            sr,
        }
    }
    pub fn empty() -> Self {
        Self {
            lenf: 0,
            f: Stream::empty(),
            sf: Stream::empty(),
            lenr: 0,
            r: Stream::empty(),
            sr: Stream::empty(),
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
            let r = rotate_drop(self.r, i, self.f, c);
            Self::new(i, f.clone(), f, j, r.clone(), r)
        } else if self.lenr > c * self.lenf + 1 {
            let j = (self.lenf + self.lenr) / 2;
            let i = self.lenf + self.lenr - j;
            let r = self.r.take(j);
            let f = rotate_drop(self.f, j, self.r, c);
            Self::new(i, f.clone(), f, j, r.clone(), r)
        } else {
            self
        }
    }
    pub fn cons(x: T, xs: &Self) -> Self {
        let f = xs.f.clone();
        Self::new(
            xs.lenf + 1,
            Stream::cons(lazy!((x, f))),
            exec1(&xs.sf),
            xs.lenr,
            xs.r.clone(),
            exec1(&xs.sr),
        )
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
                Self::new(
                    self.lenf - 1,
                    f,
                    exec2(&self.sf),
                    self.lenr,
                    self.r.clone(),
                    exec2(&self.sr),
                )
                .check(),
            ),
        }
    }
    pub fn snoc(&self, x: T) -> Self {
        let r = self.r.clone();
        Self::new(
            self.lenf,
            self.f.clone(),
            exec1(&self.sf),
            self.lenr + 1,
            Stream::cons(lazy!((x, r))),
            exec1(&self.sr),
        )
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
                Self::new(
                    self.lenf,
                    self.f.clone(),
                    exec2(&self.sf),
                    self.lenr - 1,
                    r,
                    exec2(&self.sr),
                )
                .check(),
            ),
        }
    }
}

impl_deque!(RealTimeDeque);

fn exec1<T: Clone + 'static>(xs: &Stream<T>) -> Stream<T> {
    match xs.force() {
        StreamCell::Cons(_, xs) => xs,
        _ => xs.clone(),
    }
}

fn exec2<T: Clone + 'static>(xs: &Stream<T>) -> Stream<T> {
    exec1(&exec1(xs))
}

fn rotate_rev<T: Clone + 'static>(f: Stream<T>, r: Stream<T>, a: Stream<T>, c: usize) -> Stream<T> {
    match f.force() {
        StreamCell::Nil => r.reverse().extend(&a),
        StreamCell::Cons(x, f) => Stream::cons(lazy!((
            x,
            rotate_rev(f, r.drop(c), r.take(c).reverse().extend(&a), c)
        ))),
    }
}

fn rotate_drop<T: Clone + 'static>(f: Stream<T>, j: usize, r: Stream<T>, c: usize) -> Stream<T> {
    if j < c {
        rotate_rev(f, r.drop(j), Stream::empty(), c)
    } else {
        match f.force() {
            StreamCell::Cons(x, f) => Stream::cons(lazy!((x, rotate_drop(f, j - c, r.drop(c), c)))),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let deq = RealTimeDeque::empty();
        let deq = RealTimeDeque::cons(5, &deq); // [5]
        assert_eq!(deq.head(), Some(5));
        assert_eq!(deq.last(), Some(5));

        let deq = RealTimeDeque::cons(7, &deq); // [7, 5]
        assert_eq!(deq.head(), Some(7));
        assert_eq!(deq.last(), Some(5));

        let deq = RealTimeDeque::snoc(&deq, 2); // [7, 5, 2]
        assert_eq!(deq.head(), Some(7));
        assert_eq!(deq.last(), Some(2));

        let deq = deq.tail().unwrap(); // [5, 2]
        assert_eq!(deq.head(), Some(5));
        assert_eq!(deq.last(), Some(2));

        let deq = RealTimeDeque::cons(4, &deq); // [4, 5, 2]
        assert_eq!(deq.head(), Some(4));
        assert_eq!(deq.last(), Some(2));

        let deq = deq.init().unwrap(); // [4, 5]
        assert_eq!(deq.head(), Some(4));
        assert_eq!(deq.last(), Some(5));
    }
}
