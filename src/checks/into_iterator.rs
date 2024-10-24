use std::fmt::Debug;

use crate::core::{CheckState, Checked, Should, Times};
use crate::message::format_value;

impl<T, S> Should<T, S>
where
    T: IntoIterator + Clone,
    S: CheckState<T>,
{
    pub fn match_predicate_times<F, C>(self, predicate: F, times: C, message: String) -> S
    where
        F: Fn(T::Item) -> Checked<T::Item>,
        C: Times<T>,
    {
        let check;
        match self.check_state_ref() {
            Checked::Valid { value } => {
                let checks = value.clone().into_iter().map(|v| (predicate(v)));
                check = times.check(value.to_owned(), message, checks)
            }
            Checked::Invalid { value, message } => {
                check = Checked::invalid(value.to_owned(), message.clone())
            }
        };
        self.propagate_check(check)
    }
}

impl<T, S, U> Should<T, S>
where
    T: IntoIterator<Item = U> + Clone,
    T::Item: PartialEq<U> + 'static,
    U: Debug,
    S: CheckState<T>,
{
    pub fn have_count(self, count: usize) -> S {
        self.match_predicate(|inner| -> Checked<T> {
            if inner.clone().into_iter().count() == count {
                Checked::valid(inner)
            } else {
                Checked::invalid(
                    inner,
                    format!("Iterator should contain exactly {count} items"),
                )
            }
        })
    }

    pub fn be_empty(self) -> S {
        self.match_predicate(|inner| -> Checked<T> {
            if inner.clone().into_iter().count() == 0 {
                Checked::valid(inner)
            } else {
                Checked::invalid(inner, format!("Iterator should be empty"))
            }
        })
    }

    pub fn not_be_empty(self) -> S {
        self.match_predicate(|inner| -> Checked<T> {
            if inner.clone().into_iter().count() > 0 {
                Checked::valid(inner)
            } else {
                Checked::invalid(inner, format!("Iterator should not be empty"))
            }
        })
    }

    pub fn contain<C: Times<T>>(self, value: U, times: C) -> S {
        let predicate = |val: T::Item| -> Checked<T::Item> {
            if val == value {
                Checked::valid(val)
            } else {
                Checked::invalid(
                    val,
                    format!("Iterator should contain {}", format_value(&value)),
                )
            }
        };

        self.match_predicate_times(
            predicate,
            times,
            format!("Iterator should contain {}", format_value(&value)),
        )
    }

    pub fn contain_any_of(self, values: impl IntoIterator<Item = U> + 'static) -> S {
        let msg = format!("Iterator should contain any of {}", format_value(&values));
        self.match_predicate(|val| -> Checked<T> {
            for value in values.into_iter() {
                if val.clone().into_iter().any(|x| x == value) {
                    return Checked::valid(val);
                };
            }
            Checked::invalid(val, msg)
        })
    }

    pub fn contain_all_of(self, values: impl IntoIterator<Item = U> + 'static) -> S {
        let msg = format!("Iterator should contain all of {}", format_value(&values));
        self.match_predicate(|val| -> Checked<T> {
            for value in values.into_iter() {
                if !val.clone().into_iter().any(|x| x == value) {
                    return Checked::invalid(val, msg);
                };
            }
            Checked::valid(val)
        })
    }

    pub fn contain_none_of(self, values: impl IntoIterator<Item = U> + 'static) -> S {
        let msg = format!("Iterator should contain none of {}", format_value(&values));
        self.match_predicate(|val| -> Checked<T> {
            for value in values.into_iter() {
                if val.clone().into_iter().any(|x| x == value) {
                    return Checked::invalid(val, msg);
                };
            }
            Checked::valid(val)
        })
    }
}
