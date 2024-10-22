use thiserror::Error;
use derive_more::Constructor;
use std::fmt::Debug;

#[derive(Error, Debug)]
pub enum CheckMateError {
    #[error("{0}")]
    Custom(String),
}

impl CheckMateError {
    pub fn custom(val: impl std::fmt::Display) -> Self {
        Self::Custom(val.to_string())
    }
}

pub type CheckResult = Result<(), CheckMateError>;

pub enum Checked<T> {
    Valid{
        value: T,
    },
    Invalid{
        value: T,
        message: String,
    },
}

impl<T> Checked<T> {
    pub fn valid(value: T) -> Self {
        Self::Valid { value }
    }

    pub fn invalid(value: T, message: String) -> Self {
        Self::Invalid { value, message }
    }

    pub fn with_message(self, message: String) -> Self {
        match self {
            Self::Valid { value } => Self::valid(value),
            Self::Invalid { value, message: _} => Self::invalid(value, message),
        }
    }

    pub fn is_valid(&self) -> bool {
        match self {
            Self::Valid { value: _ } => true,
            Self::Invalid { value: _, message: _ } => false,
        }
    }

    pub fn assert_ok(self) {
        assert_eq!(self.get_result().is_ok(), true)
    }

    pub fn assert_err(self) {
        assert_eq!(self.get_result().is_err(), true)
    }

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

    pub fn get_result(self) -> CheckResult {
        match self {
            Self::Invalid { value: _, message } => {
                Err(CheckMateError::Custom(String::from(message)))
            },
            Self::Valid { value: _ } => {
                Ok(())
            }
        }
    }
}

#[derive(Constructor)]
pub struct Should<T> {
    pub check_state: Checked<T>
}

pub trait Directive<T> {
    fn should(self) -> Should<T>;
}

pub trait Chain {
    type Chained;

    fn and(self) -> Should<Self::Chained>;
    fn which(self) -> Self::Chained;
    fn where_<T>(self, f: impl FnOnce(&Self::Chained) -> Checked<T>) -> Checked<Self::Chained>;
}

pub trait Times<T> {
    fn do_count(self, inner: T, message: String, checks: impl IntoIterator<Item = bool>) -> Checked<T>;
}

impl<T> Directive<T> for T {
    fn should(self) -> Should<T> {
        Should::new(Checked::valid(self))
    }
}

impl<T: Clone> Chain for Checked<T> {
    type Chained = T;

    fn and(self) -> Should<T> {
        Should::new(self)
    }

    fn which(self) -> T {
        match self {
            Self::Valid { value } => value,
            Self::Invalid { value: _, message } => panic!("{message}"),
        }
    }

    fn where_<U>(self, f: impl FnOnce(&Self::Chained) -> Checked<U>) -> Checked<Self::Chained> {
        match self {
            Self::Invalid { value, message } => Self::invalid(value, message),
            Self::Valid { value } => {
                match (f)(&value) {
                    Checked::Valid { value: _} => Checked::valid(value),
                    Checked::Invalid { value: _, message } => Checked::invalid(value, message)
                }
            },
        }
    }
}

impl<T> Checked<Option<T>> {
    pub fn which_some(self) -> T {
        match self {
            Self::Valid { value } => {
                match value {
                    Some(v) => v,
                    None => panic!("Called `which()` on None value"),
                }
            },
            Self::Invalid { value: _, message } => panic!("{message}"),
        }
    }
}

impl<T: Debug, E: Debug> Checked<Result<T, E>> {
    pub fn which_ok(self) -> T {
        match self {
            Self::Valid { value } => {
                match value {
                    Ok(v) => v,
                    Err(e) => panic!("Called `which_ok()` on Err value, {e:?}"),
                }
            },
            Self::Invalid { value: _, message } => panic!("{message}"),
        }
    }

    pub fn which_err(self) -> E {
        match self {
            Self::Valid { value } => {
                match value {
                    Ok(v) => panic!("Called `which_err()` on Ok value, {v:?}"),
                    Err(e) => e,
                }
            },
            Self::Invalid { value: _, message } => panic!("{message}"),
        }
    }
}