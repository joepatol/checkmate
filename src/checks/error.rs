use std::error::Error;

use crate::core::{Should, Checked, CheckState};

impl<E: Error, S: CheckState<E>> Should<E, S> {
    pub fn have_message(self, message: &str) -> S {
        self.match_predicate(|err| -> Checked<E> {
            if format!("{err}") != message {
                Checked::invalid(err, format!("Error message should be {message}"))
            } else {
                Checked::valid(err)
            }
        })
    }

    pub fn have_message_that_contains(self, message: &str) -> S {
        self.match_predicate(|err| -> Checked<E> {
            if format!("{err}").contains(message) {
                Checked::valid(err)
            } else {
                Checked::invalid(err, format!("Error message should contain {message}"))
            }
        })
    }
}