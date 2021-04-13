
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegSettlRateFallbackRateSource {
	/// LegSettlRateFallbackRateSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40366")]
	pub leg_settl_rate_fallback_rate_source_item: Option<LegSettlRateFallbackRateSourceItem>,
	/// Conditionally required when LegSettlRateFallbackRateSource(40366) = 3 (ISDA Settlement Rate Option) or 99 (Other).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40370")]
	pub leg_settl_rate_fallback_reference_page: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegSettlRateFallbackRateSourceItem {
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

impl Default for LegSettlRateFallbackRateSourceItem {
	fn default() -> Self {
		LegSettlRateFallbackRateSourceItem::Bloomberg
	}
}
