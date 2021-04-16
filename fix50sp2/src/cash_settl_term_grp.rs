
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CashSettlTermGrp {
	/// NoCashSettlTerms
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40022")]
	pub cash_settl_terms: Option<fix_common::RepeatingValues<CashSettlTerm>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CashSettlTerm {
	/// Required if NoCashSettlTerms(40022) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40023")]
	pub cash_settl_currency: Option<String>,
	/// CashSettlValuationFirstBusinessDayOffset
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40024")]
	pub cash_settl_valuation_first_business_day_offset: Option<i32>,
	/// CashSettlValuationSubsequentBusinessDaysOffset
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40916")]
	pub cash_settl_valuation_subsequent_business_days_offset: Option<i32>,
	/// CashSettlNumOfValuationDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40917")]
	pub cash_settl_num_of_valuation_dates: Option<i32>,
	/// CashSettlValuationTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40025")]
	pub cash_settl_valuation_time: Option<String>,
	/// CashSettlBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40026")]
	pub cash_settl_business_center: Option<String>,
	/// CashSettlQuoteMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40027")]
	pub cash_settl_quote_method: Option<CashSettlQuoteMethod>,
	/// CashSettlQuoteAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40028")]
	pub cash_settl_quote_amount: Option<f64>,
	/// CashSettlQuoteCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40029")]
	pub cash_settl_quote_currency: Option<String>,
	/// CashSettlMinimumQuoteAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40030")]
	pub cash_settl_minimum_quote_amount: Option<f64>,
	/// CashSettlMinimumQuoteCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40031")]
	pub cash_settl_minimum_quote_currency: Option<String>,
	/// CashSettlBusinessDays
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40033")]
	pub cash_settl_business_days: Option<i32>,
	/// CashSettlAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40034")]
	pub cash_settl_amount: Option<f64>,
	/// CashSettlRecoveryFactor
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40035")]
	pub cash_settl_recovery_factor: Option<f64>,
	/// CashSettlFixedTermIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40036")]
	pub cash_settl_fixed_term_indicator: Option<fix_common::Boolean>,
	/// CashSettlAccruedInterestIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40037")]
	pub cash_settl_accrued_interest_indicator: Option<fix_common::Boolean>,
	/// CashSettlValuationMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40038")]
	pub cash_settl_valuation_method: Option<CashSettlValuationMethod>,
	/// CashSettlTermXID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40039")]
	pub cash_settl_term_xid: Option<String>,
	/// CashSettlPriceSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42216")]
	pub cash_settl_price_source: Option<String>,
	/// CashSettlPriceDefault
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42217")]
	pub cash_settl_price_default: Option<CashSettlPriceDefault>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum CashSettlQuoteMethod {
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

impl Default for CashSettlQuoteMethod {
	fn default() -> Self {
		CashSettlQuoteMethod::Bid
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum CashSettlValuationMethod {
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

impl Default for CashSettlValuationMethod {
	fn default() -> Self {
		CashSettlValuationMethod::Market
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum CashSettlPriceDefault {
	/// Close
	#[serde(rename = "0")]
	Close,
	/// Hedge
	#[serde(rename = "1")]
	Hedge,
}

impl Default for CashSettlPriceDefault {
	fn default() -> Self {
		CashSettlPriceDefault::Close
	}
}
