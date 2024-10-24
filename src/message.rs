use std::{any::Any, fmt::{Display, Debug}};

pub fn format_value(value: &dyn Any) -> String {
    if let Some(v) = value.downcast_ref::<&dyn Display>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<&dyn Debug>() {
        format!("{:?}", v)
    } else {
        "<type does not implement Display or Debug>".to_string()
    }
}