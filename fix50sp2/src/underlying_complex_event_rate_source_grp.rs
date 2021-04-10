
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingComplexEventRateSourceGrp {
	/// NoUnderlyingComplexEventRateSources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41732")]
	pub underlying_complex_event_rate_sources: Option<fix_common::RepeatingValues<UnderlyingComplexEventRateSource>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingComplexEventRateSource {
	/// Required if NoUnderlyingComplexEventRateSources(41732) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41733")]
	pub underlying_complex_event_rate_source_item: Option<UnderlyingComplexEventRateSourceItem>,
	/// Required if NoUnderlyingComplexEventRateSources(41732) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41734")]
	pub underlying_complex_event_rate_source_type: Option<UnderlyingComplexEventRateSourceType>,
	/// Conditionally required when ComplexEventRateSource(41014) = 99 (Other).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41735")]
	pub underlying_complex_event_reference_page: Option<String>,
	/// UnderlyingComplexEvenReferencePageHeading
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41736")]
	pub underlying_complex_even_reference_page_heading: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingComplexEventRateSourceItem {
	/// Bloomberg
	#[serde(rename = "0")]
	Bloomberg,
	/// Reuters
	#[serde(rename = "1")]
	Reuters,
	/// Telerate
	#[serde(rename = "2")]
	Telerate,
	/// ISDA Settlement Rate Option (Elaboration: The source of the currency conversion as specified by the ISDA terms in Annex A
	/// to the 1998 FX and Currency Option Definitions. See <a xmlns="http://www.b2bits.com/FIXProtocol" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" href="http://www.fpml.org/coding-scheme/settlement-rate-option" target="_blank">http://www.fpml.org/coding-scheme/settlement-rate-option</a> )
	#[serde(rename = "3")]
	IsdaSettlementRateOption,
	/// Other
	#[serde(rename = "99")]
	Other,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingComplexEventRateSourceType {
	/// Primary
	#[serde(rename = "0")]
	Primary,
	/// Secondary
	#[serde(rename = "1")]
	Secondary,
}
