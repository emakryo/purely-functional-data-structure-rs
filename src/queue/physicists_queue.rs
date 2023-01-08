use crate::{lazy, lazy::Lazy, List};

use super::impl_queue;

pub struct PhysicistsQueue<T> {
    w: List<T>,
    lenf: usize,
    f: Lazy<List<T>>,
    lenr: usize,
    r: List<T>,
}

impl<T: Clone + 'static> PhysicistsQueue<T> {
    pub fn empty() -> Self {
        Self {
            w: List::empty(),
            lenf: 0,
            f: lazy!(List::empty()),
            lenr: 0,
            r: List::empty(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.lenf == 0
    }
    fn checkw(self) -> Self {
        if self.w.is_empty() {
            Self {
                w: self.f.force(),
                lenf: self.lenf,
                f: self.f.clone(),
                lenr: self.lenr,
                r: self.r,
            }
        } else {
            self
        }
    }
    fn check(self) -> Self {
        if self.lenr <= self.lenf {
            self.checkw()
        } else {
            let fp = self.f.force();
            Self {
                w: fp.clone(),
                lenf: self.lenf + self.lenr,
                f: lazy!(fp.extend(self.r.reverse())),
                lenr: 0,
                r: List::empty(),
            }
            .checkw()
        }
    }
    pub fn snoc(&self, x: T) -> Self {
        Self {
            w: self.w.clone(),
            lenf: self.lenf,
            f: self.f.clone(),
            lenr: self.lenr + 1,
            r: List::cons(x, self.r.clone()),
        }
        .check()
    }
    pub fn head(&self) -> Option<T> {
        self.w.head()
    }
    pub fn tail(&self) -> Option<Self> {
        let f = self.f.clone();
        Some(
            Self {
                w: self.w.tail()?,
                lenf: self.lenf - 1,
                f: lazy!(f.force().tail().unwrap()),
                lenr: self.lenr,
                r: self.r.clone(),
            }
            .check(),
        )
    }
}

impl_queue!(PhysicistsQueue);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let xs = [1, 3, 2, 0, 4];
        let q = xs.iter().fold(PhysicistsQueue::empty(), |q, &x| q.snoc(x));

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
