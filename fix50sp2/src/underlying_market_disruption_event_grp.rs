
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingMarketDisruptionEventGrp {
	/// NoUnderlyingMarketDisruptionEvents
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41864")]
	pub underlying_market_disruption_events: Option<crate::entities::RepeatingValues<UnderlyingMarketDisruptionEvent>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingMarketDisruptionEvent {
	/// <p>Required if NoUndelryingMarketDisruptionEvents(41864) &gt; 0</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41865")]
	pub underlying_market_disruption_event: Option<String>,
	/// UnderlyingMarketDisruptionValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41338")]
	pub underlying_market_disruption_value: Option<String>,
}
