
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReturnRateGrp {
	/// NoReturnRates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42735")]
	pub return_rates: Option<fix_common::RepeatingValues<ReturnRate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReturnRate {
	/// Required if NoReturnRates(42735) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42736")]
	pub return_rate_price_sequence: Option<ReturnRatePriceSequence>,
	/// ReturnRateCommissionBasis
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42737")]
	pub return_rate_commission_basis: Option<char>,
	/// ReturnRateCommissionAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42738")]
	pub return_rate_commission_amount: Option<f64>,
	/// If not specified, this is defaulted to the reporting currency.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42739")]
	pub return_rate_commission_currency: Option<String>,
	/// ReturnRateTotalCommissionPerTrade
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42740")]
	pub return_rate_total_commission_per_trade: Option<f64>,
	/// ReturnRateDeterminationMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42741")]
	pub return_rate_determination_method: Option<String>,
	/// ReturnRateAmountRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42742")]
	pub return_rate_amount_relative_to: Option<i32>,
	/// ReturnRateQuoteMeasureType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42743")]
	pub return_rate_quote_measure_type: Option<String>,
	/// ReturnRateQuoteUnits
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42744")]
	pub return_rate_quote_units: Option<String>,
	/// ReturnRateQuoteMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42745")]
	pub return_rate_quote_method: Option<i32>,
	/// ReturnRateQuoteCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42746")]
	pub return_rate_quote_currency: Option<String>,
	/// ReturnRateQuoteCurrencyType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42747")]
	pub return_rate_quote_currency_type: Option<String>,
	/// Mutually exclusive with ReturnRateQuoteTime(42749).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42748")]
	pub return_rate_quote_time_type: Option<ReturnRateQuoteTimeType>,
	/// Mutually exclusive with ReturnRateQuoteTimeType(42748).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42749")]
	pub return_rate_quote_time: Option<String>,
	/// ReturnRateQuoteDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42750")]
	pub return_rate_quote_date: Option<fix_common::LocalMktDate>,
	/// ReturnRateQuoteExpirationTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42751")]
	pub return_rate_quote_expiration_time: Option<String>,
	/// ReturnRateQuoteBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42752")]
	pub return_rate_quote_business_center: Option<String>,
	/// ReturnRateQuoteExchange
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42753")]
	pub return_rate_quote_exchange: Option<String>,
	/// ReturnRateQuotePricingModel
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42754")]
	pub return_rate_quote_pricing_model: Option<String>,
	/// ReturnRateCashFlowType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42755")]
	pub return_rate_cash_flow_type: Option<String>,
	/// Mutually exclusive with ReturnRateValuationTime(42757).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42756")]
	pub return_rate_valuation_time_type: Option<i32>,
	/// Mutually exclusive with ReturnRateValuationTimeType(42756).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42757")]
	pub return_rate_valuation_time: Option<String>,
	/// ReturnRateValuationTimeBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42758")]
	pub return_rate_valuation_time_business_center: Option<String>,
	/// ReturnRateValuationPriceOption
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42759")]
	pub return_rate_valuation_price_option: Option<ReturnRateValuationPriceOption>,
	/// ReturnRateFinalPriceFallback
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42760")]
	pub return_rate_final_price_fallback: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ReturnRatePriceSequence {
	/// Initial
	#[serde(rename = "0")]
	Initial,
	/// Interim
	#[serde(rename = "1")]
	Interim,
	/// Final
	#[serde(rename = "2")]
	Final,
}

impl Default for ReturnRatePriceSequence {
	fn default() -> Self {
		ReturnRatePriceSequence::Initial
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ReturnRateQuoteTimeType {
	/// Open
	#[serde(rename = "0")]
	Open,
	/// Official settlement price time
	#[serde(rename = "1")]
	OfficialSettlementPriceTime,
	/// XETRA
	#[serde(rename = "2")]
	Xetra,
	/// Close
	#[serde(rename = "3")]
	Close,
	/// Derivatives close
	#[serde(rename = "4")]
	DerivativesClose,
	/// High
	#[serde(rename = "5")]
	High,
	/// Low
	#[serde(rename = "6")]
	Low,
	/// As specified in the master confirmation
	#[serde(rename = "7")]
	AsSpecifiedInTheMasterConfirmation,
}

impl Default for ReturnRateQuoteTimeType {
	fn default() -> Self {
		ReturnRateQuoteTimeType::Open
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ReturnRateValuationPriceOption {
	/// None (the default)
	#[serde(rename = "0")]
	None,
	/// Futures price
	#[serde(rename = "1")]
	FuturesPrice,
	/// Options price
	#[serde(rename = "2")]
	OptionsPrice,
}

impl Default for ReturnRateValuationPriceOption {
	fn default() -> Self {
		ReturnRateValuationPriceOption::None
	}
}
