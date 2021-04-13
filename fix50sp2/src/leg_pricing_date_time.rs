
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPricingDateTime {
	/// LegPricingDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41609")]
	pub leg_pricing_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// LegPricingDateBusinessDayConvention
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41610")]
	pub leg_pricing_date_business_day_convention: Option<LegPricingDateBusinessDayConvention>,
	/// LegPricingDateBusinessCenterGrp
	#[serde(flatten)]
	pub leg_pricing_date_business_center_grp: Option<super::leg_pricing_date_business_center_grp::LegPricingDateBusinessCenterGrp>,
	/// LegPricingDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41611")]
	pub leg_pricing_date_adjusted: Option<fix_common::LocalMktDate>,
	/// LegPricingTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41612")]
	pub leg_pricing_time: Option<String>,
	/// LegPricingTimeBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41613")]
	pub leg_pricing_time_business_center: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegPricingDateBusinessDayConvention {
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

impl Default for LegPricingDateBusinessDayConvention {
	fn default() -> Self {
		LegPricingDateBusinessDayConvention::NotApplicable
	}
}
