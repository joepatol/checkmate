use crate::core::{Should, Checked};

impl Should<bool> {
    pub fn be_true(self, message: &str) -> Checked<bool> {
        self.match_predicate(|inner| -> Checked<bool> {
            if inner {
                Checked::valid(inner)
            } else {
                Checked::invalid(inner, format!("{message}"))
            }
        })
    }

    pub fn be_false(self, message: &str) -> Checked<bool> {
        self.match_predicate(|inner| -> Checked<bool> {
            if inner == false {
                Checked::valid(inner)
            } else {
                Checked::invalid(inner, format!("{message}"))
            }
        })
    }
}