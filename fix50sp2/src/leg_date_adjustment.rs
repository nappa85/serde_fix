
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegDateAdjustment {
	/// LegBusinessDayConvention
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40925")]
	pub leg_business_day_convention: Option<i32>,
	/// LegBusinessCenterGrp
	#[serde(flatten)]
	pub leg_business_center_grp: Option<super::leg_business_center_grp::LegBusinessCenterGrp>,
	/// LegDateRollConvention
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40926")]
	pub leg_date_roll_convention: Option<String>,
}
