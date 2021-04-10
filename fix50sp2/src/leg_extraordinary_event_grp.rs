
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegExtraordinaryEventGrp {
	/// NoLegExtraordinaryEvents
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42388")]
	pub leg_extraordinary_events: Option<crate::entities::RepeatingValues<LegExtraordinaryEvent>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegExtraordinaryEvent {
	/// Required if NoLegExtraordinaryEvents(42388) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42389")]
	pub leg_extraordinary_event_type: Option<String>,
	/// Required if NoLegExtraordinaryEvents(42388) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42390")]
	pub leg_extraordinary_event_value: Option<String>,
}
