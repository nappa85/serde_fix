
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingReturnRateGrp {
	/// NoUnderlyingReturnRates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43034")]
	pub underlying_return_rates: Option<fix_common::RepeatingValues<UnderlyingReturnRate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingReturnRate {
	/// Required if NoUnderlyingReturnRates(43034) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43035")]
	pub underlying_return_rate_price_sequence: Option<i32>,
	/// UnderlyingReturnRateCommissionBasis
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43036")]
	pub underlying_return_rate_commission_basis: Option<char>,
	/// UnderlyingReturnRateCommissionAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43037")]
	pub underlying_return_rate_commission_amount: Option<f64>,
	/// If not specified, this is defaulted to the reporting currency.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43038")]
	pub underlying_return_rate_commission_currency: Option<String>,
	/// UnderlyingReturnRateTotalCommissionPerTrade
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43039")]
	pub underlying_return_rate_total_commission_per_trade: Option<f64>,
	/// UnderlyingReturnRateDeterminationMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43040")]
	pub underlying_return_rate_determination_method: Option<String>,
	/// UnderlyingReturnRateAmountRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43041")]
	pub underlying_return_rate_amount_relative_to: Option<i32>,
	/// UnderlyingReturnRateQuoteMeasureType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43042")]
	pub underlying_return_rate_quote_measure_type: Option<String>,
	/// UnderlyingReturnRateQuoteUnits
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43043")]
	pub underlying_return_rate_quote_units: Option<String>,
	/// UnderlyingReturnRateQuoteMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43044")]
	pub underlying_return_rate_quote_method: Option<i32>,
	/// UnderlyingReturnRateQuoteCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43045")]
	pub underlying_return_rate_quote_currency: Option<String>,
	/// UnderlyingReturnRateQuoteCurrencyType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43046")]
	pub underlying_return_rate_quote_currency_type: Option<String>,
	/// Mutually exclusive with UnderlyingReturnRateQuoteTime(43048).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43047")]
	pub underlying_return_rate_quote_time_type: Option<i32>,
	/// Mutually exclusive with UnderlyingReturnRateQuoteTimeType(43047).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43048")]
	pub underlying_return_rate_quote_time: Option<fix_common::LocalMktDate>,
	/// UnderlyingReturnRateQuoteDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43049")]
	pub underlying_return_rate_quote_date: Option<fix_common::LocalMktDate>,
	/// UnderlyingReturnRateQuoteExpirationTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43050")]
	pub underlying_return_rate_quote_expiration_time: Option<String>,
	/// UnderlyingReturnRateQuoteBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43051")]
	pub underlying_return_rate_quote_business_center: Option<String>,
	/// UnderlyingReturnRateQuoteExchange
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43052")]
	pub underlying_return_rate_quote_exchange: Option<String>,
	/// UnderlyingReturnRateQuotePricingModel
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43053")]
	pub underlying_return_rate_quote_pricing_model: Option<String>,
	/// UnderlyingReturnRateCashFlowType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43054")]
	pub underlying_return_rate_cash_flow_type: Option<String>,
	/// Mutually exclusive with UnderlyingReturnRateValuationTime(43056)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43055")]
	pub underlying_return_rate_valuation_time_type: Option<i32>,
	/// Mutually exclusive with UnderlyingReturnRateValuationTimeType(43055).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43056")]
	pub underlying_return_rate_valuation_time: Option<String>,
	/// UnderlyingReturnRateValuationTimeBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43057")]
	pub underlying_return_rate_valuation_time_business_center: Option<String>,
	/// UnderlyingReturnRateValuationPriceOption
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43058")]
	pub underlying_return_rate_valuation_price_option: Option<i32>,
	/// UnderlyingReturnRateFinalPriceFallback
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43059")]
	pub underlying_return_rate_final_price_fallback: Option<i32>,
}
