use std::error::Error;

use crate::core::{Should, Checked};

impl<E: Error> Should<E> {
    pub fn have_message(self, message: &str) -> Checked<E> {
        self.match_predicate(|err| -> Checked<E> {
            if format!("{err}") != message {
                Checked::invalid(err, format!("Should be {message}"))
            } else {
                Checked::valid(err)
            }
        })
    }

    pub fn have_message_that_contains(self, message: &str) -> Checked<E> {
        self.match_predicate(|err| -> Checked<E> {
            if format!("{err}").contains(message) {
                Checked::valid(err)
            } else {
                Checked::invalid(err, format!("Error should contain {message}"))
            }
        })
    }
}