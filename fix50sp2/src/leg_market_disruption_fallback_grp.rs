
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegMarketDisruptionFallbackGrp {
	/// NoLegMarketDisruptionFallbacks
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41469")]
	pub leg_market_disruption_fallbacks: Option<crate::entities::RepeatingValues<LegMarketDisruptionFallback>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegMarketDisruptionFallback {
	/// <p>Required if NoLegMarketDisruptionFallbacks(41469) &gt; 0.</p>
	/// <p>The sequence of entries specifies the order in which the fallback provisions should be applied.</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41470")]
	pub leg_market_disruption_fallback_type: Option<String>,
	/// LegMarketDisruptionFallbackValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40990")]
	pub leg_market_disruption_fallback_value: Option<String>,
}
