use std::{borrow::Cow, convert::TryFrom};

use chrono::{DateTime, Local, TimeZone};
use serde::{self, Serialize, Deserialize, Serializer, Deserializer};

const FORMAT: &str = "%Y%m%d";

#[derive(Clone, Debug, PartialEq)]
pub struct LocalMktDate(DateTime<Local>);

impl Default for LocalMktDate {
    fn default() -> Self {
        LocalMktDate(Local::now())
    }
}

impl From<DateTime<Local>> for LocalMktDate {
    fn from(date: DateTime<Local>) -> Self {
        LocalMktDate(date)
    }
}

impl AsRef<DateTime<Local>> for LocalMktDate {
    fn as_ref(&self) -> &DateTime<Local> {
        &self.0
    }
}

impl AsMut<DateTime<Local>> for LocalMktDate {
    fn as_mut(&mut self) -> &mut DateTime<Local> {
        &mut self.0
    }
}

impl TryFrom<&str> for LocalMktDate {
    type Error = chrono::ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(LocalMktDate(Local.datetime_from_str(s, FORMAT)?))
    }
}

impl ToString for LocalMktDate {
    fn to_string(&self) -> String {
        format!("{}", self.0.format(FORMAT))
    }
}

impl Serialize for LocalMktDate {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
} 

impl<'de> Deserialize<'de> for LocalMktDate {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<LocalMktDate, D::Error> {
        LocalMktDate::try_from(<Cow<'_, str> as Deserialize>::deserialize(deserializer)?.as_ref()).map_err(serde::de::Error::custom)
    }
}
