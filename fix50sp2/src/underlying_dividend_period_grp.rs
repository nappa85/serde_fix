
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingDividendPeriodGrp {
	/// NoUnderlyingDividendPeriods
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42862")]
	pub underlying_dividend_periods: Option<crate::entities::RepeatingValues<UnderlyingDividendPeriod>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingDividendPeriod {
	/// Required if NoUnderlyingDividendPeriods(42862) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42863")]
	pub underlying_dividend_period_sequence: Option<i32>,
	/// UnderlyingDividendPeriodStartDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42864")]
	pub underlying_dividend_period_start_date_unadjusted: Option<crate::entities::LocalMktDate>,
	/// UnderlyingDividendPeriodEndDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42865")]
	pub underlying_dividend_period_end_date_unadjusted: Option<crate::entities::LocalMktDate>,
	/// When specified, this overrides UnderlyingDividendUnderlierRefID(42829). The specified value would be specific to this dividend
	/// period instance.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42866")]
	pub underlying_dividend_period_underlier_ref_id: Option<String>,
	/// UnderlyingDividendPeriodStrikePrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42867")]
	pub underlying_dividend_period_strike_price: Option<f64>,
	/// When specified, this overrides the business day convention defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified value would be specific to this dividend period instance.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42868")]
	pub underlying_dividend_period_business_day_convention: Option<i32>,
	/// UnderlyingDividendPeriodValuationDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42869")]
	pub underlying_dividend_period_valuation_date_unadjusted: Option<crate::entities::LocalMktDate>,
	/// UnderlyingDividendPeriodValuationDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42870")]
	pub underlying_dividend_period_valuation_date_relative_to: Option<i32>,
	/// Conditionally required when UnderlyingDividendPeriodValuationDateOffsetUnit(42872) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42871")]
	pub underlying_dividend_period_valuation_date_offset_period: Option<i32>,
	/// Conditionally required when UnderlyingDividendPeriodValuationDateOffsetPeriod(42871) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42872")]
	pub underlying_dividend_period_valuation_date_offset_unit: Option<String>,
	/// UnderlyingDividendPeriodValuationDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42873")]
	pub underlying_dividend_period_valuation_date_offset_day_type: Option<i32>,
	/// UnderlyingDividendPeriodValuationDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42874")]
	pub underlying_dividend_period_valuation_date_adjusted: Option<crate::entities::LocalMktDate>,
	/// UnderlyingDividendPeriodPaymentDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42875")]
	pub underlying_dividend_period_payment_date_unadjusted: Option<crate::entities::LocalMktDate>,
	/// UnderlyingDividendPeriodPaymentDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42876")]
	pub underlying_dividend_period_payment_date_relative_to: Option<i32>,
	/// Conditionally required when UnderlyingDividendPeriodPaymentDateOffsetUnit(42878) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42877")]
	pub underlying_dividend_period_payment_date_offset_period: Option<i32>,
	/// Conditionally required when UnderlyingDividendPeriodPaymentDateOffsetPeriod(42877) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42878")]
	pub underlying_dividend_period_payment_date_offset_unit: Option<String>,
	/// UnderlyingDividendPeriodPaymentDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42879")]
	pub underlying_dividend_period_payment_date_offset_day_type: Option<i32>,
	/// UnderlyingDividendPeriodPaymentDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42880")]
	pub underlying_dividend_period_payment_date_adjusted: Option<crate::entities::LocalMktDate>,
	/// UnderlyingDividendPeriodXID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42881")]
	pub underlying_dividend_period_xid: Option<String>,
}
