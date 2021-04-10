
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PricingDateTime {
	/// PricingDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41232")]
	pub pricing_date_unadjusted: Option<crate::entities::LocalMktDate>,
	/// PricingDateBusinessDayConvention
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41233")]
	pub pricing_date_business_day_convention: Option<PricingDateBusinessDayConvention>,
	/// PricingDateBusinessCenterGrp
	#[serde(flatten)]
	pub pricing_date_business_center_grp: Option<super::pricing_date_business_center_grp::PricingDateBusinessCenterGrp>,
	/// PricingDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41234")]
	pub pricing_date_adjusted: Option<crate::entities::LocalMktDate>,
	/// PricingTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41235")]
	pub pricing_time: Option<String>,
	/// PricingTimeBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41236")]
	pub pricing_time_business_center: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PricingDateBusinessDayConvention {
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
