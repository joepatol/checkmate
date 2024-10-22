use crate::core::{Should, Checked};

impl<T, E> Should<Result<T, E>> {
    pub fn be_ok(self) -> Checked<Result<T, E>> {
        self.match_predicate(|inner| -> Checked<Result<T, E>> {
            if inner.is_ok() {
                Checked::valid(inner)
            } else {
                Checked::invalid(inner, format!("Should be ok"))
            }
        })
    }

    pub fn be_err(self) -> Checked<Result<T, E>> {
        self.match_predicate(|inner| -> Checked<Result<T, E>> {
            if inner.is_err() {
                Checked::valid(inner)
            } else {
                Checked::invalid(inner, format!("Should be Error"))
            }
        })
    }
}