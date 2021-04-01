use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// get string representation length without casting
const fn get_strlen(n: u32) -> usize {
    let mut n = n;
    let mut l = 1;
    while n >= 10 {
        n /= 10;
        l += 1;
    }
    l
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct EncodedText<const N: u32>(pub String);

impl<const N: u32> AsRef<String> for EncodedText<N> {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl<const N: u32> AsMut<String> for EncodedText<N> {
    fn as_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

impl<'de, const N: u32> Deserialize<'de> for EncodedText<N> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let a = <Cow<'_, str> as Deserialize>::deserialize(deserializer)?;

        let key_len = get_strlen(N);
        let (field1, field2) = match a.find('\u{1}') {
            Some(i) => (&a[0..i], if a.len() > i + (key_len + 2) { &a[(i + key_len + 2)..] } else { "" }),
            None => (&a[0..], ""),
        };

        let len: usize = field1.parse().map_err(serde::de::Error::custom)?;
        if field2.len() == len {
            Ok(EncodedText(field2.to_owned()))
        }
        else {
            Err(serde::de::Error::custom(format!("Field {} length not matching, got {} but expected {}", N, field2.len(), len)))
        }
    }
}

impl<const N: u32> Serialize for EncodedText<N> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let temp = format!("{}\u{1}{}={}", self.0.len(), N, self.0);
        temp.serialize(serializer)
    }
}
