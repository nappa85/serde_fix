use std::convert::{AsRef, TryFrom};

use chrono::{DateTime, Utc, TimeZone};
use serde::{self, Serialize, Deserialize, Serializer, Deserializer};

const FORMAT: &'static str = "%Y%m%d-%T%.3f";

#[derive(Debug)]
pub struct UTCTimestamp(DateTime<Utc>);

impl AsRef<DateTime<Utc>> for UTCTimestamp {
    fn as_ref(&self) -> &DateTime<Utc> {
        &self.0
    }
}

impl TryFrom<&str> for UTCTimestamp {
    type Error = chrono::ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(UTCTimestamp(Utc.datetime_from_str(s, FORMAT)?))
    }
}

impl ToString for UTCTimestamp {
    fn to_string(&self) -> String {
        format!("{}", self.0.format(FORMAT))
    }
}

impl Serialize for UTCTimestamp {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
} 

impl<'de> Deserialize<'de> for UTCTimestamp {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<UTCTimestamp, D::Error> {
        UTCTimestamp::try_from(<&str as Deserialize>::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
