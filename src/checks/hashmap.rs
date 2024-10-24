use std::collections::HashMap;

use crate::message::format_value;
use crate::{
    core::{CheckState, Should},
    Checked,
};

impl<K, V, S: CheckState<HashMap<K, V>>> Should<HashMap<K, V>, S>
where
    K: std::cmp::PartialEq + 'static,
    V: std::cmp::PartialEq + 'static,
{
    pub fn contain_key(self, key: K) -> S {
        self.match_predicate(|inner| -> Checked<HashMap<K, V>> {
            if inner.keys().any(|k| k == &key) {
                Checked::valid(inner)
            } else {
                Checked::invalid(
                    inner,
                    format!("Should contain key '{}'", format_value(&key)),
                )
            }
        })
    }

    pub fn contain_value(self, value: V) -> S {
        self.match_predicate(|inner| -> Checked<HashMap<K, V>> {
            if inner.values().any(|k| k == &value) {
                Checked::valid(inner)
            } else {
                Checked::invalid(
                    inner,
                    format!("Should contain value '{}'", format_value(&value)),
                )
            }
        })
    }

    // pub fn contain_value<T: Times<Vec<V>>>(self, value: V, times: T) -> S 
    // where
    //     V: 'static,
    //     K: std::cmp::Eq + std::hash::Hash,
    // {
    //     self.match_predicate(|inner| -> Checked<HashMap<K, V>> {
    //         let checks: Vec<Checked<(K, V)>> = inner.into_iter().map(|(map_key, map_value)| -> Checked<(K, V)> {
    //             let checked = if value == map_value {
    //                 Checked::valid((map_key, map_value))
    //             } else {
    //                 Checked::invalid((map_key, map_value), format!("HashMap should contain value '{}'", format_value(&value)))
    //             };
    //             checked
    //         }).collect();
    //         Checked::valid(HashMap::new())
    //     })
    // }

    pub fn contain_pair(self, key: K, value: V) -> S
    where
        K: std::cmp::Eq + std::hash::Hash,
    {
        self.match_predicate(|inner| -> Checked<HashMap<K, V>> {
            if inner.get(&key).map(|v| v == &value) == Some(true) {
                Checked::valid(inner)
            } else {
                Checked::invalid(
                    inner,
                    format!(
                        "The HashMap should contain key-value pair: '{}, {}'",
                        format_value(&key),
                        format_value(&value)
                    ),
                )
            }
        })
    }
}
