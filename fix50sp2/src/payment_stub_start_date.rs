
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStubStartDate {
	/// PaymentStubStartDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42698")]
	pub payment_stub_start_date_unadjusted: Option<crate::entities::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the DateAdjustment component in Instrument. The specified
	/// value would be specific to this payment stub instance.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42699")]
	pub payment_stub_start_date_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the DateAdjustment component in Instrument. The specified values
	/// would be specific to this payment stub instance.
	#[serde(flatten)]
	pub payment_stub_start_date_business_center_grp: Option<super::payment_stub_start_date_business_center_grp::PaymentStubStartDateBusinessCenterGrp>,
	/// PaymentStubStartDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42700")]
	pub payment_stub_start_date_relative_to: Option<i32>,
	/// Conditionally required when PaymentStubStartDateOffsetUnit(42702) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42701")]
	pub payment_stub_start_date_offset_period: Option<i32>,
	/// Conditionally required when PaymentStubStartDateOffsetPeriod(42701) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42702")]
	pub payment_stub_start_date_offset_unit: Option<String>,
	/// PaymentStubStartDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42703")]
	pub payment_stub_start_date_offset_day_type: Option<i32>,
	/// PaymentStubStartDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42704")]
	pub payment_stub_start_date_adjusted: Option<crate::entities::LocalMktDate>,
}
