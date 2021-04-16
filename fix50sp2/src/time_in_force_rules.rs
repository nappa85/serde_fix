
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TimeInForceRules {
	/// Number of time in force techniques
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1239")]
	pub time_in_force_rules: Option<fix_common::RepeatingValues<TimeInForceRule>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TimeInForceRule {
	/// Indicates time in force techniques that are valid for the specified market segment
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "59")]
	pub time_in_force: Option<TimeInForce>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TimeInForce {
	/// Day (or session)
	#[serde(rename = "0")]
	Day,
	/// Good Till Cancel (GTC)
	#[serde(rename = "1")]
	GoodTillCancel,
	/// At the Opening (OPG)
	#[serde(rename = "2")]
	AtTheOpening,
	/// Immediate Or Cancel (IOC)
	#[serde(rename = "3")]
	ImmediateOrCancel,
	/// Fill Or Kill (FOK)
	#[serde(rename = "4")]
	FillOrKill,
	/// Good Till Crossing (GTX)
	#[serde(rename = "5")]
	GoodTillCrossing,
	/// Good Till Date (GTD)
	#[serde(rename = "6")]
	GoodTillDate,
	/// At the Close
	#[serde(rename = "7")]
	AtTheClose,
	/// Good Through Crossing
	#[serde(rename = "8")]
	GoodThroughCrossing,
	/// At Crossing
	#[serde(rename = "9")]
	AtCrossing,
	/// Good For Time
	#[serde(rename = "A")]
	GoodForTime,
	/// Good for auction (GFA)
	#[serde(rename = "B")]
	GoodForAuction,
	/// Good for this Month (GFM)
	#[serde(rename = "C")]
	GoodForThisMonth,
}

impl Default for TimeInForce {
	fn default() -> Self {
		TimeInForce::Day
	}
}
