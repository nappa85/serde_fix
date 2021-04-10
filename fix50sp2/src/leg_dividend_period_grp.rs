
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegDividendPeriodGrp {
	/// NoLegDividendPeriods
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42366")]
	pub leg_dividend_periods: Option<crate::entities::RepeatingValues<LegDividendPeriod>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegDividendPeriod {
	/// Required if NoLegDividendPeriods(42366) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42367")]
	pub leg_dividend_period_sequence: Option<i32>,
	/// LegDividendPeriodStartDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42368")]
	pub leg_dividend_period_start_date_unadjusted: Option<crate::entities::LocalMktDate>,
	/// LegDividendPeriodEndDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42369")]
	pub leg_dividend_period_end_date_unadjusted: Option<crate::entities::LocalMktDate>,
	/// When specified, this overrides LegDividendUnderlierRefID(42340). The specified value would be specific to this dividend period
	/// instance.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42370")]
	pub leg_dividend_period_underlier_ref_id: Option<String>,
	/// LegDividendPeriodStrikePrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42371")]
	pub leg_dividend_period_strike_price: Option<f64>,
	/// When specified, this overrides the business day convention defined in the LegDateAdjustment component in InstrumentLeg. The
	/// specified value would be specific to this dividend period instance.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42372")]
	pub leg_dividend_period_business_day_convention: Option<i32>,
	/// LegDividendPeriodValuationDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42373")]
	pub leg_dividend_period_valuation_date_unadjusted: Option<crate::entities::LocalMktDate>,
	/// LegDividendPeriodValuationDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42374")]
	pub leg_dividend_period_valuation_date_relative_to: Option<i32>,
	/// Conditionally required when LegDividendPeriodValuationDateOffsetUnit(42376) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42375")]
	pub leg_dividend_period_valuation_date_offset_period: Option<i32>,
	/// Conditionally required when LegDividendPeriodValuationDateOffsetPeriod(42375) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42376")]
	pub leg_dividend_period_valuation_date_offset_unit: Option<String>,
	/// LegDividendPeriodValuationDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42377")]
	pub leg_dividend_period_valuation_date_offset_day_type: Option<i32>,
	/// LegDividendPeriodValuationDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42378")]
	pub leg_dividend_period_valuation_date_adjusted: Option<crate::entities::LocalMktDate>,
	/// LegDividendPeriodPaymentDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42379")]
	pub leg_dividend_period_payment_date_unadjusted: Option<crate::entities::LocalMktDate>,
	/// LegDividendPeriodPaymentDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42380")]
	pub leg_dividend_period_payment_date_relative_to: Option<i32>,
	/// Conditionally required when LegDividendPeriodPaymentDateOffsetUnit(42382) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42381")]
	pub leg_dividend_period_payment_date_offset_period: Option<i32>,
	/// Conditionally required when LegDividendPeriodPaymentDateOffsetPeriod(42381) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42382")]
	pub leg_dividend_period_payment_date_offset_unit: Option<String>,
	/// LegDividendPeriodPaymentDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42383")]
	pub leg_dividend_period_payment_date_offset_day_type: Option<i32>,
	/// LegDividendPeriodPaymentDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42384")]
	pub leg_dividend_period_payment_date_adjusted: Option<crate::entities::LocalMktDate>,
	/// LegDividendPeriodXID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42385")]
	pub leg_dividend_period_xid: Option<String>,
}
