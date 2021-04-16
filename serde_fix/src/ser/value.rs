use crate::ser::part::{PartSerializer, Sink};
use crate::ser::Error;
use crate::ser::encoder;
// use form_urlencoded::Serializer as UrlEncodedSerializer;
// use form_urlencoded::Target as UrlEncodedTarget;
use serde::ser::Serialize;
use std::str;

pub struct ValueSink<'key, 'target> {
    urlencoder: &'target mut encoder::Encoder,
    key: &'key str,
}

impl<'key, 'target> ValueSink<'key, 'target> {
    pub fn new(
        urlencoder: &'target mut encoder::Encoder,
        key: &'key str,
    ) -> Self {
        ValueSink { urlencoder, key }
    }
}

impl<'key, 'target> Sink for ValueSink<'key, 'target> {
    type Ok = ();

    fn serialize_str(self, value: &str) -> Result<(), Error> {
        self.urlencoder.append_pair(self.key, value);
        Ok(())
    }

    fn serialize_static_str(self, value: &'static str) -> Result<(), Error> {
        self.serialize_str(value)
    }

    fn serialize_string(self, value: String) -> Result<(), Error> {
        self.serialize_str(&value)
    }

    fn serialize_none(self) -> Result<Self::Ok, Error> {
        Ok(())
    }

    fn serialize_some<T: ?Sized + Serialize>(
        self,
        value: &T,
    ) -> Result<Self::Ok, Error> {
        value.serialize(PartSerializer::new(self))
    }

    fn unsupported(self) -> Error {
        Error::Custom("unsupported value".into())
    }
}
