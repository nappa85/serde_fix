use std::{borrow::Cow, convert::{AsRef, TryFrom}, ops::Deref};

use chrono::{DateTime, Local, TimeZone};
use serde::{self, Serialize, Deserialize, Serializer, Deserializer};

const FORMAT: &'static str = "%Y%m%d";

#[derive(Clone, Debug, PartialEq)]
pub struct LocalMktDate(DateTime<Local>);

impl Default for LocalMktDate {
    fn default() -> Self {
        LocalMktDate(Local::now())
    }
}

impl AsRef<DateTime<Local>> for LocalMktDate {
    fn as_ref(&self) -> &DateTime<Local> {
        &self.0
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
        LocalMktDate::try_from(<Cow<'_, str> as Deserialize>::deserialize(deserializer)?.deref()).map_err(serde::de::Error::custom)
    }
}
