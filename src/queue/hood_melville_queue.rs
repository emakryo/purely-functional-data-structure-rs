use crate::{stack::Cell, List};

use super::impl_queue;

#[derive(Clone)]
pub struct HoodMelvilleQueue<T> {
    lenf: usize,
    f: List<T>,
    lenr: usize,
    r: List<T>,
    state: RotationState<T>,
}

#[derive(Clone)]
enum RotationState<T> {
    Idle,
    Reversing {
        ok: usize,
        f: List<T>,
        fp: List<T>,
        r: List<T>,
        rp: List<T>,
    },
    Appending {
        ok: usize,
        f: List<T>,
        r: List<T>,
    },
    Done {
        r: List<T>,
    },
}

impl<T: Clone> RotationState<T> {
    fn exec(self) -> Self {
        match self.clone() {
            Self::Reversing { ok, f, fp, r, rp } => match (&*f, &*r) {
                (Cell::Cons(x, f), Cell::Cons(y, r)) => Self::Reversing {
                    ok: ok + 1,
                    f: f.clone(),
                    fp: List::cons(x.clone(), fp),
                    r: r.clone(),
                    rp: List::cons(y.clone(), rp),
                },
                (Cell::Nil, Cell::Cons(y, r)) if r.is_empty() => Self::Appending {
                    ok,
                    f: fp,
                    r: List::cons(y.clone(), rp),
                },
                _ => self,
            },
            Self::Appending { ok: 0, f: _, r } => Self::Done { r },
            Self::Appending { ok, f, r } => match &*f {
                Cell::Cons(x, f) => Self::Appending {
                    ok: ok - 1,
                    f: f.clone(),
                    r: List::cons(x.clone(), r),
                },
                _ => self,
            },
            _ => self,
        }
    }

    fn invalidate(self) -> Self {
        match self.clone() {
            Self::Reversing { ok, f, fp, r, rp } => Self::Reversing {
                ok: ok - 1,
                f,
                fp,
                r,
                rp,
            },
            Self::Appending { ok: 0, f: _, r } if !r.is_empty() => Self::Done {
                r: r.tail().unwrap(),
            },
            Self::Appending { ok, f, r } => Self::Appending { ok: ok - 1, f, r },
            _ => self,
        }
    }
}

impl<T: Clone> HoodMelvilleQueue<T> {
    fn exec2(self) -> Self {
        match self.state.exec().exec() {
            RotationState::Done { r } => Self {
                lenf: self.lenf,
                f: r,
                lenr: self.lenr,
                r: self.r,
                state: RotationState::Idle,
            },
            state => Self {
                lenf: self.lenf,
                f: self.f,
                lenr: self.lenr,
                r: self.r,
                state,
            },
        }
    }
    fn check(self) -> Self {
        if self.lenr <= self.lenf {
            self.exec2()
        } else {
            let state = RotationState::Reversing {
                ok: 0,
                f: self.f.clone(),
                fp: List::empty(),
                r: self.r,
                rp: List::empty(),
            };
            Self {
                lenf: self.lenf + self.lenr,
                f: self.f,
                lenr: 0,
                r: List::empty(),
                state,
            }
            .exec2()
        }
    }
    pub fn empty() -> Self {
        Self {
            lenf: 0,
            f: List::empty(),
            lenr: 0,
            r: List::empty(),
            state: RotationState::Idle,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.lenf == 0
    }
    pub fn snoc(&self, x: T) -> Self {
        Self {
            lenf: self.lenf,
            f: self.f.clone(),
            lenr: self.lenr + 1,
            r: List::cons(x, self.r.clone()),
            state: self.state.clone(),
        }
        .check()
    }
    pub fn head(&self) -> Option<T> {
        self.f.head()
    }
    pub fn tail(&self) -> Option<Self> {
        Some(
            Self {
                lenf: self.lenf - 1,
                f: self.f.tail()?,
                lenr: self.lenr,
                r: self.r.clone(),
                state: self.state.clone().invalidate(),
            }
            .check(),
        )
    }
}

impl_queue!(HoodMelvilleQueue);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let xs = [1, 3, 2, 0, 4];
        let q = xs
            .iter()
            .fold(HoodMelvilleQueue::empty(), |q, &x| q.snoc(x));

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
