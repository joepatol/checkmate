use crate::core::{Times, Checked};

pub struct AtLeast(usize);

impl AtLeast {
    pub fn once() -> Self {
        Self::times(1)
    }

    pub fn twice() -> Self {
        Self::times(2)
    }

    pub fn times(times: usize) -> Self {
        Self { 0: times }
    }
}

impl<T> Times<T> for AtLeast{
    fn do_count(self, inner: T, message: String, checks: impl IntoIterator<Item = bool>) -> Checked<T> {
        let mut count = 0;
        for check in checks.into_iter() {
            if check {
                count += 1;
            }
        };
        if count >= self.0 {
            Checked::valid(inner)
        } else {
            Checked::invalid(inner, format!("{message} at least {} times", self.0))
        }
    }
}

pub struct AtMost(usize);

impl AtMost {
    pub fn once() -> Self {
        Self::times(1)
    }

    pub fn twice() -> Self {
        Self::times(2)
    }

    pub fn times(times: usize) -> Self {
        Self { 0: times }
    }
}

impl<T> Times<T> for AtMost{
    fn do_count(self, inner: T, message: String, checks: impl IntoIterator<Item = bool>) -> Checked<T> {
        let mut count = 0;
        for check in checks.into_iter() {
            if check {
                count += 1;
            }
        };
        if count <= self.0 {
            Checked::valid(inner)
        } else {
            Checked::invalid(inner, format!("{message} at most {} times", self.0))
        }
    }
}

pub struct Exactly(usize);

impl Exactly {
    pub fn once() -> Self {
        Self::times(1)
    }

    pub fn twice() -> Self {
        Self::times(2)
    }

    pub fn times(times: usize) -> Self {
        Self { 0: times }
    }
}

impl<T> Times<T> for Exactly{
    fn do_count(self, inner: T, message: String, checks: impl IntoIterator<Item = bool>) -> Checked<T> {
        let mut count = 0;
        for check in checks.into_iter() {
            if check {
                count += 1;
            }
        };
        if count == self.0 {
            Checked::valid(inner)
        } else {
            Checked::invalid(inner, format!("{message} exactly {} times", self.0))
        }
    }
}