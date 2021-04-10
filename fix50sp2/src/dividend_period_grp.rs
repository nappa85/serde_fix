
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DividendPeriodGrp {
	/// NoDividendPeriods
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42274")]
	pub dividend_periods: Option<fix_common::RepeatingValues<DividendPeriod>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DividendPeriod {
	/// Required if NoDividendPeriods(42274) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42275")]
	pub dividend_period_sequence: Option<i32>,
	/// DividendPeriodStartDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42276")]
	pub dividend_period_start_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// DividendPeriodEndDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42277")]
	pub dividend_period_end_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// When specified, this overrides DividendUnderlierRefID(42248). The specified value would be specific to this dividend period
	/// instance.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42278")]
	pub dividend_period_underlier_ref_id: Option<String>,
	/// DividendPeriodStrikePrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42279")]
	pub dividend_period_strike_price: Option<f64>,
	/// When specified, this overrides the business day convention defined in the DateAdjustment component in Instrument. The specified
	/// value would be specific to this dividend period instance.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42280")]
	pub dividend_period_business_day_convention: Option<i32>,
	/// DividendPeriodValuationDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42281")]
	pub dividend_period_valuation_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// DividendPeriodValuationDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42282")]
	pub dividend_period_valuation_date_relative_to: Option<i32>,
	/// Conditionally required when DividendPeriodValuationDateOffsetUnit(42284) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42283")]
	pub dividend_period_valuation_date_offset_period: Option<i32>,
	/// Conditionally required when DividendPeriodValuationDateOffsetPeriod(42283) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42284")]
	pub dividend_period_valuation_date_offset_unit: Option<String>,
	/// DividendPeriodValuationDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42285")]
	pub dividend_period_valuation_date_offset_day_type: Option<i32>,
	/// DividendPeriodValuationDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42286")]
	pub dividend_period_valuation_date_adjusted: Option<fix_common::LocalMktDate>,
	/// DividendPeriodPaymentDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42287")]
	pub dividend_period_payment_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// DividendPeriodPaymentDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42288")]
	pub dividend_period_payment_date_relative_to: Option<i32>,
	/// Conditionally required when DividendPeriodPaymentDateOffsetUnit(42290) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42289")]
	pub dividend_period_payment_date_offset_period: Option<i32>,
	/// Conditionally required when DividendPeriodPaymentDateOffsetPeriod(42289) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42290")]
	pub dividend_period_payment_date_offset_unit: Option<String>,
	/// DividendPeriodPaymentDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42291")]
	pub dividend_period_payment_date_offset_day_type: Option<i32>,
	/// DividendPeriodPaymentDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42292")]
	pub dividend_period_payment_date_adjusted: Option<fix_common::LocalMktDate>,
	/// DividendPeriodXID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42293")]
	pub dividend_period_xid: Option<String>,
}
