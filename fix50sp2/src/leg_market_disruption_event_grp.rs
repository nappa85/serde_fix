
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegMarketDisruptionEventGrp {
	/// NoLegMarketDisruptionEvents
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41467")]
	pub leg_market_disruption_events: Option<crate::entities::RepeatingValues<LegMarketDisruptionEvent>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegMarketDisruptionEvent {
	/// <p>Required if NoLegMarketDisruptionEvents(41467) &gt; 0</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41468")]
	pub leg_market_disruption_event: Option<String>,
	/// LegMarketDisruptionValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40223")]
	pub leg_market_disruption_value: Option<String>,
}
