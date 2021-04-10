
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MarketDisruptionFallbackGrp {
	/// NoMarketDisruptionFallbacks
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41094")]
	pub market_disruption_fallbacks: Option<crate::entities::RepeatingValues<MarketDisruptionFallback>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MarketDisruptionFallback {
	/// Required if NoMarketDisruptionFallbacks(41094) &gt; 0. The sequence of entries specifies the order in which the fallback provisions
	/// should be applied.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41095")]
	pub market_disruption_fallback_type: Option<String>,
	/// MarketDisruptionFallbackValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40992")]
	pub market_disruption_fallback_value: Option<String>,
}
