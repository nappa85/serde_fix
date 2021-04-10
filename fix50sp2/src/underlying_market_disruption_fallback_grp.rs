
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingMarketDisruptionFallbackGrp {
	/// NoUnderlyingMarketDisruptionFallbacks
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41866")]
	pub underlying_market_disruption_fallbacks: Option<crate::entities::RepeatingValues<UnderlyingMarketDisruptionFallback>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingMarketDisruptionFallback {
	/// Required if NoUnderlyingMarketDisruptionFallbacks(41866) &gt; 0. The sequence of entries specifies the order in which the fallback
	/// provisions should be applied.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41867")]
	pub underlying_market_disruption_fallback_type: Option<String>,
	/// UnderlyingMarketDisruptionFallbackValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41339")]
	pub underlying_market_disruption_fallback_value: Option<String>,
}
