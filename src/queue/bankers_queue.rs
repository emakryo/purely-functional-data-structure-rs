use crate::{lazy, lazy::Stream};

#[derive(Clone)]
pub struct BankersQueue<T> {
    lenf: usize,
    f: Stream<T>,
    lenr: usize,
    r: Stream<T>,
}

impl<T: Clone + 'static> BankersQueue<T> {
    pub fn empty() -> Self {
        Self {
            lenf: 0,
            f: Stream::empty(),
            lenr: 0,
            r: Stream::empty(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.lenf == 0
    }
    fn check(self) -> Self {
        if self.lenr <= self.lenf {
            self
        } else {
            Self {
                lenf: self.lenf + self.lenr,
                f: self.f.extend(&self.r.reverse()),
                lenr: 0,
                r: Stream::empty(),
            }
        }
    }
    pub fn snoc(&self, x: T) -> Self {
        let this = self.clone();
        Self {
            lenf: self.lenf,
            f: self.f.clone(),
            lenr: self.lenr + 1,
            r: Stream::cons(lazy!((x, this.r))),
        }
        .check()
    }
    pub fn head(&self) -> Option<T> {
        self.f.head().force()
    }
    pub fn tail(&self) -> Option<Self> {
        Some(
            Self {
                lenf: self.lenf - 1,
                f: self.f.tail(),
                lenr: self.lenr,
                r: self.r.clone(),
            }
            .check(),
        )
    }
}

crate::queue::impl_queue!(BankersQueue);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let xs = [1, 3, 2, 0, 4];
        let q = xs.iter().fold(BankersQueue::empty(), |q, &x| q.snoc(x));

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
