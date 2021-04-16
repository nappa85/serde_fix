
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ComplexEventPeriodDateGrp {
	/// NoComplexEventPeriodDateTimes
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41007")]
	pub complex_event_period_date_times: Option<fix_common::RepeatingValues<ComplexEventPeriodDateTime>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ComplexEventPeriodDateTime {
	/// Required if NoComplexEventPeriodDateTimes(41007) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41008")]
	pub complex_event_period_date: Option<fix_common::LocalMktDate>,
	/// ComplexEventPeriodTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41009")]
	pub complex_event_period_time: Option<String>,
}
