
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStubEndDate {
	/// PaymentStubEndDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42689")]
	pub payment_stub_end_date_unadjusted: Option<crate::entities::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the DateAdjustment component in Instrument. The specified
	/// value would be specific to this payment stub instance.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42690")]
	pub payment_stub_end_date_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the DateAdjustment component in Instrument. The specified values
	/// would be specific to this payment stub instance.
	#[serde(flatten)]
	pub payment_stub_end_date_business_center_grp: Option<super::payment_stub_end_date_business_center_grp::PaymentStubEndDateBusinessCenterGrp>,
	/// PaymentStubEndDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42691")]
	pub payment_stub_end_date_relative_to: Option<i32>,
	/// Conditionally required when PaymentStubEndDateOffsetUnit(42693) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42692")]
	pub payment_stub_end_date_offset_period: Option<i32>,
	/// Conditionally required when PaymentStubEndDateOffsetPeriod(42692) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42693")]
	pub payment_stub_end_date_offset_unit: Option<String>,
	/// PaymentStubEndDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42694")]
	pub payment_stub_end_date_offset_day_type: Option<i32>,
	/// PaymentStubEndDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42695")]
	pub payment_stub_end_date_adjusted: Option<crate::entities::LocalMktDate>,
}
