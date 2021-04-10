
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPricingDateTime {
	/// UnderlyingPricingDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41949")]
	pub underlying_pricing_date_unadjusted: Option<crate::entities::LocalMktDate>,
	/// UnderlyingPricingDateBusinessDayConvention
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41950")]
	pub underlying_pricing_date_business_day_convention: Option<UnderlyingPricingDateBusinessDayConvention>,
	/// UnderlyingPricingDateBusinessCenterGrp
	#[serde(flatten)]
	pub underlying_pricing_date_business_center_grp: Option<super::underlying_pricing_date_business_center_grp::UnderlyingPricingDateBusinessCenterGrp>,
	/// UnderlyingPricingDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41951")]
	pub underlying_pricing_date_adjusted: Option<crate::entities::LocalMktDate>,
	/// UnderlyingPricingTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41952")]
	pub underlying_pricing_time: Option<String>,
	/// UnderlyingPricingTimeBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41953")]
	pub underlying_pricing_time_business_center: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingPricingDateBusinessDayConvention {
	/// Not applicable
	#[serde(rename = "0")]
	NotApplicable,
	/// None (current day)
	#[serde(rename = "1")]
	None,
	/// Following day
	#[serde(rename = "2")]
	FollowingDay,
	/// Floating rate note
	#[serde(rename = "3")]
	FloatingRateNote,
	/// Modified following day
	#[serde(rename = "4")]
	ModifiedFollowingDay,
	/// Preceding day
	#[serde(rename = "5")]
	PrecedingDay,
	/// Modified preceding day
	#[serde(rename = "6")]
	ModifiedPrecedingDay,
	/// Nearest day
	#[serde(rename = "7")]
	NearestDay,
}
