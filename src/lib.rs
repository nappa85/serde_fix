//! `FiX` meets Serde

#![warn(unused_extern_crates)]

pub mod de;
pub mod ser;
pub mod entities;

const CHECKSUM_MOD: usize = 256;

#[doc(inline)]
pub use crate::de::{from_bytes, from_bytes_checked, from_reader, from_reader_checked, from_str, from_str_checked, Deserializer};
#[doc(inline)]
pub use crate::ser::{to_string, to_string_checked, Serializer};
