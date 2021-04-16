
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingComplexEventPeriodDateGrp {
	/// NoUnderlyingComplexEventPeriodDateTimes
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41726")]
	pub underlying_complex_event_period_date_times: Option<fix_common::RepeatingValues<UnderlyingComplexEventPeriodDateTime>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingComplexEventPeriodDateTime {
	/// Required if NoUnderlyingComplexEventPeriodDateTimes(41726) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41727")]
	pub underlying_complex_event_period_date: Option<fix_common::LocalMktDate>,
	/// UnderlyingComplexEventPeriodTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41728")]
	pub underlying_complex_event_period_time: Option<String>,
}
