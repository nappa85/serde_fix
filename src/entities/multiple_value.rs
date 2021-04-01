use std::borrow::Cow;
use std::fmt::Debug;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
pub struct MultipleValue<T: DeserializeOwned + Serialize + Clone + Debug + PartialEq>(pub Vec<T>);

impl<T> AsRef<Vec<T>> for MultipleValue<T>
where T: DeserializeOwned + Serialize + Clone + Debug + PartialEq {
    fn as_ref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T> Serialize for MultipleValue<T>
where T: DeserializeOwned + Serialize + Clone + Debug + PartialEq {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut v = Vec::new();
        for t in &self.0 {
            v.push(crate::to_string(t).map_err(serde::ser::Error::custom)?);
        }
        let s = v.join(" ");
        s.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for MultipleValue<T>
where T: DeserializeOwned + Serialize + Clone + Debug + PartialEq {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let a = <Cow<'_, str> as Deserialize>::deserialize(deserializer)?;
        let mut v = Vec::new();
        for t in a.split(' ') {
            v.push(crate::from_str::<T>(t).map_err(serde::de::Error::custom)?);
        }
        Ok(MultipleValue(v))
    }
}
