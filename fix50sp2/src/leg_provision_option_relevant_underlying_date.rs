
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProvisionOptionRelevantUnderlyingDate {
	/// LegProvisionOptionRelevantUnderlyingDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40508")]
	pub leg_provision_option_relevant_underlying_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the LegDateAdjustment component in InstrumentLeg. The
	/// specified value would be specific to this instance of the leg provision option relevant underlying date.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40509")]
	pub leg_provision_option_relevant_underlying_date_business_day_convention: Option<LegProvisionOptionRelevantUnderlyingDateBusinessDayConvention>,
	/// When specified, this overrides the business centers defined in the LegDateAdjustment component in InstrumentLeg. The specified
	/// values would be specific to this instance of the leg provision option relevant underlying date.
	#[serde(flatten)]
	pub leg_provision_option_relevant_underlying_date_business_center_grp: Option<super::leg_provision_option_relevant_underlying_date_business_center_grp::LegProvisionOptionRelevantUnderlyingDateBusinessCenterGrp>,
	/// LegProvisionOptionRelevantUnderlyingDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40511")]
	pub leg_provision_option_relevant_underlying_date_relative_to: Option<i32>,
	/// Conditionally required when LegProvisionOptionRelevantUnderlyingDateOffsetUnit(40513) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40512")]
	pub leg_provision_option_relevant_underlying_date_offset_period: Option<i32>,
	/// Conditionally required when LegProvisionOptionRelevantUnderlyingDateOffsetPeriod(40512) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40513")]
	pub leg_provision_option_relevant_underlying_date_offset_unit: Option<String>,
	/// LegProvisionOptionRelevantUnderlyingDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40514")]
	pub leg_provision_option_relevant_underlying_date_offset_day_type: Option<i32>,
	/// LegProvisionOptionRelevantUnderlyingDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40515")]
	pub leg_provision_option_relevant_underlying_date_adjusted: Option<fix_common::LocalMktDate>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegProvisionOptionRelevantUnderlyingDateBusinessDayConvention {
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

impl Default for LegProvisionOptionRelevantUnderlyingDateBusinessDayConvention {
	fn default() -> Self {
		LegProvisionOptionRelevantUnderlyingDateBusinessDayConvention::NotApplicable
	}
}
