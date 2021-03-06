use std::{borrow::Cow, convert::TryFrom};

use chrono::{DateTime, Utc, TimeZone};
use serde::{self, Serialize, Deserialize, Serializer, Deserializer};

const FORMAT: &str = "%Y%m%d-%T%.3f";

#[derive(Clone, Debug, PartialEq)]
pub struct UTCTimestamp(DateTime<Utc>);

impl Default for UTCTimestamp {
    fn default() -> Self {
        UTCTimestamp(Utc::now())
    }
}

impl From<DateTime<Utc>> for UTCTimestamp {
    fn from(date: DateTime<Utc>) -> Self {
        UTCTimestamp(date)
    }
}

impl AsRef<DateTime<Utc>> for UTCTimestamp {
    fn as_ref(&self) -> &DateTime<Utc> {
        &self.0
    }
}

impl AsMut<DateTime<Utc>> for UTCTimestamp {
    fn as_mut(&mut self) -> &mut DateTime<Utc> {
        &mut self.0
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
        UTCTimestamp::try_from(<Cow<'_, str> as Deserialize>::deserialize(deserializer)?.as_ref()).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod test {
    use std::convert::TryFrom;

    use chrono::{DateTime, NaiveDate, Utc};

    use super::UTCTimestamp;

    #[test]
    fn decode() {
        let msg = "20210511-08:55:12.123";
        assert_eq!(UTCTimestamp::try_from(msg).unwrap(), UTCTimestamp::from(DateTime::from_utc(NaiveDate::from_ymd(2021, 5, 11).and_hms_milli(8, 55, 12, 123), Utc)));
    }
}
