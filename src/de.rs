//! Deserialization support for the FiX format.

use std::borrow::Cow;
use std::io::Read;

use serde::de::value::MapDeserializer;
use serde::de::Error as de_Error;
use serde::de::{self, IntoDeserializer};
use serde::forward_to_deserialize_any;

#[doc(inline)]
pub use serde::de::value::Error;

struct Parser<'de, T: Iterator<Item=&'de [u8]> + Clone> {
    inner: T,
}

impl<'de, T: Iterator<Item=&'de [u8]> + Clone> Parser<'de, T> {
    fn starts_with(value: &[u8], next_fields: &[&[u8]]) -> Option<usize> {
        for field in next_fields {
            if value.starts_with(field) {
                return Some(field.len());
            }
        }
        None
    }
    fn extract_length(&mut self, value: Cow<'de, str>, len: usize, next_fields: &[&[u8]]) -> Cow<'de, str> {
        let backup = self.inner.clone();
        let mut rollback = false;
        if let Some(next) = self.inner.next() {
            if let Some(l) = Self::starts_with(next, next_fields) {
                // buffer next field data
                let mut append = next.to_vec();
                while append.len() < len + l {
                    append.push(1);// b'\u{1}' = 1
                    if append.len() < len + l {
                        match self.inner.next() {
                            Some(bytes) => {
                                for byte in bytes {
                                    append.push(*byte);
                                }
                            },
                            None => {
                                // malformed message
                                rollback = true;
                                break;
                            },
                        }
                    }
                    else {
                        // reached end with the divider
                        break;
                    }
                }

                if !rollback {
                    // build new value
                    let mut owned_value = value.into_owned();
                    owned_value.push('\u{1}');
                    owned_value += &String::from_utf8_lossy(&append);
                    return Cow::Owned(owned_value);
                }
            }
            else {
                rollback = true;
            }
        }

        if rollback {
            self.inner = backup;
        }

        value
    }
    fn extract_fields(&mut self, value: Cow<'de, str>, next_fields: &[&[u8]]) -> Cow<'de, str> {
        let mut rollback = false;
        let mut backup;
        let mut owned_value = value.into_owned();
        loop {
            backup = self.inner.clone();
            if let Some(next) = self.inner.next() {
                if Self::starts_with(next, next_fields).is_some() {
                    owned_value.push('\u{1}');
                    owned_value += &String::from_utf8_lossy(next);
                }
                else {
                    rollback = true;
                    break;
                }
            }
            else {
                break;
            }
        }

        if rollback {
            self.inner = backup;
        }

        Cow::Owned(owned_value)
    }
}

impl<'de, T: Iterator<Item=&'de [u8]> + Clone> Iterator for Parser<'de, T> {
    type Item = (Cow<'de, str>, Cow<'de, str>);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(a) = self.inner.next() {
            if a.is_empty() {
                return self.next();
            }

            let (key, mut value) = match a.iter().position(|b| b == &b'=') {
                Some(i) => (String::from_utf8_lossy(&a[0..i]), String::from_utf8_lossy(&a[(i + 1)..])),
                None => (String::from_utf8_lossy(&a[0..]), Cow::Borrowed("")),
            };
            // special fields who needs to be grouped
            match &key as &str {
                "90" => {// SecureDataLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be SecureData (91)
                        value = self.extract_length(value, len, &[b"91="]);
                    }
                },
                "93" => {// SignatureLength
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be Signature (89)
                        value = self.extract_length(value, len, &[b"89="]);
                    }
                },
                "95" => {// RawDataLength
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be RawData (96)
                        value = self.extract_length(value, len, &[b"96="]);
                    }
                },
                "212" => {// XmlDataLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be XmlData (213)
                        value = self.extract_length(value, len, &[b"213="]);
                    }
                },
                "354" => {// EncodedTextLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedText (355)
                        value = self.extract_length(value, len, &[b"355="]);
                    }
                },
                "1401" => {// EncryptedPasswordLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncryptedPassword (1402)
                        value = self.extract_length(value, len, &[b"1402="]);
                    }
                },
                "1403" => {// EncryptedNewPasswordLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncryptedNewPassword (1404)
                        value = self.extract_length(value, len, &[b"1404="]);
                    }
                },
                "384" => {// NoMsgTypes
                    // next fields can be RefMsgType (372), MsgDirection (385), RefApplVerID (1130), RefApplExtID (1406), RefCstmApplVerID (1131) and DefaultVerIndicator (1410)
                    value = self.extract_fields(value, &[b"372=", b"385=", b"1130=", b"1406=", b"1131=", b"1410="]);
                },
                "627" => {// NoHops
                    // next fields can be HopCompID (628), HopSendingTime (629) and HopRefID (630)
                    value = self.extract_fields(value, &[b"628=", b"629=", b"630="]);
                },
                _=> {},
            }
            Some((key, value))
        }
        else {
            None
        }
    }
}

fn parse<'de>(input: &'de [u8], check: bool) -> Result<impl Iterator<Item=(Cow<'de, str>, Cow<'de, str>)>, Error> {
    if check {
        if let Some((pos, _)) = input.windows(4).enumerate().find(|(_, b)| *b == "\u{1}10=".as_bytes()) {
            let sum: usize = input[0..pos].iter().map(|b| *b as usize).sum();
            if &input[pos..] != format!("\u{1}10={:03}\u{1}", sum % crate::CHECKSUM_MOD).as_bytes() {
                return Err(Error::custom("Mismatching checksum"));
            }
        }
        else {
            return Err(Error::custom("Malformed message"));
        }
    }
    Ok(Parser {
        inner: input.split(|b| *b == 1)// b'\u{1}' = 1
    })
}

fn _from_bytes<'de, T>(input: &'de [u8], check: bool) -> Result<T, Error>
where
    T: de::Deserialize<'de>,
{
    T::deserialize(Deserializer::new(parse(input, check)?))
}

/// Deserializes a FiX value from a `&[u8]`.
///
/// ```
/// let meal = vec![
///     ("bread".to_owned(), "baguette".to_owned()),
///     ("cheese".to_owned(), "comté".to_owned()),
///     ("meat".to_owned(), "ham".to_owned()),
///     ("fat".to_owned(), "butter".to_owned()),
/// ];
///
/// assert_eq!(
///     serde_fix::from_bytes::<Vec<(String, String)>>("bread=baguette\u{1}cheese=comté\u{1}meat=ham\u{1}fat=butter\u{1}".as_bytes()),
///     Ok(meal));
/// ```
pub fn from_bytes<'de, T>(input: &'de [u8]) -> Result<T, Error>
where
    T: de::Deserialize<'de>,
{
    _from_bytes(input, false)
}

/// Deserializes a FiX value from a `&[u8]`, with checksum
///
/// ```
/// let meal = vec![
///     ("bread".to_owned(), "baguette".to_owned()),
///     ("cheese".to_owned(), "comté".to_owned()),
///     ("meat".to_owned(), "ham".to_owned()),
///     ("fat".to_owned(), "butter".to_owned()),
/// ];
///
/// assert_eq!(
///     serde_fix::from_bytes::<Vec<(String, String)>>("bread=baguette\u{1}cheese=comté\u{1}meat=ham\u{1}fat=butter\u{1}".as_bytes()),
///     Ok(meal));
/// ```
pub fn from_bytes_checked<'de, T>(input: &'de [u8]) -> Result<T, Error>
where
    T: de::Deserialize<'de>,
{
    _from_bytes(input, true)
}

/// Deserializes a FiX value from a `&str`.
///
/// ```
/// let meal = vec![
///     ("bread".to_owned(), "baguette".to_owned()),
///     ("cheese".to_owned(), "comté".to_owned()),
///     ("meat".to_owned(), "ham".to_owned()),
///     ("fat".to_owned(), "butter".to_owned()),
/// ];
///
/// assert_eq!(
///     serde_fix::from_str::<Vec<(String, String)>>(
///         "bread=baguette\u{1}cheese=comté\u{1}meat=ham\u{1}fat=butter\u{1}"),
///     Ok(meal));
/// ```
pub fn from_str<'de, T>(input: &'de str) -> Result<T, Error>
where
    T: de::Deserialize<'de>,
{
    _from_bytes(input.as_bytes(), false)
}

/// Deserializes a FiX value from a `&str`, with checksum
///
/// ```
/// let meal = vec![
///     ("bread".to_owned(), "baguette".to_owned()),
///     ("cheese".to_owned(), "comté".to_owned()),
///     ("meat".to_owned(), "ham".to_owned()),
///     ("fat".to_owned(), "butter".to_owned()),
/// ];
///
/// assert_eq!(
///     serde_fix::from_str::<Vec<(String, String)>>(
///         "bread=baguette\u{1}cheese=comté\u{1}meat=ham\u{1}fat=butter\u{1}"),
///     Ok(meal));
/// ```
pub fn from_str_checked<'de, T>(input: &'de str) -> Result<T, Error>
where
    T: de::Deserialize<'de>,
{
    _from_bytes(input.as_bytes(), true)
}

/// Convenience function that reads all bytes from `reader` and deserializes
/// them with `from_bytes`.
pub fn from_reader<T, R>(mut reader: R) -> Result<T, Error>
where
    T: de::DeserializeOwned,
    R: Read,
{
    let mut buf = vec![];
    reader.read_to_end(&mut buf).map_err(|e| {
        de::Error::custom(format_args!("could not read input: {}", e))
    })?;
    _from_bytes(&buf, false)
}

/// Convenience function that reads all bytes from `reader` and deserializes
/// them with `from_bytes`, with checksum
pub fn from_reader_checked<T, R>(mut reader: R) -> Result<T, Error>
where
    T: de::DeserializeOwned,
    R: Read,
{
    let mut buf = vec![];
    reader.read_to_end(&mut buf).map_err(|e| {
        de::Error::custom(format_args!("could not read input: {}", e))
    })?;
    _from_bytes(&buf, true)
}

/// A deserializer for the FiX format.
///
/// * Supported top-level outputs are structs, maps and sequences of pairs,
///   with or without a given length.
///
/// * Main `deserialize` methods defers to `deserialize_map`.
///
/// * Everything else but `deserialize_seq` and `deserialize_seq_fixed_size`
///   defers to `deserialize`.
pub struct Deserializer<'de, T: Iterator<Item=(Cow<'de, str>, Cow<'de, str>)>> {
    inner: MapDeserializer<'de, PartIterator<'de, T>, Error>,
}

impl<'de, T: Iterator<Item=(Cow<'de, str>, Cow<'de, str>)>> Deserializer<'de, T> {
    /// Returns a new `Deserializer`.
    pub fn new(parser: T) -> Self {
        Deserializer {
            inner: MapDeserializer::new(PartIterator(parser)),
        }
    }
}

impl<'de, T: Iterator<Item=(Cow<'de, str>, Cow<'de, str>)>> de::Deserializer<'de> for Deserializer<'de, T> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_map(self.inner)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(self.inner)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.inner.end()?;
        visitor.visit_unit()
    }

    forward_to_deserialize_any! {
        bool
        u8
        u16
        u32
        u64
        i8
        i16
        i32
        i64
        f32
        f64
        char
        str
        string
        option
        bytes
        byte_buf
        unit_struct
        newtype_struct
        tuple_struct
        struct
        identifier
        tuple
        enum
        ignored_any
    }
}

struct PartIterator<'de, T: Iterator<Item=(Cow<'de, str>, Cow<'de, str>)>>(T);

impl<'de, T: Iterator<Item=(Cow<'de, str>, Cow<'de, str>)>> Iterator for PartIterator<'de, T> {
    type Item = (Part<'de>, Part<'de>);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(k, v)| (Part(k), Part(v)))
    }
}

struct Part<'de>(Cow<'de, str>);

impl<'de> IntoDeserializer<'de> for Part<'de> {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

macro_rules! forward_parsed_value {
    ($($ty:ident => $method:ident,)*) => {
        $(
            fn $method<V>(self, visitor: V) -> Result<V::Value, Self::Error>
                where V: de::Visitor<'de>
            {
                match self.0.parse::<$ty>() {
                    Ok(val) => val.into_deserializer().$method(visitor),
                    Err(e) => Err(de::Error::custom(e))
                }
            }
        )*
    }
}

impl<'de> de::Deserializer<'de> for Part<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.0 {
            Cow::Borrowed(value) => visitor.visit_borrowed_str(value),
            Cow::Owned(value) => visitor.visit_string(value),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_enum(ValueEnumAccess(self.0))
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    forward_to_deserialize_any! {
        char
        str
        string
        unit
        bytes
        byte_buf
        unit_struct
        tuple_struct
        struct
        identifier
        tuple
        ignored_any
        seq
        map
    }

    forward_parsed_value! {
        bool => deserialize_bool,
        u8 => deserialize_u8,
        u16 => deserialize_u16,
        u32 => deserialize_u32,
        u64 => deserialize_u64,
        i8 => deserialize_i8,
        i16 => deserialize_i16,
        i32 => deserialize_i32,
        i64 => deserialize_i64,
        f32 => deserialize_f32,
        f64 => deserialize_f64,
    }
}

struct ValueEnumAccess<'de>(Cow<'de, str>);

impl<'de> de::EnumAccess<'de> for ValueEnumAccess<'de> {
    type Error = Error;
    type Variant = UnitOnlyVariantAccess;

    fn variant_seed<V>(
        self,
        seed: V,
    ) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let variant = seed.deserialize(self.0.into_deserializer())?;
        Ok((variant, UnitOnlyVariantAccess))
    }
}

struct UnitOnlyVariantAccess;

impl<'de> de::VariantAccess<'de> for UnitOnlyVariantAccess {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        Err(Error::custom("expected unit variant"))
    }

    fn tuple_variant<V>(
        self,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::custom("expected unit variant"))
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::custom("expected unit variant"))
    }
}
