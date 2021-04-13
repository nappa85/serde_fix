
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ComplexEventPeriodGrp {
	/// NoComplexEventPeriods
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41010")]
	pub complex_event_periods: Option<fix_common::RepeatingValues<ComplexEventPeriod>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ComplexEventPeriod {
	/// Required if NoComplexEventPeriods(41010) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41011")]
	pub complex_event_period_type: Option<ComplexEventPeriodType>,
	/// ComplexEventBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41012")]
	pub complex_event_business_center: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ComplexEventPeriodType {
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

impl Default for ComplexEventPeriodType {
	fn default() -> Self {
		ComplexEventPeriodType::AsianOut
	}
}
