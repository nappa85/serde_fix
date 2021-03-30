use std::borrow::Cow;

use serde::{Serialize, Deserialize};

use super::instrument::SecurityIDSource;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SecAltIDGrp {
    // #[serde(rename = "454")]
    len: usize,
    inner: Vec<SecAltID>,
}

impl<'de> Deserialize<'de> for SecAltIDGrp {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<SecAltIDGrp, D::Error> {
        let temp = <Cow<'_, str> as Deserialize>::deserialize(deserializer).map_err(serde::de::Error::custom)?;
        let mut group = SecAltIDGrp::default();
        let mut actual = SecAltID::default();
        let iterator = temp.split('\u{1}').map(|a| match a.find('=') {
            Some(i) => (&a[0..i], &a[(i + 1)..]),
            None => ("", &a[0..]),
        });
        for (code, value) in iterator {
            match &*code {
                "455" => {
                    if actual.security_alt_id.is_some() || actual.security_alt_id_source.is_some() {
                        group.inner.push(actual);
                        actual = SecAltID::default();
                    }
                    actual.security_alt_id = Some(value.to_owned());
                },
                "456" => {
                    if actual.security_alt_id_source.is_some() {
                        group.inner.push(actual);
                        actual = SecAltID::default();
                    }
                    actual.security_alt_id_source = crate::from_str(value).map_err(serde::de::Error::custom)?;
                },
                // 454
                _ => {
                    group.len = value.parse().map_err(serde::de::Error::custom)?;
                },
            }
        }
        if actual.security_alt_id.is_some() || actual.security_alt_id_source.is_some() {
            group.inner.push(actual);
        }
        Ok(group)
    }
}

impl Serialize for SecAltIDGrp {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut temp = vec![self.len.to_string()];
        for hop in &self.inner {
            if let Some(s) = &hop.security_alt_id {
                temp.push(format!("455={}", s));
            }
            if let Some(s) = &hop.security_alt_id_source {
                temp.push(format!("456={}", crate::to_string(s).map_err(serde::ser::Error::custom)?));
            }
        }
        temp.join("\u{1}").serialize(serializer)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SecAltID {
    // #[serde(rename = "455")]
    pub security_alt_id: Option<String>,
    // #[serde(rename = "456")]
    pub security_alt_id_source: Option<SecurityIDSource>,
}
