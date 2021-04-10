
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ComplexEventRateSourceGrp {
	/// NoComplexEventRateSources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41013")]
	pub complex_event_rate_sources: Option<fix_common::RepeatingValues<ComplexEventRateSource>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ComplexEventRateSource {
	/// Required if NoComplexEventRateSources(41013) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41014")]
	pub complex_event_rate_source_item: Option<ComplexEventRateSourceItem>,
	/// Required if NoComplexEventRateSources(41013) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41015")]
	pub complex_event_rate_source_type: Option<ComplexEventRateSourceType>,
	/// Conditionally required when ComplexEventRateSource(41014) = 99 (Other).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41016")]
	pub complex_event_reference_page: Option<String>,
	/// ComplexEventReferencePageHeading
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41017")]
	pub complex_event_reference_page_heading: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ComplexEventRateSourceItem {
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
pub enum ComplexEventRateSourceType {
	/// Primary
	#[serde(rename = "0")]
	Primary,
	/// Secondary
	#[serde(rename = "1")]
	Secondary,
}
