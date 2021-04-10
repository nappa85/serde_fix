
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingComplexEventPeriodGrp {
	/// NoUnderlyingComplexEventPeriods
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41729")]
	pub underlying_complex_event_periods: Option<fix_common::RepeatingValues<UnderlyingComplexEventPeriod>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingComplexEventPeriod {
	/// Required if NoUnderlyingComplexEventPeriods(41729) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41730")]
	pub underlying_complex_event_period_type: Option<UnderlyingComplexEventPeriodType>,
	/// UnderlyingComplexEventBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41731")]
	pub underlying_complex_event_business_center: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingComplexEventPeriodType {
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
