use crate::core::{Should, Checked, CheckState};

impl<T, E, S: CheckState<Result<T, E>>> Should<Result<T, E>, S> {
    pub fn be_ok(self) -> S {
        self.match_predicate(|inner| -> Checked<Result<T, E>> {
            if inner.is_ok() {
                Checked::valid(inner)
            } else {
                Checked::invalid(inner, format!("Should be ok"))
            }
        })
    }

    pub fn be_err(self) -> S {
        self.match_predicate(|inner| -> Checked<Result<T, E>> {
            if inner.is_err() {
                Checked::valid(inner)
            } else {
                Checked::invalid(inner, format!("Should be Error"))
            }
        })
    }
}