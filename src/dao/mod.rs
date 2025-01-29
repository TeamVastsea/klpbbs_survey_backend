pub mod entity;
pub mod model;

use sea_orm::prelude::DateTime;
use serde::{Serializer, ser::Error, Deserialize};

fn serialize_datetime_as_z<S>(dt: &DateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = dt.to_string() + "Z";
    serializer.serialize_str(&s)
}

fn serialize_option_datetime_as_z<S>(
    dt: &Option<DateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match dt {
        Some(date) => {
            let s = date.to_string() + "Z";
            serializer.serialize_str(&s)
        }
        None => serializer.serialize_none()
    }
}