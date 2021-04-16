
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingDividendPaymentGrp {
	/// NoUnderlyingDividendPayments
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42855")]
	pub underlying_dividend_payments: Option<fix_common::RepeatingValues<UnderlyingDividendPayment>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingDividendPayment {
	/// Required if NoUnderlyingDividendPayments (42855) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42856")]
	pub underlying_dividend_payment_date: Option<fix_common::LocalMktDate>,
	/// Required if NoUnderlyingDividendPayments (42855) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42857")]
	pub underlying_dividend_payment_amount: Option<f64>,
	/// UnderlyingDividendPaymentCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42858")]
	pub underlying_dividend_payment_currency: Option<String>,
	/// UnderlyingDividendAccruedInterest
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42859")]
	pub underlying_dividend_accrued_interest: Option<f64>,
}
