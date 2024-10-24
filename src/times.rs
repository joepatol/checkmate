use crate::core::{Times, Checked, CheckState};

fn count_valid_checks<T, S: CheckState<T>>(checks: impl Iterator<Item = S>) -> usize {
    let mut count = 0;
    for check in checks {
        match check.get_state_as_ref() {
            Checked::Valid { value: _ } => count += 1,
            Checked::Invalid { value: _, message: _ } => continue,
        }
    };
    return count;
}

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

impl<T> Times<T> for AtLeast
where
    T: IntoIterator
{
    fn check<S: CheckState<T::Item>>(self, inner: T, message: String, checks: impl Iterator<Item = S>) -> Checked<T> {
        if count_valid_checks(checks) >= self.0 {
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

impl<T> Times<T> for AtMost
where
    T: IntoIterator
{
    fn check<S: CheckState<T::Item>>(self, inner: T, message: String, checks: impl Iterator<Item = S>) -> Checked<T> {
        if count_valid_checks(checks) <= self.0 {
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

impl<T> Times<T> for Exactly
where
    T: IntoIterator
{
    fn check<S: CheckState<T::Item>>(self, inner: T, message: String, checks: impl Iterator<Item = S>) -> Checked<T> {
        if count_valid_checks(checks) >= self.0 {
            Checked::valid(inner)
        } else {
            Checked::invalid(inner, format!("{message} exactly {} times", self.0))
        }
    }
}