
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingDividendAccrualPaymentDate {
	/// UnderlyingDividendAccrualPaymentDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42819")]
	pub underlying_dividend_accrual_payment_date_relative_to: Option<i32>,
	/// Conditionally required when UnderlyingDividendAccrualPaymentDateOffsetUnit(42821) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42820")]
	pub underlying_dividend_accrual_payment_date_offset_period: Option<i32>,
	/// Conditionally required when UnderlyingDividendAccrualPaymentDateOffsetPeriod(42820) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42821")]
	pub underlying_dividend_accrual_payment_date_offset_unit: Option<String>,
	/// UnderlyingDividendAccrualPaymentDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42822")]
	pub underlying_dividend_accrual_payment_date_offset_day_type: Option<i32>,
	/// UnderlyingDividendAccrualPaymentDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42823")]
	pub underlying_dividend_accrual_payment_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The value would be specific to this instance of UnderlyingDividendAccrualPaymentDate.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42824")]
	pub underlying_dividend_accrual_payment_date_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The values would be specific to this instance of UnderlyingDividendAccrualPaymentDate.
	#[serde(flatten)]
	pub underlying_dividend_accrual_payment_date_business_center_grp: Option<super::underlying_dividend_accrual_payment_date_business_center_grp::UnderlyingDividendAccrualPaymentDateBusinessCenterGrp>,
	/// UnderlyingDividendAccrualPaymentDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42825")]
	pub underlying_dividend_accrual_payment_date_adjusted: Option<fix_common::LocalMktDate>,
}
