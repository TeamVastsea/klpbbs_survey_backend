pub mod entity;
pub mod model;

use sea_orm::prelude::DateTime;
use serde::{Deserialize, Deserializer, Serializer};
use std::str::FromStr;

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
        None => serializer.serialize_none(),
    }
}

pub fn deserialize_datetime_as_z<'de, D>(deserializer: D) -> Result<DateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    println!("{}", s);
    let s = s.trim_end_matches('Z');
    DateTime::from_str(s).map_err(serde::de::Error::custom)
}

fn deserialize_option_datetime_as_z<'de, D>(deserializer: D) -> Result<Option<DateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    println!("{:?}", opt);
    match opt {
        Some(s) => {
            let s = s.trim_end_matches('Z'); // 去掉末尾的 'Z'
            DateTime::from_str(s)
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
        None => Ok(None),
    }
}
