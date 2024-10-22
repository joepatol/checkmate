use std::fmt::Debug;

use crate::core::{Checked, Should, Times};

impl<T> Should<T>
where 
    T: IntoIterator + Clone
{
    pub fn match_predicate_times<F, C>(self, predicate: F, times: C, message: String) -> Checked<T>
    where 
        F: Fn(T::Item) -> Checked<T::Item>,
        C: Times<T>,
    {
        match self.check_state {
            Checked::Valid { value } => {
                let bools = value.clone().into_iter().map(|v|(predicate(v).is_valid()));
                times.do_count(value, message, bools)
            },
            Checked::Invalid { value, message } => {
                Checked::invalid(value, message.clone())
            }
        }
    }
}

impl<T, U> Should<T>
where 
    T: IntoIterator<Item = U> + Clone,
    T::Item: PartialEq<U>,
    U: Debug,
{
    pub fn have_count(self, count: usize) -> Checked<T> {
        self.match_predicate(|inner| -> Checked<T> {
            if inner.clone().into_iter().count() == count {
                Checked::valid(inner)
            } else {
                Checked::invalid(inner, format!("Should contain exactly {count} items"))
            }
        })
    }

    pub fn be_empty(self) -> Checked<T> {
        self.match_predicate(|inner| -> Checked<T> {
            if inner.clone().into_iter().count() == 0 {
                Checked::valid(inner)
            } else {
                Checked::invalid(inner, format!("Should be empty"))
            } 
        })
    }

    pub fn not_be_empty(self) -> Checked<T> {
        self.match_predicate(|inner| -> Checked<T> {
            if inner.clone().into_iter().count() > 0 {
                Checked::valid(inner)
            } else {
                Checked::invalid(inner, format!("Should not be empty"))
            } 
        })
    }

    pub fn contain<C: Times<T>>(self, value: U, times: C) -> Checked<T> {
        let predicate = |val: T::Item| -> Checked<T::Item> {
            if val == value {
                Checked::valid(val)
            } else {
                Checked::invalid(val, format!("Should contain {value:?}"))
            }
        };

        self.match_predicate_times(predicate, times, format!("Should contain {value:?}"))
    }

    pub fn contain_any_of(self, values: impl IntoIterator<Item = U> + Debug) -> Checked<T> {
        let msg = format!("Should contain {values:?}");
        self.match_predicate(|val| -> Checked<T> {
            for value in values.into_iter(){
                if val.clone().into_iter().any(|x| x == value) {
                    return Checked::valid(val)
                };
            };
            Checked::invalid(val, msg)
        })
    }

    pub fn contain_all_of(self, values: impl IntoIterator<Item = U> + Debug) -> Checked<T> {
        let msg = format!("Should contain all of {values:?}");
        self.match_predicate(|val| -> Checked<T> {
            for value in values.into_iter(){
                if !val.clone().into_iter().any(|x| x == value) {
                    return Checked::invalid(val, msg)
                };
            };
            Checked::valid(val)
        })
    }

    pub fn contain_none_of(self, values: impl IntoIterator<Item = U> + Debug) -> Checked<T> {
        let msg = format!("Should contain none of {values:?}");
        self.match_predicate(|val| -> Checked<T> {
            for value in values.into_iter(){
                if val.clone().into_iter().any(|x| x == value) {
                    return Checked::invalid(val, msg)
                };
            };
            Checked::valid(val)
        })
    }
}