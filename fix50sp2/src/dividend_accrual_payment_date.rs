
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DividendAccrualPaymentDate {
	/// DividendAccrualPaymentDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42238")]
	pub dividend_accrual_payment_date_relative_to: Option<i32>,
	/// Conditionally required when DividendAccrualPaymentDateOffsetUnit(42240) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42239")]
	pub dividend_accrual_payment_date_offset_period: Option<i32>,
	/// Conditionally required when DividendAccrualPaymentDateOffsetPeriod(42239) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42240")]
	pub dividend_accrual_payment_date_offset_unit: Option<String>,
	/// DividendAccrualPaymentDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42241")]
	pub dividend_accrual_payment_date_offset_day_type: Option<i32>,
	/// DividendAccrualPaymentDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42242")]
	pub dividend_accrual_payment_date_unadjusted: Option<crate::entities::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the DateAdjustment component in Instrument. The value
	/// would be specific to this instance of DividendAccrualPaymentDate.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42243")]
	pub dividend_accrual_paymeent_date_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the DateAdjustment component in Instrument. The values would
	/// be specific to this instance of DividendAccrualPaymentDate.
	#[serde(flatten)]
	pub dividend_accrual_payment_date_business_center_grp: Option<super::dividend_accrual_payment_date_business_center_grp::DividendAccrualPaymentDateBusinessCenterGrp>,
	/// DividendAccrualPaymentDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42244")]
	pub dividend_accrual_payment_date_adjusted: Option<crate::entities::LocalMktDate>,
}
