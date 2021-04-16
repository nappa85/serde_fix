use std::{borrow::Cow, str::FromStr, fmt::Display, any::type_name};

use serde::{Deserialize, Deserializer};

/// due to https://github.com/serde-rs/serde/issues/1183
pub fn from_str<'de, D, S>(deserializer: D) -> Result<S, D::Error>
    where D: Deserializer<'de>,
          S: FromStr,
          S::Err: Display,
{
    let s = <Cow<'_, str> as Deserialize>::deserialize(deserializer)?;
    S::from_str(&s).map_err(|_| serde::de::Error::custom(format!("could not parse {}", type_name::<S>())))
}

/// due to https://github.com/serde-rs/serde/issues/1183
pub fn from_opt_str<'de, D, S>(deserializer: D) -> Result<Option<S>, D::Error>
    where D: Deserializer<'de>,
          S: FromStr,
          S::Err: Display,
{
    Ok(match <Option<Cow<'_, str>> as Deserialize>::deserialize(deserializer)? {
        Some(s) => Some(S::from_str(&s).map_err(|e| serde::de::Error::custom(format!("could not parse option {}: {}", type_name::<S>(), e)))?),
        None => None,
    })
}
