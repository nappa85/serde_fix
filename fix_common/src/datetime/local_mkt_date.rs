use std::{borrow::Cow, convert::TryFrom};

use chrono::{NaiveDate, Local};
use serde::{self, Serialize, Deserialize, Serializer, Deserializer};

const FORMAT: &str = "%Y%m%d";

#[derive(Clone, Debug, PartialEq)]
pub struct LocalMktDate(NaiveDate);

impl Default for LocalMktDate {
    fn default() -> Self {
        LocalMktDate(Local::today().naive_local())
    }
}

impl From<NaiveDate> for LocalMktDate {
    fn from(date: NaiveDate) -> Self {
        LocalMktDate(date)
    }
}

impl AsRef<NaiveDate> for LocalMktDate {
    fn as_ref(&self) -> &NaiveDate {
        &self.0
    }
}

impl AsMut<NaiveDate> for LocalMktDate {
    fn as_mut(&mut self) -> &mut NaiveDate {
        &mut self.0
    }
}

impl TryFrom<&str> for LocalMktDate {
    type Error = chrono::ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(LocalMktDate(NaiveDate::parse_from_str(s, FORMAT)?))
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
