
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStubEndDate {
	/// UnderlyingPaymentStubEndDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42984")]
	pub underlying_payment_stub_end_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified value would be specific to this payment stub instance.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42985")]
	pub underlying_payment_stub_end_date_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified values would be specific to this payment stub instance.
	#[serde(flatten)]
	pub underlying_payment_stub_end_date_business_center_grp: Option<super::underlying_payment_stub_end_date_business_center_grp::UnderlyingPaymentStubEndDateBusinessCenterGrp>,
	/// UnderlyingPaymentStubEndDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42986")]
	pub underlying_payment_stub_end_date_relative_to: Option<i32>,
	/// Conditionally required when UnderlyingPaymentStubEndDateOffsetUnit(42988) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42987")]
	pub underlying_payment_stub_end_date_offset_period: Option<i32>,
	/// Conditionally required when UnderlyingPaymentStubEndDateOffsetPeriod(42987) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42988")]
	pub underlying_payment_stub_end_date_offset_unit: Option<String>,
	/// UnderlyingPaymentStubEndDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42989")]
	pub underlying_payment_stub_end_date_offset_day_type: Option<i32>,
	/// UnderlyingPaymentStubEndDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42990")]
	pub underlying_payment_stub_end_date_adjusted: Option<fix_common::LocalMktDate>,
}
