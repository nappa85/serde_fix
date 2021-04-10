
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExtraordinaryEventGrp {
	/// NoExtraordinaryEvents
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42296")]
	pub extraordinary_events: Option<crate::entities::RepeatingValues<ExtraordinaryEvent>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExtraordinaryEvent {
	/// Required if NoExtraordinaryEvents(42296) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42297")]
	pub extraordinary_event_type: Option<String>,
	/// Required if NoExtraordinaryEvents(42296) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42298")]
	pub extraordinary_event_value: Option<String>,
}
