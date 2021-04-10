
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStubEndDate {
	/// LegPaymentStubEndDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42488")]
	pub leg_payment_stub_end_date_unadjusted: Option<crate::entities::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the LegDateAdjustment component in InstrumentLeg. The
	/// specified value would be specific to this payment stub instance.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42489")]
	pub leg_payment_stub_end_date_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the LegDateAdjustment component in InstrumentLeg. The specified
	/// values would be specific to this payment stub instance.
	#[serde(flatten)]
	pub leg_payment_stub_end_date_business_center_grp: Option<super::leg_payment_stub_end_date_business_center_grp::LegPaymentStubEndDateBusinessCenterGrp>,
	/// LegPaymentStubEndDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42490")]
	pub leg_payment_stub_end_date_relative_to: Option<i32>,
	/// Conditionally required when LegPaymentStubEndDateOffsetUnit(42492) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42491")]
	pub leg_payment_stub_end_date_offset_period: Option<i32>,
	/// Conditionally required when LegPaymentStubEndDateOffsetPeriod(42491) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42492")]
	pub leg_payment_stub_end_date_offset_unit: Option<String>,
	/// LegPaymentStubEndDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42493")]
	pub leg_payment_stub_end_date_offset_day_type: Option<i32>,
	/// LegPaymentStubEndDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42494")]
	pub leg_payment_stub_end_date_adjusted: Option<crate::entities::LocalMktDate>,
}
