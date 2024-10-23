use crate::core::{Checked, Should, CheckState};

impl<S: CheckState<bool>> Should<bool, S> {
    pub fn be_true(self, message: &str) -> S {
        self.match_predicate(|inner| -> Checked<bool> {
            if inner {
                Checked::valid(inner)
            } else {
                Checked::invalid(inner, format!("{message}"))
            }
        })
    }

    pub fn be_false(self, message: &str) -> S {
        self.match_predicate(|inner| -> Checked<bool> {
            if inner == false {
                Checked::valid(inner)
            } else {
                Checked::invalid(inner, format!("{message}"))
            }
        })
    }
}