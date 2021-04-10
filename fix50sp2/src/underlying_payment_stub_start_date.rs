
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStubStartDate {
	/// UnderlyingPaymentStubStartDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42993")]
	pub underlying_payment_stub_start_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified value would be specific to this payment stub instance.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42994")]
	pub underlying_payment_stub_start_date_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified values would be specific to this payment stub instance.
	#[serde(flatten)]
	pub underlying_payment_stub_start_date_business_center_grp: Option<super::underlying_payment_stub_start_date_business_center_grp::UnderlyingPaymentStubStartDateBusinessCenterGrp>,
	/// UnderlyingPaymentStubStartDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42995")]
	pub underlying_payment_stub_start_date_relative_to: Option<i32>,
	/// Conditionally required when UnderlyingPaymentStubStartDateOffsetUnit(42997) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42996")]
	pub underlying_payment_stub_start_date_offset_period: Option<i32>,
	/// Conditionally required when UnderlyingPaymentStubStartDateOffsetPeriod(42996) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42997")]
	pub underlying_payment_stub_start_date_offset_unit: Option<String>,
	/// UnderlyingPaymentStubStartDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42998")]
	pub underlying_payment_stub_start_date_offset_day_type: Option<i32>,
	/// UnderlyingPaymentStubStartDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42999")]
	pub underlying_payment_stub_start_date_adjusted: Option<fix_common::LocalMktDate>,
}
