
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegComplexEventPeriodDateGrp {
	/// NoLegComplexEventPeriodDateTimes
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41376")]
	pub leg_complex_event_period_date_times: Option<crate::entities::RepeatingValues<LegComplexEventPeriodDateTime>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegComplexEventPeriodDateTime {
	/// Required if NoLegComplexEventPeriodDateTimes(41376) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41377")]
	pub leg_complex_event_period_date: Option<crate::entities::LocalMktDate>,
	/// LegComplexEventPeriodTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41378")]
	pub leg_complex_event_period_time: Option<String>,
}
