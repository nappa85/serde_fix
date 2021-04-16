
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegComplexEventTimes {
	/// NoLegComplexEventTimes
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2253")]
	pub leg_complex_event_times: Option<fix_common::RepeatingValues<LegComplexEventTime>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegComplexEventTime {
	/// Required if NoLegComplexEventTimes(2253) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2204")]
	pub leg_complex_event_start_time: Option<fix_common::UTCTimeOnly>,
	/// Required if NoLegComplexEventTimes(2253) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2247")]
	pub leg_complex_event_end_time: Option<fix_common::UTCTimeOnly>,
}
