
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegDividendAccrualPaymentDate {
	/// LegDividendAccrualPaymentDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42330")]
	pub leg_dividend_accrual_payment_date_relative_to: Option<i32>,
	/// Conditionally required when LegDividendAccrualPaymentDateOffsetUnit(42332) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42331")]
	pub leg_dividend_accrual_payment_date_offset_period: Option<i32>,
	/// Conditionally required when LegDividendAccrualPaymentDateOffsetPeriod(42331) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42332")]
	pub leg_dividend_accrual_payment_date_offset_unit: Option<String>,
	/// LegDividendAccrualPaymentDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42333")]
	pub leg_dividend_accrual_payment_date_offset_day_type: Option<i32>,
	/// LegDividendAccrualPaymentDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42334")]
	pub leg_dividend_accrual_payment_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the LegDateAdjustment component in InstrumentLeg. The
	/// value would be specific to this instance of LegDividendAccrualPaymentDate.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42335")]
	pub leg_dividend_accrual_payment_date_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the LegDateAdjustment component in InstrumentLeg. The values
	/// would be specific to this instance of LegDividendAccrualPaymentDate.
	#[serde(flatten)]
	pub leg_dividend_accrual_payment_date_business_center_grp: Option<super::leg_dividend_accrual_payment_date_business_center_grp::LegDividendAccrualPaymentDateBusinessCenterGrp>,
	/// LegDividendAccrualPaymentDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42336")]
	pub leg_dividend_accrual_payment_date_adjusted: Option<fix_common::LocalMktDate>,
}
