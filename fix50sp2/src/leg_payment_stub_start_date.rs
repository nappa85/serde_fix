
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStubStartDate {
	/// LegPaymentStubStartDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42497")]
	pub leg_payment_stub_start_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the LegDateAdjustment component in InstrumentLeg. The
	/// specified value would be specific to this payment stub instance.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42498")]
	pub leg_payment_stub_start_date_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the LegDateAdjustment component in InstrumentLeg. The specified
	/// values would be specific to this payment stub instance.
	#[serde(flatten)]
	pub leg_payment_stub_start_date_business_center_grp: Option<super::leg_payment_stub_start_date_business_center_grp::LegPaymentStubStartDateBusinessCenterGrp>,
	/// LegPaymentStubStartDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42499")]
	pub leg_payment_stub_start_date_relative_to: Option<i32>,
	/// Conditionally required when LegPaymentStubStartDateOffsetUnit(42501) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42500")]
	pub leg_payment_stub_start_date_offset_period: Option<i32>,
	/// Conditionally required when LegPaymentStubStartDateOffsetPeriod(42500) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42501")]
	pub leg_payment_stub_start_date_offset_unit: Option<String>,
	/// LegPaymentStubStartDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42502")]
	pub leg_payment_stub_start_date_offset_day_type: Option<i32>,
	/// LegPaymentStubStartDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42503")]
	pub leg_payment_stub_start_date_adjusted: Option<fix_common::LocalMktDate>,
}
