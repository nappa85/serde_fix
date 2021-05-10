use std::{borrow::Cow, convert::TryFrom};

use chrono::{Date, NaiveDate, Utc};
use serde::{self, Serialize, Deserialize, Serializer, Deserializer};

const FORMAT: &str = "%Y%m%d";

#[derive(Clone, Debug, PartialEq)]
pub struct UTCDateOnly(Date<Utc>);

impl Default for UTCDateOnly {
    fn default() -> Self {
        UTCDateOnly(Utc::today())
    }
}

impl From<Date<Utc>> for UTCDateOnly {
    fn from(date: Date<Utc>) -> Self {
        UTCDateOnly(date)
    }
}

impl AsRef<Date<Utc>> for UTCDateOnly {
    fn as_ref(&self) -> &Date<Utc> {
        &self.0
    }
}

impl AsMut<Date<Utc>> for UTCDateOnly {
    fn as_mut(&mut self) -> &mut Date<Utc> {
        &mut self.0
    }
}

impl TryFrom<&str> for UTCDateOnly {
    type Error = chrono::ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(UTCDateOnly(Date::from_utc(NaiveDate::parse_from_str(s, FORMAT)?, Utc)))
    }
}

impl ToString for UTCDateOnly {
    fn to_string(&self) -> String {
        format!("{}", self.0.format(FORMAT))
    }
}

impl Serialize for UTCDateOnly {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
} 

impl<'de> Deserialize<'de> for UTCDateOnly {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<UTCDateOnly, D::Error> {
        UTCDateOnly::try_from(<Cow<'_, str> as Deserialize>::deserialize(deserializer)?.as_ref()).map_err(serde::de::Error::custom)
    }
}
