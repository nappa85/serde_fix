
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegComplexEventRateSourceGrp {
	/// NoLegComplexEventRateSources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41382")]
	pub leg_complex_event_rate_sources: Option<fix_common::RepeatingValues<LegComplexEventRateSource>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegComplexEventRateSource {
	/// Required if NoLegComplexEventRateSources(41382) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41383")]
	pub leg_complex_event_rate_source_item: Option<LegComplexEventRateSourceItem>,
	/// Required if NoLegComplexEventRateSources(41382) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41384")]
	pub leg_complex_event_rate_source_type: Option<LegComplexEventRateSourceType>,
	/// Conditionally required when LegComplexEventRateSource(41383) = 99 (Other).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41385")]
	pub leg_complex_event_reference_page: Option<String>,
	/// LegComplexEvenReferencePageHeading
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41386")]
	pub leg_complex_even_reference_page_heading: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegComplexEventRateSourceItem {
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

impl Default for LegComplexEventRateSourceItem {
	fn default() -> Self {
		LegComplexEventRateSourceItem::Bloomberg
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegComplexEventRateSourceType {
	/// Primary
	#[serde(rename = "0")]
	Primary,
	/// Secondary
	#[serde(rename = "1")]
	Secondary,
}

impl Default for LegComplexEventRateSourceType {
	fn default() -> Self {
		LegComplexEventRateSourceType::Primary
	}
}
