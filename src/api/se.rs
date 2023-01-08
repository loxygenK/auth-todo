use std::fmt::Display;

use serde::Serializer;

pub fn with_display<D, S>(value: &D, serializer: S) -> Result<S::Ok, S::Error>
    where D: Display, S: Serializer
{
    serializer.collect_str(value)
}
