use std::borrow::Cow;

use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RepeatingValues<T: Serialize + DeserializeOwned + Default>(pub Vec<T>);

impl<T> AsRef<Vec<T>> for RepeatingValues<T>
where T: Serialize + DeserializeOwned + Default {
    fn as_ref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T> AsMut<Vec<T>> for RepeatingValues<T>
where T: Serialize + DeserializeOwned + Default {
    fn as_mut(&mut self) -> &mut Vec<T> {
        &mut self.0
    }
}

impl<'de, T> Deserialize<'de> for RepeatingValues<T>
where T: Serialize + DeserializeOwned + Default {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<RepeatingValues<T>, D::Error> {
        let temp = <Cow<'_, str> as Deserialize>::deserialize(deserializer).map_err(serde::de::Error::custom)?;
        let (len, parts) = if let Some(p) = temp.find('\u{1}') {
            (&temp[0..p], &temp[(p + 1)..])
        }
        else {
            (temp.as_ref(), "")
        };
        let n = len.parse().map_err(serde::de::Error::custom)?;
        let parts = crate::from_str::<Vec<(Cow<'_, str>, Cow<'_, str>)>>(parts).map_err(serde::de::Error::custom)?;
        let pieces = parts.into_iter().fold(Vec::with_capacity(n), |mut p: Vec<Vec<(Cow<'_, str>, Cow<'_, str>)>>, part| {
            // contains
            if p.is_empty() || p[p.len() - 1].iter().any(|(k, _)| *k == part.0) {
                p.push(vec![part]);
            }
            else {
                let k = p.len() - 1;
                p[k].push(part);
            }
            p
        });
        if pieces.len() != n {
            return Err(serde::de::Error::custom(format!("Got {} elements but expected {}", pieces.len(), n)));
        }
        let mut inner = Vec::with_capacity(n);
        for piece in pieces {
            let mut s_piece = String::new();
            for (k, v) in piece {
                s_piece += k.as_ref();
                s_piece += "=";
                s_piece += v.as_ref();
                s_piece += "\u{1}";
            }
            inner.push(crate::from_str(&s_piece).map_err(serde::de::Error::custom)?);
        }
        Ok(RepeatingValues(inner))
    }
}

impl<T> Serialize for RepeatingValues<T>
where T: Serialize + DeserializeOwned + Default {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut temp = vec![self.0.len().to_string()];
        for t in &self.0 {
            let mut s = crate::to_string(t).map_err(serde::ser::Error::custom)?;
            if s.ends_with('\u{1}') {
                // avoid double separator
                s.truncate(s.len() - 1);
            }
            temp.push(s);
        }
        temp.join("\u{1}").serialize(serializer)
    }
}
