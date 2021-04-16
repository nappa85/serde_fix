
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegReturnRateGrp {
	/// NoLegReturnRates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42534")]
	pub leg_return_rates: Option<fix_common::RepeatingValues<LegReturnRate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegReturnRate {
	/// Required if NoLegReturnRates(42534) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42535")]
	pub leg_return_rate_price_sequence: Option<i32>,
	/// LegReturnRateCommissionBasis
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42536")]
	pub leg_return_rate_commission_basis: Option<char>,
	/// LegReturnRateCommissionAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42537")]
	pub leg_return_rate_commission_amount: Option<f64>,
	/// If not specified, this is defaulted to the reporting currency.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42538")]
	pub leg_return_rate_commission_currency: Option<String>,
	/// LegReturnRateTotalCommissionPerTrade
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42539")]
	pub leg_return_rate_total_commission_per_trade: Option<f64>,
	/// LegReturnRateDeterminationMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42540")]
	pub leg_return_rate_determination_method: Option<String>,
	/// LegReturnRateAmountRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42541")]
	pub leg_return_rate_amount_relative_to: Option<i32>,
	/// LegReturnRateQuoteMeasureType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42542")]
	pub leg_return_rate_quote_measure_type: Option<String>,
	/// LegReturnRateQuoteUnits
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42543")]
	pub leg_return_rate_quote_units: Option<String>,
	/// LegReturnRateQuoteMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42544")]
	pub leg_return_rate_quote_method: Option<i32>,
	/// LegReturnRateQuoteCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42545")]
	pub leg_return_rate_quote_currency: Option<String>,
	/// LegReturnRateQuoteCurrencyType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42546")]
	pub leg_return_rate_quote_currency_type: Option<String>,
	/// Mutually exclusive with LegReturnRateQuoteTime(42548).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42547")]
	pub leg_return_rate_quote_time_type: Option<i32>,
	/// Mutually exclusive with LegReturnRateQuoteTimeType(42547).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42548")]
	pub leg_return_rate_quote_time: Option<String>,
	/// LegReturnRateQuoteDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42549")]
	pub leg_return_rate_quote_date: Option<fix_common::LocalMktDate>,
	/// LegReturnRateQuoteExpirationTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42550")]
	pub leg_return_rate_quote_expiration_time: Option<String>,
	/// LegReturnRateQuoteBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42551")]
	pub leg_return_rate_quote_business_center: Option<String>,
	/// LegReturnRateQuoteExchange
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42552")]
	pub leg_return_rate_quote_exchange: Option<String>,
	/// LegReturnRateQuotePricingModel
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42553")]
	pub leg_return_rate_quote_pricing_model: Option<String>,
	/// LegReturnRateCashFlowType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42554")]
	pub leg_return_rate_cash_flow_type: Option<String>,
	/// Mutually exclusive with LegReturnRateValuationTime(42556).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42555")]
	pub leg_return_rate_valuation_time_type: Option<i32>,
	/// Mutually exclusive with LegReturnRateValuationTimeType(42555).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42556")]
	pub leg_return_rate_valuation_time: Option<String>,
	/// LegReturnRateValuationTimeBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42557")]
	pub leg_return_rate_valuation_time_business_center: Option<String>,
	/// LegReturnRateValuationPriceOption
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42558")]
	pub leg_return_rate_valuation_price_option: Option<i32>,
	/// LegReturnRateFinalPriceFallback
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42559")]
	pub leg_return_rate_final_price_fallback: Option<i32>,
}
