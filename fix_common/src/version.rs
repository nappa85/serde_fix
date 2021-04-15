
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub enum FixVersion<const V: u8> {
    #[serde(rename = "FIX.4.0")]
    FIX40,
    #[serde(rename = "FIX.4.1")]
    FIX41,
    #[serde(rename = "FIX.4.2")]
    FIX42,
    #[serde(rename = "FIX.4.3")]
    FIX43,
    #[serde(rename = "FIX.4.4")]
    FIX44,
    #[serde(rename = "FIXT.1.1")]
    FIXT11,
}

impl<const V: u8> Default for FixVersion<V> {
    fn default() -> Self {
        match V {
            0 => FixVersion::FIX40,
            1 => FixVersion::FIX41,
            2 => FixVersion::FIX42,
            3 => FixVersion::FIX43,
            4 => FixVersion::FIX44,
            5 => FixVersion::FIXT11,
            _ => unimplemented!(),
        }
    }
}

// impl<const V1: u8, const V2: u8> From<FixVersion<V2>> for FixVersion<V1> {
//     fn from(other: FixVersion<V2>) -> Self {
//         match other {
impl<const V: u8> FixVersion<V> {
    pub fn convert<const V2: u8>(&self) -> FixVersion<V2> {
        match self {
            FixVersion::FIX40 => FixVersion::FIX40,
            FixVersion::FIX41 => FixVersion::FIX41,
            FixVersion::FIX42 => FixVersion::FIX42,
            FixVersion::FIX43 => FixVersion::FIX43,
            FixVersion::FIX44 => FixVersion::FIX44,
            FixVersion::FIXT11 => FixVersion::FIXT11,
        }
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub enum ApplVerID<const V: u8> {
    /// FIX27
    #[serde(rename = "0")]
    FIX27,
    /// FIX30
    #[serde(rename = "1")]
    FIX30,
    /// FIX40
    #[serde(rename = "2")]
    FIX40,
    /// FIX41
    #[serde(rename = "3")]
    FIX41,
    /// FIX42
    #[serde(rename = "4")]
    FIX42,
    /// FIX43
    #[serde(rename = "5")]
    FIX43,
    /// FIX44
    #[serde(rename = "6")]
    FIX44,
    /// FIX50
    #[serde(rename = "7")]
    FIX50,
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1,
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2,
    /// FIXLatest
    #[serde(rename = "10")]
    FIXLatest,
}

impl<const V: u8> Default for ApplVerID<V> {
    fn default() -> Self {
        match V {
            0 => ApplVerID::FIX27,
            1 => ApplVerID::FIX30,
            2 => ApplVerID::FIX40,
            3 => ApplVerID::FIX41,
            4 => ApplVerID::FIX42,
            5 => ApplVerID::FIX43,
            6 => ApplVerID::FIX44,
            7 => ApplVerID::FIX50,
            8 => ApplVerID::FIX50SP1,
            9 => ApplVerID::FIX50SP2,
            10 => ApplVerID::FIXLatest,
            _ => unimplemented!(),
        }
    }
}

// impl<const V1: u8, const V2: u8> From<ApplVerID<V2>> for ApplVerID<V1> {
//     fn from(other: ApplVerID<V2>) -> Self {
//         match other {
impl<const V: u8> ApplVerID<V> {
    pub fn convert<const V2: u8>(&self) -> ApplVerID<V2> {
        match self {
            ApplVerID::FIX27 => ApplVerID::FIX27,
            ApplVerID::FIX30 => ApplVerID::FIX30,
            ApplVerID::FIX40 => ApplVerID::FIX40,
            ApplVerID::FIX41 => ApplVerID::FIX41,
            ApplVerID::FIX42 => ApplVerID::FIX42,
            ApplVerID::FIX43 => ApplVerID::FIX43,
            ApplVerID::FIX44 => ApplVerID::FIX44,
            ApplVerID::FIX50 => ApplVerID::FIX50,
            ApplVerID::FIX50SP1 => ApplVerID::FIX50SP1,
            ApplVerID::FIX50SP2 => ApplVerID::FIX50SP2,
            ApplVerID::FIXLatest => ApplVerID::FIXLatest,
        }
    }
}
