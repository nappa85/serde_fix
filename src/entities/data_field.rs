use std::borrow::Cow;

use serde::{Serialize, Serializer, Deserialize, Deserializer};

pub trait DataField: Default {
    fn get_len(&self) -> &usize;
    fn set_len(&mut self, len: usize);
    fn get_data(&self) -> &str;
    fn set_data(&mut self, data: String);
}

pub fn serialize<T: DataField, S: Serializer>(obj: &T, serializer: S, data_key: &str) -> Result<S::Ok, S::Error> {
    let temp = format!("{}\u{1}{}={}", obj.get_len(), data_key, obj.get_data());
    temp.serialize(serializer)
}

pub fn deserialize<'de, D: Deserializer<'de>, T: DataField>(deserializer: D, data_key: &str) -> Result<T, D::Error> {
    let a = <Cow<'_, str> as Deserialize>::deserialize(deserializer)?;

    let (field1, field2) = match a.find('\u{1}') {
        Some(i) => (&a[0..i], if a.len() > i + (data_key.len() + 2) { &a[(i + data_key.len() + 2)..] } else { "" }),
        None => (&a[0..], ""),
    };

    let mut out = T::default();
    out.set_len(field1.parse().map_err(serde::de::Error::custom)?);
    out.set_data(field2.to_owned());
    Ok(out)
}
