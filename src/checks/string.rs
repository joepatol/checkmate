use crate::core::{Checked, Should, CheckState};

impl<S: CheckState<String>> Should<String, S> {
    pub fn contain_substring(self, value: &str) -> S {
        self.match_predicate(move |inner| -> Checked<String> {
            if inner.contains(value) {
                Checked::valid(inner)
            } else {
                Checked::invalid(inner , format!("Should contain substring '{value}"))
            }
        })
    }

    pub fn contain_any_of_the_substrings<'i>(self, values: impl IntoIterator<Item = &'i str> + std::fmt::Debug + Clone) -> S {
        let msg = format!("Should contain one of {values:?}");
        self.match_predicate(|inner| -> Checked<String> {
            for value in values.into_iter(){
                if inner.clone().contains(value) {
                    return Checked::valid(inner)
                };
            };
            Checked::invalid(inner, msg)
        })
    }

    pub fn contain_all_of_the_substrings<'i>(self, values: impl IntoIterator<Item = &'i str> + std::fmt::Debug + Clone) -> S {
        let msg = format!("Should contain all of {values:?}");
        self.match_predicate(|inner| -> Checked<String> {
            for value in values.into_iter(){
                if !inner.clone().contains(value) {
                    return Checked::invalid(inner, msg)
                };
            };
            Checked::valid(inner)
        })
    }

    pub fn start_with<'a>(self, value: &'a str) -> S {
        self.match_predicate(|inner| -> Checked<String> {
            if inner.starts_with(value) {
                Checked::valid(inner)
            } else {
                Checked::invalid(inner, format!("Should start with {value:?}"))
            }
        })
    }

    pub fn end_with<'a>(self, value: &'a str) -> S {
        self.match_predicate(|inner| -> Checked<String> {
            if inner.ends_with(value) {
                Checked::valid(inner)
            } else {
                Checked::invalid(inner, format!("Should end with {value:?}"))
            }
        })
    }

    pub fn have_length(self, length: usize) -> S {
        self.match_predicate(|inner| -> Checked<String> {
            if inner.len() == length {
                Checked::valid(inner)
            } else {
                Checked::invalid(inner, format!("Lenght should be {length}"))
            }
        })
    }

    pub fn be_empty_str(self) -> S {
        self.match_predicate(|inner| -> Checked<String> {
            if inner == "" {
                Checked::valid(inner)
            } else {
                Checked::invalid(inner, String::from("Should be empty"))
            }
        })
    }

    pub fn not_be_empty_str(self) -> S {
        self.match_predicate(|inner| -> Checked<String> {
            if inner == "" {
                Checked::invalid(inner, String::from("Should not be empty"))
            } else {
                Checked::valid(inner)
            }
        })
    }

    pub fn be_equivalent_to(self, value: &str) -> S {
        self.match_predicate(|inner| -> Checked<String> {
            if value.to_lowercase() == inner.to_lowercase() {
                Checked::valid(inner)
            } else {
                Checked::invalid(inner, format!("Should be equivalent to {value:?}"))
            }
        })
    }

    pub fn not_be_equivalent_to(self, value: &str) -> S {
        self.match_predicate(|inner| -> Checked<String> {
            if value.to_lowercase() == inner.to_lowercase() {
                Checked::invalid(inner, format!("Should not be equivalent to {value:?}"))
            } else {
                Checked::valid(inner)
            }
        })
    }
}