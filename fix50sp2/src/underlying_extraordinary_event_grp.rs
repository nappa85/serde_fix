
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingExtraordinaryEventGrp {
	/// NoUnderlyingExtraordinaryEvents
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42884")]
	pub underlying_extraordinary_events: Option<fix_common::RepeatingValues<UnderlyingExtraordinaryEvent>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingExtraordinaryEvent {
	/// Required if NoUnderlyingExtraordinaryEvents(42884) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42885")]
	pub underlying_extraordinary_event_type: Option<String>,
	/// Required if NoUnderlyingExtraordinaryEvents(42884) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42886")]
	pub underlying_extraordinary_event_value: Option<String>,
}
