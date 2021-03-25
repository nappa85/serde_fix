//! Serialization support for the FiX format.

mod key;
mod pair;
mod part;
mod value;
mod encoder;

// use form_urlencoded::Serializer as UrlEncodedSerializer;
// use form_urlencoded::Target as UrlEncodedTarget;
use serde::ser;
use std::borrow::Cow;
use std::error;
use std::fmt;
use std::str;

/// Serializes a value into a FiX `String` buffer.
///
/// ```
/// let meal = &[
///     ("bread", "baguette"),
///     ("cheese", "comté"),
///     ("meat", "ham"),
///     ("fat", "butter"),
/// ];
///
/// assert_eq!(
///     serde_fix::to_string(meal),
///     Ok("bread=baguette\u{1}cheese=comté\u{1}meat=ham\u{1}fat=butter\u{1}".to_owned()));
/// ```
pub fn to_string<T: ser::Serialize>(input: T) -> Result<String, Error> {
    let mut encoder = encoder::Encoder::new();
    input.serialize(Serializer::new(&mut encoder))?;
    Ok(encoder.finish(false))
}

/// Serializes a value into a FiX `String` buffer, with calculated checksum
///
/// ```
/// let meal = &[
///     ("bread", "baguette"),
///     ("cheese", "comté"),
///     ("meat", "ham"),
///     ("fat", "butter"),
/// ];
///
/// assert_eq!(
///     serde_fix::to_string_checksum(meal),
///     Ok("bread=baguette\u{1}cheese=comté\u{1}meat=ham\u{1}fat=butter\u{1}10=146\u{1}".to_owned()));
/// ```
pub fn to_string_checksum<T: ser::Serialize>(input: T) -> Result<String, Error> {
    let mut encoder = encoder::Encoder::new();
    input.serialize(Serializer::new(&mut encoder))?;
    Ok(encoder.finish(true))
}

/// A serializer for the FiX format.
///
/// * Supported top-level inputs are structs, maps and sequences of pairs,
///   with or without a given length.
///
/// * Supported keys and values are integers, bytes (if convertible to strings),
///   unit structs and unit variants.
///
/// * Newtype structs defer to their inner values.
pub struct Serializer<'output> {
    encoder: &'output mut encoder::Encoder,
}

impl<'output> Serializer<'output> {
    /// Returns a new `Serializer`.
    pub fn new(
        encoder: &'output mut encoder::Encoder,
    ) -> Self {
        Serializer { encoder }
    }
}

/// Errors returned during serializing to FiX.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Custom(Cow<'static, str>),
    Utf8(str::Utf8Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Custom(ref msg) => msg.fmt(f),
            Error::Utf8(ref err) => write!(f, "invalid UTF-8: {}", err),
        }
    }
}

impl error::Error for Error {
    /// The lower-level cause of this error, in the case of a `Utf8` error.
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::Custom(_) => None,
            Error::Utf8(ref err) => Some(err),
        }
    }

    /// The lower-level source of this error, in the case of a `Utf8` error.
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Custom(_) => None,
            Error::Utf8(ref err) => Some(err),
        }
    }
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Custom(format!("{}", msg).into())
    }
}

/// Sequence serializer.
pub struct SeqSerializer<'output> {
    encoder: &'output mut encoder::Encoder,
}

/// Tuple serializer.
///
/// Mostly used for arrays.
pub struct TupleSerializer<'output> {
    encoder: &'output mut encoder::Encoder,
}

/// Tuple struct serializer.
///
/// Never instantiated, tuple structs are not supported.
pub struct TupleStructSerializer<'output> {
    inner: ser::Impossible<&'output mut encoder::Encoder, Error>,
}

/// Tuple variant serializer.
///
/// Never instantiated, tuple variants are not supported.
pub struct TupleVariantSerializer<'output> {
    inner: ser::Impossible<&'output mut encoder::Encoder, Error>,
}

/// Map serializer.
pub struct MapSerializer<'output> {
    encoder: &'output mut encoder::Encoder,
    key: Option<Cow<'static, str>>,
}

/// Struct serializer.
pub struct StructSerializer<'output> {
    encoder: &'output mut encoder::Encoder,
}

/// Struct variant serializer.
///
/// Never instantiated, struct variants are not supported.
pub struct StructVariantSerializer<'output> {
    inner: ser::Impossible<&'output mut encoder::Encoder, Error>,
}

impl<'output> ser::Serializer for Serializer<'output> {
    type Ok = &'output mut encoder::Encoder;
    type Error = Error;
    type SerializeSeq = SeqSerializer<'output>;
    type SerializeTuple = TupleSerializer<'output>;
    type SerializeTupleStruct = TupleStructSerializer<'output>;
    type SerializeTupleVariant =
        TupleVariantSerializer<'output>;
    type SerializeMap = MapSerializer<'output>;
    type SerializeStruct = StructSerializer<'output>;
    type SerializeStructVariant =
        StructVariantSerializer<'output>;

    /// Returns an error.
    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Returns an error.
    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Returns an error.
    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Returns an error.
    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Returns an error.
    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Returns an error.
    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Returns an error.
    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Returns an error.
    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Returns an error.
    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Returns an error.
    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Returns an error.
    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Returns an error.
    fn serialize_char(self, _v: char) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Returns an error.
    fn serialize_str(self, _value: &str) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Returns an error.
    fn serialize_bytes(self, _value: &[u8]) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Returns `Ok`.
    fn serialize_unit(self) -> Result<Self::Ok, Error> {
        Ok(self.encoder)
    }

    /// Returns `Ok`.
    fn serialize_unit_struct(
        self,
        _name: &'static str,
    ) -> Result<Self::Ok, Error> {
        Ok(self.encoder)
    }

    /// Returns an error.
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Serializes the inner value, ignoring the newtype name.
    fn serialize_newtype_struct<T: ?Sized + ser::Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Error> {
        value.serialize(self)
    }

    /// Returns an error.
    fn serialize_newtype_variant<T: ?Sized + ser::Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Returns `Ok`.
    fn serialize_none(self) -> Result<Self::Ok, Error> {
        Ok(self.encoder)
    }

    /// Serializes the given value.
    fn serialize_some<T: ?Sized + ser::Serialize>(
        self,
        value: &T,
    ) -> Result<Self::Ok, Error> {
        value.serialize(self)
    }

    /// Serialize a sequence, given length (if any) is ignored.
    fn serialize_seq(
        self,
        _len: Option<usize>,
    ) -> Result<Self::SerializeSeq, Error> {
        Ok(SeqSerializer {
            encoder: self.encoder,
        })
    }

    /// Returns an error.
    fn serialize_tuple(
        self,
        _len: usize,
    ) -> Result<Self::SerializeTuple, Error> {
        Ok(TupleSerializer {
            encoder: self.encoder,
        })
    }

    /// Returns an error.
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Error> {
        Err(Error::top_level())
    }

    /// Returns an error.
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Error> {
        Err(Error::top_level())
    }

    /// Serializes a map, given length is ignored.
    fn serialize_map(
        self,
        _len: Option<usize>,
    ) -> Result<Self::SerializeMap, Error> {
        Ok(MapSerializer {
            encoder: self.encoder,
            key: None,
        })
    }

    /// Serializes a struct, given length is ignored.
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Error> {
        Ok(StructSerializer {
            encoder: self.encoder,
        })
    }

    /// Returns an error.
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Error> {
        Err(Error::top_level())
    }
}

impl<'output> ser::SerializeSeq for SeqSerializer<'output> {
    type Ok = &'output mut encoder::Encoder;
    type Error = Error;

    fn serialize_element<T: ?Sized + ser::Serialize>(
        &mut self,
        value: &T,
    ) -> Result<(), Error> {
        value.serialize(pair::PairSerializer::new(self.encoder))
    }

    fn end(self) -> Result<Self::Ok, Error> {
        Ok(self.encoder)
    }
}

impl<'output> ser::SerializeTuple for TupleSerializer<'output> {
    type Ok = &'output mut encoder::Encoder;
    type Error = Error;

    fn serialize_element<T: ?Sized + ser::Serialize>(
        &mut self,
        value: &T,
    ) -> Result<(), Error> {
        value.serialize(pair::PairSerializer::new(self.encoder))
    }

    fn end(self) -> Result<Self::Ok, Error> {
        Ok(self.encoder)
    }
}

impl<'output> ser::SerializeTupleStruct for TupleStructSerializer<'output> {
    type Ok = &'output mut encoder::Encoder;
    type Error = Error;

    fn serialize_field<T: ?Sized + ser::Serialize>(
        &mut self,
        value: &T,
    ) -> Result<(), Error> {
        self.inner.serialize_field(value)
    }

    fn end(self) -> Result<Self::Ok, Error> {
        self.inner.end()
    }
}

impl<'output> ser::SerializeTupleVariant for TupleVariantSerializer<'output> {
    type Ok = &'output mut encoder::Encoder;
    type Error = Error;

    fn serialize_field<T: ?Sized + ser::Serialize>(
        &mut self,
        value: &T,
    ) -> Result<(), Error> {
        self.inner.serialize_field(value)
    }

    fn end(self) -> Result<Self::Ok, Error> {
        self.inner.end()
    }
}

impl<'output> ser::SerializeMap for MapSerializer<'output> {
    type Ok = &'output mut encoder::Encoder;
    type Error = Error;

    fn serialize_entry<
        K: ?Sized + ser::Serialize,
        V: ?Sized + ser::Serialize,
    >(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<(), Error> {
        let key_sink = key::KeySink::new(|key| {
            let value_sink = value::ValueSink::new(self.encoder, &key);
            value.serialize(part::PartSerializer::new(value_sink))?;
            self.key = None;
            Ok(())
        });
        let entry_serializer = part::PartSerializer::new(key_sink);
        key.serialize(entry_serializer)
    }

    fn serialize_key<T: ?Sized + ser::Serialize>(
        &mut self,
        key: &T,
    ) -> Result<(), Error> {
        let key_sink = key::KeySink::new(|key| Ok(key.into()));
        let key_serializer = part::PartSerializer::new(key_sink);
        self.key = Some(key.serialize(key_serializer)?);
        Ok(())
    }

    fn serialize_value<T: ?Sized + ser::Serialize>(
        &mut self,
        value: &T,
    ) -> Result<(), Error> {
        {
            let key = self.key.as_ref().ok_or_else(Error::no_key)?;
            let value_sink = value::ValueSink::new(self.encoder, &key);
            value.serialize(part::PartSerializer::new(value_sink))?;
        }
        self.key = None;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Error> {
        Ok(self.encoder)
    }
}

impl<'output> ser::SerializeStruct for StructSerializer<'output> {
    type Ok = &'output mut encoder::Encoder;
    type Error = Error;

    fn serialize_field<T: ?Sized + ser::Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Error> {
        let value_sink = value::ValueSink::new(self.encoder, key);
        value.serialize(part::PartSerializer::new(value_sink))
    }

    fn end(self) -> Result<Self::Ok, Error> {
        Ok(self.encoder)
    }
}

impl<'output> ser::SerializeStructVariant for StructVariantSerializer<'output> {
    type Ok = &'output mut encoder::Encoder;
    type Error = Error;

    fn serialize_field<T: ?Sized + ser::Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Error> {
        self.inner.serialize_field(key, value)
    }

    fn end(self) -> Result<Self::Ok, Error> {
        self.inner.end()
    }
}

impl Error {
    fn top_level() -> Self {
        let msg = "top-level serializer supports only maps and structs";
        Error::Custom(msg.into())
    }

    fn no_key() -> Self {
        let msg = "tried to serialize a value before serializing key";
        Error::Custom(msg.into())
    }
}
