use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;

pub trait CheckState<T> {
    fn get_state_as_ref(&self) -> &Checked<T>;
    fn get_state(self) -> Checked<T>;
    fn propagate(self, checked: Checked<T>) -> Self;
    fn propagate_predicate(self, check: impl FnOnce(T) -> Checked<T>) -> Self;
}

pub trait Assertions<T> {
    fn assert_valid(&self);
    fn assert_invalid(&self);
    fn assert_invalid_with_message(&self, err_message: &str);
}

pub trait CheckMateEntrypoint<T> {
    fn value(self) -> Checked<T>;
    fn value_as_ref(&self) -> Checked<&T>;
    fn value_rc(self) -> Checked<Rc<T>>;
    fn value_cloned(&self) -> Checked<T> where T: Clone + Deref;
}

pub trait Directive<T, C: CheckState<T>> {
    fn should(self) -> Should<T, C>;
}

pub trait Chain<T, S: CheckState<T>> {
    fn and(self) -> Should<T, S>;
    fn which<C>(self, f: impl FnOnce(&T) -> Checked<C>) -> CheckedChild<T, C>;
    fn then<C>(self, f: impl FnOnce(&T) -> Checked<C>) -> Checked<T>;
    fn also<C>(self, f: impl FnOnce(&T) -> C) -> CheckedChild<T, C>;
}

pub trait Times<T> 
where
    T: IntoIterator
{
    fn check<S: CheckState<T::Item>>(self, inner: T, message: String, checks: impl Iterator<Item = S>) -> Checked<T>;
    // fn check_rebuild<S: CheckState<T::Item>>(self, checks: impl Iterator<Item = S>, message: String) -> Checked<impl IntoIterator<Item = T::Item>>;
}

impl<T> CheckMateEntrypoint<T> for T {
    fn value(self) -> Checked<T> {
        Checked::valid(self)
    }

    fn value_as_ref(&self) -> Checked<&T> {
        Checked::valid(self)
    }

    fn value_rc(self) -> Checked<Rc<T>> {
        Checked::valid(Rc::new(self))
    }

    fn value_cloned(&self) -> Checked<T> 
    where 
        T: Clone + Deref
    {
        let self_clone = self.clone();
        Checked::valid(self_clone)
    }
}

impl<T, S: CheckState<T>> Assertions<T> for S {
    fn assert_valid(&self) {
        match self.get_state_as_ref() {
            Checked::Valid { value: _ } => (),
            Checked::Invalid { value: _, message } => panic!("{message}")
        }
    }

    fn assert_invalid(&self) {
        match self.get_state_as_ref() {
            Checked::Valid { value: _ } => panic!("Should be invalid. Got valid value."),
            Checked::Invalid { value: _, message } => panic!("{message}")
        }
    }

    fn assert_invalid_with_message(&self, err_message: &str) {
        match self.get_state_as_ref() {
            Checked::Valid { value: _ } => self.assert_invalid(),
            Checked::Invalid { value: _, message } => {
                if message != err_message {
                    panic!("Found error value but with invalid message. Got {message}, expected {err_message}");
                };
            }
        }
    }
}

impl<T, S: CheckState<T>> Chain<T, S> for S {
    fn and(self) -> Should<T, S> { 
        Should::new(self)
    }
    
    fn which<C>(self, f: impl FnOnce(&T) -> Checked<C>) -> CheckedChild<T, C> {
        match self.get_state() {
            Checked::Valid { value } => {
                let child = (f)(&value);
                CheckedChild::new(Checked::valid(value), child)
            },
            Checked::Invalid { value, message } => {
                let child = (f)(&value);
                CheckedChild::new(Checked::invalid(value, message), child)
            },
        }
    }
    
    fn then<C>(self, f: impl FnOnce(&T) -> Checked<C>) -> Checked<T> {
        match self.get_state() {
            Checked::Invalid { value, message } => Checked::invalid(value, message),
            Checked::Valid { value } => {
                match (f)(&value) {
                    Checked::Valid { value: _} => Checked::valid(value),
                    Checked::Invalid { value: _, message } => Checked::invalid(value, message)
                }
            },
        }
    }

    fn also<C>(self, f: impl FnOnce(&T) -> C) -> CheckedChild<T, C> {
        match self.get_state() {
            Checked::Valid { value } => {
                let child = (f)(&value);
                CheckedChild::new(Checked::valid(value), Checked::valid(child))
            },
            Checked::Invalid { value, message } => {
                let child = (f)(&value);
                CheckedChild::new(Checked::invalid(value, message), Checked::valid(child))
            },
        }
    }
}

pub enum Checked<T> {
    Valid{
        value: T,
    },
    Invalid{
        value: T,
        message: String,
    },
}

impl<T> CheckState<T> for Checked<T> {
    fn get_state_as_ref(&self) -> &Checked<T> {
        &self
    }

    fn get_state(self) -> Checked<T> {
        self
    }

    fn propagate(self, checked: Checked<T>) -> Self {
        checked
    }

    fn propagate_predicate(self, check: impl FnOnce(T) -> Checked<T>) -> Self {
        match self {
            Self::Valid { value } => (check)(value),
            Self::Invalid { value, message } => Self::invalid(value, message)
        }
    }
}

impl<T> Checked<T> {
    pub fn valid(value: T) -> Self {
        Self::Valid { value }
    }

    pub fn invalid(value: T, message: String) -> Self {
        Self::Invalid { value, message }
    }

    pub fn with_message(self, message: &str) -> Self {
        match self {
            Self::Valid { value } => Self::valid(value),
            Self::Invalid { value, message: _} => Self::invalid(value, message.to_owned()),
        }
    }
}

impl<T> Directive<T, Checked<T>> for Checked<T> {
    fn should(self) -> Should<T, Checked<T>> {
        Should::new(self)
    }
}

pub struct CheckedChild<P, C> {
    parent: Checked<P>,
    check_state: Checked<C>,
}

impl<P, C> CheckState<C> for CheckedChild<P, C> {
    fn get_state_as_ref(&self) -> &Checked<C> {
        &self.check_state
    }

    fn get_state(self) -> Checked<C> {
        self.check_state
    }

    fn propagate(self, checked: Checked<C>) -> Self {
        Self::new(self.parent, checked)
    }

    fn propagate_predicate(self, check: impl FnOnce(C) -> Checked<C>) -> Self {
        match self.check_state {
            Checked::Valid { value } => Self::new(self.parent, (check)(value)),
            Checked::Invalid { value, message } => Self::new(self.parent, Checked::invalid(value, message)),
        }
    }
}

impl<P, C> Directive<C, CheckedChild<P, C>> for CheckedChild<P, C> {
    fn should(self) -> Should<C, CheckedChild<P, C>> {
        Should::new(self)
    }
}

impl<P, C> CheckedChild<P, C> {
    pub fn new(parent: Checked<P>, child: Checked<C>) -> Self {
        Self {parent, check_state: child}
    }

    pub fn with_message(self, message: &str) -> Self {
        match self.check_state {
            Checked::Valid { value } => Self::new(self.parent, Checked::valid(value)),
            Checked::Invalid { value, message: _} => Self::new(self.parent, Checked::invalid(value, message.to_owned())),
        }
    }
    
    pub fn done(self) -> Checked<P> {
        match self.check_state {
            Checked::Valid { value: _ } => self.parent,
            Checked::Invalid { value: _, message } => {
                self.parent.with_message(&message)
            }
        }
    }
}

pub struct Should<T, S: CheckState<T>> {
    inner: S,
    marker: PhantomData<T>,
}

impl<T, S: CheckState<T>> Should<T, S> {
    pub fn new(inner: S) -> Self {
        Self { inner, marker: PhantomData }
    }

    pub fn check_state(self) -> Checked<T> {
        self.inner.get_state()
    }

    pub fn check_state_ref(&self) -> &Checked<T> {
        self.inner.get_state_as_ref()
    }

    pub fn propagate_check(self, check: Checked<T>) -> S {
        self.inner.propagate(check)
    }

    pub fn match_predicate(self, predicate: impl FnOnce(T) -> Checked<T>) -> S {
        self.inner.propagate_predicate(predicate)
    }
}