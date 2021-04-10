
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingAmount {
	/// Number of Distribution instructions in this message (number of repeating groups to follow).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "984")]
	pub underlying_amounts: Option<fix_common::RepeatingValues<UnderlyingAmountElement>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingAmountElement {
	/// Amount to pay in order to receive the underlying instrument.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "985")]
	pub underlying_pay_amount: Option<f64>,
	/// Amount to collect in order to deliver the underlying instrument.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "986")]
	pub underlying_collect_amount: Option<f64>,
	/// Date the underlying instrument will settle. Used for derivatives that deliver into more than one underlying instrument. Settlement
	/// dates can vary across underlying instruments.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "987")]
	pub underlying_settlement_date: Option<fix_common::LocalMktDate>,
	/// Settlement status of the underlying instrument. Used for derivatives that deliver into more than one underlying instrument.
	/// Settlement can be delayed for an underlying instrument.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "988")]
	pub underlying_settlement_status: Option<String>,
}
