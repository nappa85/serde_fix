
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegCashSettlTermGrp {
	/// NoLegCashSettlTerms
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41344")]
	pub leg_cash_settl_terms: Option<fix_common::RepeatingValues<LegCashSettlTerm>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegCashSettlTerm {
	/// Required if NoLegCashSettlTerms(41344) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41345")]
	pub leg_cash_settl_currency: Option<String>,
	/// LegCasSettlValuationFirstBusinessDayOffset
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41346")]
	pub leg_cas_settl_valuation_first_business_day_offset: Option<i32>,
	/// LegCashSettlValuationSubsequentBusinessDaysOffset
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41347")]
	pub leg_cash_settl_valuation_subsequent_business_days_offset: Option<i32>,
	/// LegCashSettlNumOfValuationDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41348")]
	pub leg_cash_settl_num_of_valuation_dates: Option<i32>,
	/// LegCashSettlValuationTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41349")]
	pub leg_cash_settl_valuation_time: Option<String>,
	/// LegCashSettlBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41350")]
	pub leg_cash_settl_business_center: Option<String>,
	/// LegCashSettlQuoteMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41351")]
	pub leg_cash_settl_quote_method: Option<LegCashSettlQuoteMethod>,
	/// LegCashSettlQuoteAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41352")]
	pub leg_cash_settl_quote_amount: Option<f64>,
	/// LegCashSettlQuoteCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41353")]
	pub leg_cash_settl_quote_currency: Option<String>,
	/// LegCashSettlMinimumQuoteAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41354")]
	pub leg_cash_settl_minimum_quote_amount: Option<f64>,
	/// LegCashSettlMinimumQuoteCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41355")]
	pub leg_cash_settl_minimum_quote_currency: Option<String>,
	/// LegCashSettlBusinessDays
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41356")]
	pub leg_cash_settl_business_days: Option<i32>,
	/// LegCashSettlAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41357")]
	pub leg_cash_settl_amount: Option<f64>,
	/// LegCashSettlRecoveryFactor
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41358")]
	pub leg_cash_settl_recovery_factor: Option<f64>,
	/// LegCashSettlFixedTermIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41359")]
	pub leg_cash_settl_fixed_term_indicator: Option<fix_common::Boolean>,
	/// LegCashSettlAccruedInterestIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41360")]
	pub leg_cash_settl_accrued_interest_indicator: Option<fix_common::Boolean>,
	/// LegCashSettlValuationMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41361")]
	pub leg_cash_settl_valuation_method: Option<LegCashSettlValuationMethod>,
	/// LegCashSettlTermXID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41362")]
	pub leg_cash_settl_term_xid: Option<String>,
	/// LegCashSettlPriceSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42308")]
	pub leg_cash_settl_price_source: Option<String>,
	/// LegCashSettlPriceDefault
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42309")]
	pub leg_cash_settl_price_default: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegCashSettlQuoteMethod {
	/// Bid
	#[serde(rename = "0")]
	Bid,
	/// Mid
	#[serde(rename = "1")]
	Mid,
	/// Offer
	#[serde(rename = "2")]
	Offer,
}

impl Default for LegCashSettlQuoteMethod {
	fn default() -> Self {
		LegCashSettlQuoteMethod::Bid
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegCashSettlValuationMethod {
	/// Market
	#[serde(rename = "0")]
	Market,
	/// Highest
	#[serde(rename = "1")]
	Highest,
	/// Average market
	#[serde(rename = "2")]
	AverageMarket,
	/// Average highest
	#[serde(rename = "3")]
	AverageHighest,
	/// Blended market
	#[serde(rename = "4")]
	BlendedMarket,
	/// Blended highest
	#[serde(rename = "5")]
	BlendedHighest,
	/// Average blended market
	#[serde(rename = "6")]
	AverageBlendedMarket,
	/// Average blended highest
	#[serde(rename = "7")]
	AverageBlendedHighest,
}

impl Default for LegCashSettlValuationMethod {
	fn default() -> Self {
		LegCashSettlValuationMethod::Market
	}
}
