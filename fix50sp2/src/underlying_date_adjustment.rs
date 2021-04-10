
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingDateAdjustment {
	/// UnderlyingBusinessDayConvention
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40964")]
	pub underlying_business_day_convention: Option<String>,
	/// UnderlyingBusinessCenterGrp
	#[serde(flatten)]
	pub underlying_business_center_grp: Option<super::underlying_business_center_grp::UnderlyingBusinessCenterGrp>,
	/// UnderlyingDateRollConvention
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40965")]
	pub underlying_date_roll_convention: Option<String>,
}
