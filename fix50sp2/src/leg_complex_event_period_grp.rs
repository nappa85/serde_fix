
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegComplexEventPeriodGrp {
	/// NoLegComplexEventPeriods
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41379")]
	pub leg_complex_event_periods: Option<fix_common::RepeatingValues<LegComplexEventPeriod>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegComplexEventPeriod {
	/// Required if NoLegComplexEventPeriods(41379) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41380")]
	pub leg_complex_event_period_type: Option<LegComplexEventPeriodType>,
	/// LegComplexEventBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41381")]
	pub leg_complex_event_business_center: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegComplexEventPeriodType {
	/// Asian Out
	#[serde(rename = "0")]
	AsianOut,
	/// Asian In
	#[serde(rename = "1")]
	AsianIn,
	/// Barrier Cap
	#[serde(rename = "2")]
	BarrierCap,
	/// Barrier Floor
	#[serde(rename = "3")]
	BarrierFloor,
	/// Knock Out
	#[serde(rename = "4")]
	KnockOut,
	/// Knock In
	#[serde(rename = "5")]
	KnockIn,
}

impl Default for LegComplexEventPeriodType {
	fn default() -> Self {
		LegComplexEventPeriodType::AsianOut
	}
}
