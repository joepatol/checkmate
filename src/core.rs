use std::error::Error;
use std::fmt::{Display, Result as FmtResult, Formatter};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
pub struct CheckMateError {
    details: String
}

impl CheckMateError {
    fn new(msg: String) -> Self {
        Self{details: msg}
    }
}

impl Display for CheckMateError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f,"{}",self.details)
    }
}

impl Error for CheckMateError {}

pub type CheckResult = Result<(), CheckMateError>;

pub trait CheckState<T> {
    fn get_state_as_ref(&self) -> &Checked<T>;
    fn get_state(self) -> Checked<T>;
    fn propagate(self, checked: Checked<T>) -> Self;
    fn propagate_predicate(self, check: impl FnOnce(T) -> Checked<T>) -> Self;
}

/// A value that has been checked
/// Together with `Should` this is one of the core types in `checkmate`'s control flow
/// 
/// Can be either `Valid` or `Invalid`. In the valid state this type simply wraps
/// the type being checked. In the invalid state, the checked type is wrapped together with
/// a message.
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
    /// Create a new `Checked` in the valid state
    pub fn valid(value: T) -> Self {
        Self::Valid { value }
    }

    /// Create a new `Checked` in the invalid state
    pub fn invalid(value: T, message: String) -> Self {
        Self::Invalid { value, message }
    }

    /// Create a new `Checked` with a custom message
    pub fn with_message(self, message: &str) -> Self {
        match self {
            Self::Valid { value } => Self::valid(value),
            Self::Invalid { value, message: _} => Self::invalid(value, message.to_owned()),
        }
    }

    pub fn to_inner(self) -> T {
        match self {
            Self::Valid { value  } => value,
            Self::Invalid { value, message: _ } => value,
        }
    }

    pub fn is_valid(&self) -> bool {
        match self {
            Self::Valid { value: _ } => true,
            Self::Invalid { value: _, message: _ } => false,
        }
    }

    /// Calls the `assert_eq` macro, to assert that `self` is in the valid state
    pub fn assert_ok(self) {
        assert_eq!(self.get_result().is_ok(), true)
    }

    /// Calls the `assert_eq` macro, to assert that `self` is in the invalid state 
    pub fn assert_err(self) {
        assert_eq!(self.get_result().is_err(), true)
    }

    /// Assert `self` is in the invalid state and validate the provided message.
    /// This function `panics` if the assertions do not hold
    pub fn assert_err_with_message(self, err_message: &str) {
        match self {
            Self::Valid { value: _ } => self.assert_err(),
            Self::Invalid { value: _, message } => {
                if message != err_message {
                    panic!("Found error value but with invalid message. Got {message}, expected {err_message}");
                };
            }
        }
    }

    /// Get a `Result` based on the current state
    /// 
    /// If `Self::Valid` return `Result::Ok(())`
    /// Else `Result::Err(CheckMateError)` is returned
    pub fn get_result(self) -> CheckResult {
        match self {
            Self::Invalid { value: _, message } => {
                Err(CheckMateError::new(message))
            },
            Self::Valid { value: _ } => {
                Ok(())
            }
        }
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

impl<P, C> CheckedChild<P, C> {
    pub fn new(parent: Checked<P>, child: Checked<C>) -> Self {
        Self {parent, check_state: child}
    }
    
    pub fn done(self) -> Checked<P> {
        match self.check_state {
            Checked::Valid { value: _ } => self.parent,
            Checked::Invalid { value: _, message } => {
                Checked::invalid(self.parent.to_inner(), message)
            }
        }
    }
}

/// `Should` wraps a value to be checked.
/// Checks are implemented on `Should<T>`. This way check functions are only available
/// when wrapping your value in a `Should`.
/// 
/// Should will store `T` in a `Checked` to enable control flow. 
/// `impl Should` blocks must thus first check the current state of the check flow
/// before being able to access the internal value.
/// 
/// ```
/// let should = Should::new(Checked::valid(1))
/// 
/// match should.check_state {
///     Checked::Valid { value } => {
///         // Add check code here
///         Checked::valid(value)
///     }
///     Checked::Invalid { value, message } => Checked::invalid(value, message)
/// }
/// ```
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

pub trait CheckMateEntrypoint<T> {
    fn hey_checkmate_this_value(self) -> Checked<T>;
    fn hey_checkmate_this_borrowed_value(&self) -> Checked<&T>;
    fn hey_checkmate_this_rc_value(self) -> Checked<Rc<T>>;
    fn hey_checkmate_this_cloned_value(&self) -> Checked<T> where T: Clone + Deref;
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

pub trait Times<T> {
    fn do_count(self, inner: T, message: String, checks: impl IntoIterator<Item = bool>) -> Checked<T>;
}

impl<T> CheckMateEntrypoint<T> for T {
    fn hey_checkmate_this_value(self) -> Checked<T> {
        Checked::valid(self)
    }

    fn hey_checkmate_this_borrowed_value(&self) -> Checked<&T> {
        Checked::valid(self)
    }

    fn hey_checkmate_this_rc_value(self) -> Checked<Rc<T>> {
        Checked::valid(Rc::new(self))
    }

    fn hey_checkmate_this_cloned_value(&self) -> Checked<T> 
    where 
        T: Clone + Deref
    {
        let self_clone = self.clone();
        Checked::valid(self_clone)
    }
}

impl<T> Directive<T, Checked<T>> for Checked<T> {
    fn should(self) -> Should<T, Checked<T>> {
        Should::new(self)
    }
}

impl<P, C> Directive<C, CheckedChild<P, C>> for CheckedChild<P, C> {
    fn should(self) -> Should<C, CheckedChild<P, C>> {
        Should::new(self)
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