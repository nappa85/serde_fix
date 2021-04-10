
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegComplexEventDates {
	/// NoLegComplexEventDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2250")]
	pub leg_complex_event_dates: Option<crate::entities::RepeatingValues<LegComplexEventDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegComplexEventDate {
	/// Required if NoLegComplexEventDates(2250) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2251")]
	pub leg_complex_event_start_date: Option<crate::entities::UTCTimestamp>,
	/// Required if NoLegComplexEventDates(2250) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2252")]
	pub leg_complex_event_end_date: Option<crate::entities::UTCTimestamp>,
}
