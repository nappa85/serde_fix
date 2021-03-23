use std::str::FromStr;
use std::fmt::Display;

use serde::{Deserialize, Deserializer};

/// due to https://github.com/serde-rs/serde/issues/1183
pub fn from_str<'de, D, S>(deserializer: D) -> Result<S, D::Error>
    where D: Deserializer<'de>,
          S: FromStr,
          S::Err: Display,
{
    let s = <&str as Deserialize>::deserialize(deserializer)?;
    S::from_str(s).map_err(|_| serde::de::Error::custom("could not parse string"))
}

/// due to https://github.com/serde-rs/serde/issues/1183
pub fn from_opt_str<'de, D, S>(deserializer: D) -> Result<Option<S>, D::Error>
    where D: Deserializer<'de>,
          S: FromStr,
          S::Err: Display,
{
    Ok(match <Option<&str> as Deserialize>::deserialize(deserializer) {
        Ok(Some(s)) => Some(S::from_str(s).map_err(|_| serde::de::Error::custom("could not parse string"))?),
        Ok(None) => None,
        Err(e) => {
            println!("{:?}", e);
            return Err(e);
        },
    })
}
