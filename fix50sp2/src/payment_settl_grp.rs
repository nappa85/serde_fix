
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentSettlGrp {
	/// NoPaymentSettls
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40230")]
	pub payment_settls: Option<fix_common::RepeatingValues<PaymentSettl>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentSettl {
	/// Required if NoPaymentSettls(40230) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40231")]
	pub payment_settl_amount: Option<f64>,
	/// PaymentSettlCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40232")]
	pub payment_settl_currency: Option<String>,
}
