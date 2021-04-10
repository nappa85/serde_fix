
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MarketDisruptionEventGrp {
	/// NoMarketDisruptionEvents
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41092")]
	pub market_disruption_events: Option<fix_common::RepeatingValues<MarketDisruptionEvent>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MarketDisruptionEvent {
	/// Required if NoMarketDisruptionEvents(41092) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41093")]
	pub market_disruption_event: Option<String>,
	/// MarketDisruptionValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40991")]
	pub market_disruption_value: Option<String>,
}
