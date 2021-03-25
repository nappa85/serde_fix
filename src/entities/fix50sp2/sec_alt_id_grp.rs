use std::borrow::Cow;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Default)]
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
        let mut temp = Vec::new();
        temp.push(self.len.to_string());
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

#[derive(Clone, Debug, Default)]
pub struct SecAltID {
    // #[serde(rename = "455")]
    pub security_alt_id: Option<String>,
    // #[serde(rename = "456")]
    pub security_alt_id_source: Option<SecurityAltIDSource>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SecurityAltIDSource {
    /// CUSIP
    #[serde(rename = "1")]
    CUSIP,
    /// SEDOL
    #[serde(rename = "2")]
    SEDOL,
    /// QUIK
    #[serde(rename = "3")]
    QUIK,
    /// ISIN
    #[serde(rename = "4")]
    ISIN,
    /// RIC
    #[serde(rename = "5")]
    RIC,
    /// ISO Currency Code
    #[serde(rename = "6")]
    ISOCurrencyCode,
    /// ISO Country Code
    #[serde(rename = "7")]
    ISOCountryCode,
    /// Exchange Symbol
    #[serde(rename = "8")]
    ExchangeSymbol,
    /// Consolidated Tape Association (CTA) Symbol (SIAC CTS/CQS line format)
    #[serde(rename = "9")]
    CTASymbol,
    /// Bloomberg Symbol
    #[serde(rename = "A")]
    BloombergSymbol,
    /// Wertpapier
    #[serde(rename = "B")]
    Wertpapier,
    /// Dutch
    #[serde(rename = "C")]
    Dutch,
    /// Valoren
    #[serde(rename = "D")]
    Valoren,
    /// Sicovam
    #[serde(rename = "E")]
    Sicovam,
    /// Belgian
    #[serde(rename = "F")]
    Belgian,
    /// "Common" (Clearstream and Euroclear)
    #[serde(rename = "G")]
    Common,
    /// Clearing House / Clearing Organization
    #[serde(rename = "H")]
    ClearingHouse,
    /// ISDA/FpML Product Specification
    #[serde(rename = "I")]
    ISDAFpMLProductSpecification,
    /// Option Price Reporting Authority
    #[serde(rename = "J")]
    OptionPriceReportingAuthority,
    /// ISDA/FpML Product URL (URL in SecurityID)
    #[serde(rename = "K")]
    ISDAFpMLProductURL,
    /// Letter of Credit
    #[serde(rename = "L")]
    LetterOfCredit,
    /// Marketplace-assigned Identifier
    #[serde(rename = "M")]
    MarketplaceAssignedIdentifier,
    /// Markit RED entity CLIP
    #[serde(rename = "N")]
    MarkitREDEntityCLIP,
    /// Markit RED pair CLIP
    #[serde(rename = "P")]
    MarkitREDPairCLIP,
    /// CFTC commodity code
    #[serde(rename = "Q")]
    CFTCCommodityCode,
    /// ISDA Commodity Reference Price
    #[serde(rename = "R")]
    ISDACommodityReferencePrice,
    /// Financial Instrument Global Identifier
    #[serde(rename = "S")]
    FinancialInstrumentGlobalIdentifier,
    /// Legal Entity Identifier
    #[serde(rename = "T")]
    LegalEntityIdentifier,
    /// Synthetic
    #[serde(rename = "U")]
    Synthetic,
    /// Fidessa Instrument Mnemonic (FIM)
    #[serde(rename = "V")]
    FidessaInstrumentMnemonic,
    /// Index name
    #[serde(rename = "W")]
    IndexName,
    /// Uniform Symbol (UMTF Symbol)
    #[serde(rename = "X")]
    UniformSymbol,
}
