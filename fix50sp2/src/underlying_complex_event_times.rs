
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingComplexEventTimes {
	/// NoUnderlyingComplexEventTimes
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2056")]
	pub underlying_complex_event_times: Option<fix_common::RepeatingValues<UnderlyingComplexEventTime>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingComplexEventTime {
	/// Required if NoUnderlyingComplexEventTimes(2056) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2057")]
	pub underlying_complex_event_start_time: Option<fix_common::UTCTimeOnly>,
	/// Required if NoUnderlyingComplexEventTimes(2056) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2058")]
	pub underlying_complex_event_end_time: Option<fix_common::UTCTimeOnly>,
}
