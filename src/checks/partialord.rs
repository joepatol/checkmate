use crate::{core::{CheckState, Checked, Should}, message::format_value};

impl<T, S: CheckState<T>> Should<T, S> {
    pub fn be_greater_than<U: PartialOrd<T>>(self, value: U) -> S 
    where
        U: PartialOrd<T> + 'static
    {
        self.match_predicate(|inner| -> Checked<T> {
            if value < inner {
                Checked::valid(inner)
            } else {
                Checked::invalid(inner, format!("Should be > {}", format_value(&value)))
            }
        })
    }

    pub fn be_smaller_than<U>(self, value: U) -> S 
    where
        U: PartialOrd<T> + 'static
    {
        self.match_predicate(|inner| -> Checked<T> {
            if value > inner {
                Checked::valid(inner)
            } else {
                Checked::invalid(inner, format!("Should be < {}", format_value(&value)))
            }
        })
    }
}