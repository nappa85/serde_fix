//! Deserialization support for the FiX format.

use std::{borrow::Cow, usize};
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
    fn extract_fields(&mut self, value: Cow<'de, str>, next_fields: &[&str]) -> Cow<'de, str> {
        let mut rollback = false;
        let mut backup;
        let mut owned_value = value.into_owned();
        loop {
            backup = self.inner.clone();
            if let Some((next_key, next_value)) = self.next() {
                if next_fields.contains(&next_key.as_ref()) {
                    owned_value.push('\u{1}');
                    owned_value += next_key.as_ref();
                    owned_value.push('=');
                    owned_value += next_value.as_ref();
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
                "348" => {// EncodedIssuerLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedIssuer (349)
                        value = self.extract_length(value, len, &[b"349="]);
                    }
                },
                "350" => {// EncodedSecurityDescLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedSecurityDesc (351)
                        value = self.extract_length(value, len, &[b"351="]);
                    }
                },
                "352" => {// EncodedListExecInstLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedListExecInst (353)
                        value = self.extract_length(value, len, &[b"353="]);
                    }
                },
                "354" => {// EncodedTextLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedText (355)
                        value = self.extract_length(value, len, &[b"355="]);
                    }
                },
                "356" => {// EncodedSubjectLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedSubject (357)
                        value = self.extract_length(value, len, &[b"357="]);
                    }
                },
                "358" => {// EncodedHeadlineLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedHeadline (359)
                        value = self.extract_length(value, len, &[b"359="]);
                    }
                },
                "360" => {// EncodedAllocTextLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedAllocText (361)
                        value = self.extract_length(value, len, &[b"361="]);
                    }
                },
                "362" => {// EncodedUnderlyingIssuerLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedUnderlyingIssuer (363)
                        value = self.extract_length(value, len, &[b"363="]);
                    }
                },
                "364" => {// EncodedUnderlyingSecurityDescLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedUnderlyingSecurityDesc (365)
                        value = self.extract_length(value, len, &[b"365="]);
                    }
                },
                "445" => {// EncodedListStatusTextLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedListStatusText (446)
                        value = self.extract_length(value, len, &[b"446="]);
                    }
                },
                "618" => {// EncodedLegIssuerLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedLegIssuer (619)
                        value = self.extract_length(value, len, &[b"619="]);
                    }
                },
                "621" => {// EncodedLegSecurityDescLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedLegSecurityDesc (622)
                        value = self.extract_length(value, len, &[b"622="]);
                    }
                },
                "1184" => {// SecurityXMLLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be SecurityXML (1185)
                        value = self.extract_length(value, len, &[b"1185="]);
                    }
                },
                "1277" => {// DerivativeEncodedIssuerLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be DerivativeEncodedIssuer (1278)
                        value = self.extract_length(value, len, &[b"1278="]);
                    }
                },
                "1280" => {// DerivativeEncodedSecurityDescLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be DerivativeEncodedSecurityDesc (1281)
                        value = self.extract_length(value, len, &[b"1281="]);
                    }
                },
                "1282" => {// DerivativeSecurityXMLLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be DerivativeSecurityXML (1283)
                        value = self.extract_length(value, len, &[b"1283="]);
                    }
                },
                "1397" => {// EncodedMktSegmDescLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedMktSegmDesc (1398)
                        value = self.extract_length(value, len, &[b"1398="]);
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
                "1468" => {// EncodedSecurityListDescLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedSecurityListDesc (1469)
                        value = self.extract_length(value, len, &[b"1469="]);
                    }
                },
                "1525" => {// EncodedDocumentationTextLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedDocumentationText (1527)
                        value = self.extract_length(value, len, &[b"1527="]);
                    }
                },
                "1578" => {// EncodedEventTextLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedEventText (1579)
                        value = self.extract_length(value, len, &[b"1579="]);
                    }
                },
                "1620" => {// InstrumentScopeEncodedSecurityDescLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be InstrumentScopeEncodedSecurityDesc (1621)
                        value = self.extract_length(value, len, &[b"1621="]);
                    }
                },
                "1664" => {// EncodedRejectTextLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedRejectText (1665)
                        value = self.extract_length(value, len, &[b"1665="]);
                    }
                },
                "1678" => {// EncodedOptionExpirationDescLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedOptionExpirationDesc (1697)
                        value = self.extract_length(value, len, &[b"1697="]);
                    }
                },
                "1733" => {// EncodedFirmAllocTextLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedFirmAllocText (1734)
                        value = self.extract_length(value, len, &[b"1734="]);
                    }
                },
                "1871" => {// LegSecurityXMLLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be LegSecurityXML (1872)
                        value = self.extract_length(value, len, &[b"1872="]);
                    }
                },
                "1874" => {// UnderlyingSecurityXMLLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be UnderlyingSecurityXML (1875)
                        value = self.extract_length(value, len, &[b"1875="]);
                    }
                },
                "2072" => {// EncodedUnderlyingEventTextLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedUnderlyingEventText (2073)
                        value = self.extract_length(value, len, &[b"2073="]);
                    }
                },
                "2074" => {// EncodedLegEventTextLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedLegEventText (2075)
                        value = self.extract_length(value, len, &[b"2075="]);
                    }
                },
                "2111" => {// EncodedAttachmentLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedAttachment (2112)
                        value = self.extract_length(value, len, &[b"2112="]);
                    }
                },
                "2179" => {// EncodedLegOptionExpirationDescLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedLegOptionExpirationDesc (2180)
                        value = self.extract_length(value, len, &[b"2180="]);
                    }
                },
                "2287" => {// EncodedUnderlyingOptionExpirationDescLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedUnderlyingOptionExpirationDesc (2288)
                        value = self.extract_length(value, len, &[b"2288="]);
                    }
                },
                "2351" => {// EncodedComplianceTextLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedComplianceText (2352)
                        value = self.extract_length(value, len, &[b"2352="]);
                    }
                },
                "2372" => {// EncodedTradeContinuationTextLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedTradeContinuationText (2371)
                        value = self.extract_length(value, len, &[b"2371="]);
                    }
                },
                "2481" => {// EncodedMDStatisticDescLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedMDStatisticDesc (2482)
                        value = self.extract_length(value, len, &[b"2482="]);
                    }
                },
                "2494" => {// EncodedLegDocumentationTextLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedLegDocumentationText (2493)
                        value = self.extract_length(value, len, &[b"2493="]);
                    }
                },
                "2522" => {// EncodedWarningTextLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedWarningText (2521)
                        value = self.extract_length(value, len, &[b"2521="]);
                    }
                },
                "2637" => {// EncodedMiscFeeSubTypeDescLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedMiscFeeSubTypeDesc (2638)
                        value = self.extract_length(value, len, &[b"2638="]);
                    }
                },
                "2651" => {// EncodedCommissionDescLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedCommissionDesc (2652)
                        value = self.extract_length(value, len, &[b"2652="]);
                    }
                },
                "2665" => {// EncodedAllocCommissionDescLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedAllocCommissionDesc (2666)
                        value = self.extract_length(value, len, &[b"2666="]);
                    }
                },
                "2715" => {// EncodedFinancialInstrumentFullNameLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedFinancialInstrumentFullName (2716)
                        value = self.extract_length(value, len, &[b"2716="]);
                    }
                },
                "2718" => {// EncodedLegFinancialInstrumentFullNameLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedLegFinancialInstrumentFullName (2719)
                        value = self.extract_length(value, len, &[b"2719="]);
                    }
                },
                "2721" => {// EncodedUnderlyingFinancialInstrumentFullNameLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedUnderlyingFinancialInstrumentFullName (2722)
                        value = self.extract_length(value, len, &[b"2722="]);
                    }
                },
                "2797" => {// EncodedMatchExceptionTextLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedMatchExceptionText (2798)
                        value = self.extract_length(value, len, &[b"2798="]);
                    }
                },
                "2802" => {// EncodedReplaceTextLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedReplaceText (2801)
                        value = self.extract_length(value, len, &[b"2801="]);
                    }
                },
                "2809" => {// EncodedCancelTextLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedCancelText (2808)
                        value = self.extract_length(value, len, &[b"2808="]);
                    }
                },
                "2815" => {// EncodedPostTradePaymentDescLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedPostTradePaymentDesc (2814)
                        value = self.extract_length(value, len, &[b"2814="]);
                    }
                },
                "40004" => {// EncodedAdditionalTermBondDescLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedAdditionalTermBondDesc (40005)
                        value = self.extract_length(value, len, &[b"40005="]);
                    }
                },
                "40008" => {// EncodedAdditionalTermBondIssuerLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedAdditionalTermBondIssuer (40009)
                        value = self.extract_length(value, len, &[b"40009="]);
                    }
                },
                "40978" => {// EncodedLegStreamTextLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedLegStreamText (40979)
                        value = self.extract_length(value, len, &[b"40979="]);
                    }
                },
                "40980" => {// EncodedLegProvisionTextLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedLegProvisionText (40981)
                        value = self.extract_length(value, len, &[b"40981="]);
                    }
                },
                "40982" => {// EncodedStreamTextLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedStreamText (40983)
                        value = self.extract_length(value, len, &[b"40983="]);
                    }
                },
                "40984" => {// EncodedPaymentTextLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedPaymentText (40985)
                        value = self.extract_length(value, len, &[b"40985="]);
                    }
                },
                "40986" => {// EncodedProvisionTextLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedProvisionText (40987)
                        value = self.extract_length(value, len, &[b"40987="]);
                    }
                },
                "40988" => {// EncodedUnderlyingStreamTextLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedUnderlyingStreamText (40989)
                        value = self.extract_length(value, len, &[b"40989="]);
                    }
                },
                "41083" => {// EncodedDeliveryStreamCycleDescLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedDeliveryStreamCycleDesc (41084)
                        value = self.extract_length(value, len, &[b"41084="]);
                    }
                },
                "41101" => {// EncodedMarketDisruptionFallbackUnderlierSecurityDescLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedMarketDisruptionFallbackUnderlierSecurityDesc (41102)
                        value = self.extract_length(value, len, &[b"41102="]);
                    }
                },
                "41107" => {// EncodedExerciseDescLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedExerciseDesc (41108)
                        value = self.extract_length(value, len, &[b"41108="]);
                    }
                },
                "41256" => {// EncodedStreamCommodityDescLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedStreamCommodityDesc (41257)
                        value = self.extract_length(value, len, &[b"41257="]);
                    }
                },
                "41320" => {// EncodedLegAdditionalTermBondDescLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedLegAdditionalTermBondDesc (41321)
                        value = self.extract_length(value, len, &[b"41321="]);
                    }
                },
                "41324" => {// EncodedLegAdditionalTermBondIssuerLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedLegAdditionalTermBondIssuer (41325)
                        value = self.extract_length(value, len, &[b"41325="]);
                    }
                },
                "41458" => {// EncodedLegDeliveryStreamCycleDescLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedLegDeliveryStreamCycleDesc (41459)
                        value = self.extract_length(value, len, &[b"41459="]);
                    }
                },
                "41476" => {// EncodedLegMarketDisruptionFallbackUnderlierSecurityDescLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedLegMarketDisruptionFallbackUnderlierSecurityDesc (41477)
                        value = self.extract_length(value, len, &[b"41477="]);
                    }
                },
                "41482" => {// EncodedLegExerciseDescLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedLegExerciseDesc (41483)
                        value = self.extract_length(value, len, &[b"41483="]);
                    }
                },
                "41653" => {// EncodedLegStreamCommodityDescLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedLegStreamCommodityDesc (41654)
                        value = self.extract_length(value, len, &[b"41654="]);
                    }
                },
                "41710" => {// EncodedUnderlyingAdditionalTermBondDescLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedUnderlyingAdditionalTermBondDesc (41711)
                        value = self.extract_length(value, len, &[b"41711="]);
                    }
                },
                "41806" => {// EncodedUnderlyingDeliveryStreamCycleDescLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedUnderlyingDeliveryStreamCycleDesc (41807)
                        value = self.extract_length(value, len, &[b"41807="]);
                    }
                },
                "41811" => {// EncodedUnderlyingExerciseDescLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedUnderlyingExerciseDesc (41812)
                        value = self.extract_length(value, len, &[b"41812="]);
                    }
                },
                "41873" => {// EncodedUnderlyingMarketDisruptionFallbackUnderlierSecurityDescLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedUnderlyingMarketDisruptionFallbackUnderlierSecurityDesc (41874)
                        value = self.extract_length(value, len, &[b"41874="]);
                    }
                },
                "41969" => {// EncodedUnderlyingStreamCommodityDescLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedUnderlyingStreamCommodityDesc (41970)
                        value = self.extract_length(value, len, &[b"41970="]);
                    }
                },
                "42025" => {// EncodedUnderlyingAdditionalTermBondIssuerLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedUnderlyingAdditionalTermBondIssuer (42026)
                        value = self.extract_length(value, len, &[b"42026="]);
                    }
                },
                "42171" => {// EncodedUnderlyingProvisionTextLen
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be EncodedUnderlyingProvisionText (42172)
                        value = self.extract_length(value, len, &[b"42172="]);
                    }
                },
                "42451" => {// LegPaymentStreamFormulaImageLength
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be LegPaymentStreamFormulaImage (42452)
                        value = self.extract_length(value, len, &[b"42452="]);
                    }
                },
                "42652" => {// PaymentStreamFormulaImageLength
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be PaymentStreamFormulaImage (42653)
                        value = self.extract_length(value, len, &[b"42653="]);
                    }
                },
                "42947" => {// UnderlyingPaymentStreamFormulaImageLength
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be UnderlyingPaymentStreamFormulaImage (42948)
                        value = self.extract_length(value, len, &[b"42948="]);
                    }
                },
                "43109" => {// PaymentStreamFormulaLength
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be PaymentStreamFormula (42684)
                        value = self.extract_length(value, len, &[b"42684="]);
                    }
                },
                "43110" => {// LegPaymentStreamFormulaLength
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be LegPaymentStreamFormula (42486)
                        value = self.extract_length(value, len, &[b"42486="]);
                    }
                },
                "43111" => {// UnderlyingPaymentStreamFormulaLength
                    if let Ok(len) = value.parse::<usize>() {
                        // next field should be UnderlyingPaymentStreamFormula (42982)
                        value = self.extract_length(value, len, &[b"42982="]);
                    }
                },
                "384" => {// NoMsgTypes
                    // next fields can be RefMsgType (372), MsgDirection (385), RefApplVerID (1130), RefApplExtID (1406), RefCstmApplVerID (1131) and DefaultVerIndicator (1410)
                    value = self.extract_fields(value, &["372", "385", "1130", "1406", "1131", "1410"]);
                },
                "453" => {// NoPartyIDs
                    // next fields can be PartyID (448), PartyIDSource (447), PartyRole (452), PartyRoleQualifier (2376), NoPartySubIDs (802), PartySubID (523) and PartySubIDType (803)
                    value = self.extract_fields(value, &["448", "447", "452", "2376", "802", "523", "803"]);
                },
                "454" => {// NoSecurityAltID
                    // next fields can be SecurityAltID (455) and SecurityAltIDSource (456)
                    value = self.extract_fields(value, &["455", "456"]);
                },
                "627" => {// NoHops
                    // next fields can be HopCompID (628), HopSendingTime (629) and HopRefID (630)
                    value = self.extract_fields(value, &["628", "629", "630"]);
                },
                "864" => {// NoEvents
                    // next fields can be EventType (865), EventDate (866), EventTime (630), EventPx (867), EventText (868), EventTimeUnit (1827), EventTimePeriod (1826), EventMonthYear (2340), EncodedEventTextLen (1578) and EncodedEventText (1579)
                    value = self.extract_fields(value, &["865", "866", "1145", "867", "868", "1827", "1826", "2340", "1578", "1579"]);
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

fn parse(input: &[u8]) -> impl Iterator<Item=(Cow<'_, str>, Cow<'_, str>)> {
    Parser {
        inner: input.split(|b| *b == 1)// b'\u{1}' = 1
    }
}

fn _from_bytes<'de, T>(input: &'de [u8], check: bool) -> Result<T, Error>
where
    T: de::Deserialize<'de>,
{
    if check {
        let len;
        let endpos;
        if let Some(pos) = input.windows(3).position(|b| b == "\u{1}9=".as_bytes()) {
            if let Some(relpos) = input.iter().skip(pos + 3).position(|b| *b == 1) {
                endpos = pos + 3 + relpos;
                len = endpos + String::from_utf8_lossy(&input[(pos + 3)..endpos]).parse::<usize>().map_err(|_| Error::custom("Malformed message (field 9 isn't an integer value)"))?;
            }
            else {
                return Err(Error::custom("Malformed message (missing field 9 terminator)"));
            }
        }
        else {
            return Err(Error::custom("Malformed message (missing field 9)"));
        }

        if let Some(pos) = input.windows(4).position(|b| b == "\u{1}10=".as_bytes()) {
            if pos != len {
                return Err(Error::custom(format!("Wrong body lenght, expected {} but got {}", len - endpos, pos - endpos)));
            }

            let checksum = input[0..(pos + 1)].iter().map(|b| *b as usize).sum::<usize>() % crate::CHECKSUM_MOD;
            if &input[(pos + 1)..] != format!("10={:03}\u{1}", checksum).as_bytes() {
                return Err(Error::custom(format!("Mismatching checksum, expected {:03} but got {}", checksum, String::from_utf8_lossy(&input[(pos + 4)..(pos + 7)]))));
            }
        }
        else {
            return Err(Error::custom("Malformed message (missing field 10)"));
        }
    }
    T::deserialize(Deserializer::new(parse(input)))
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
///     ("9".to_owned(), "34".to_owned()),
///     ("cheese".to_owned(), "comté".to_owned()),
///     ("meat".to_owned(), "ham".to_owned()),
///     ("fat".to_owned(), "butter".to_owned()),
///     ("10".to_owned(), "095".to_owned()),
/// ];
///
/// assert_eq!(
///     serde_fix::from_bytes_checked::<Vec<(String, String)>>("bread=baguette\u{1}9=34\u{1}cheese=comté\u{1}meat=ham\u{1}fat=butter\u{1}10=095\u{1}".as_bytes()),
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
///     ("9".to_owned(), "34".to_owned()),
///     ("cheese".to_owned(), "comté".to_owned()),
///     ("meat".to_owned(), "ham".to_owned()),
///     ("fat".to_owned(), "butter".to_owned()),
///     ("10".to_owned(), "095".to_owned()),
/// ];
///
/// assert_eq!(
///     serde_fix::from_str_checked::<Vec<(String, String)>>("bread=baguette\u{1}9=34\u{1}cheese=comté\u{1}meat=ham\u{1}fat=butter\u{1}10=095\u{1}"),
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
