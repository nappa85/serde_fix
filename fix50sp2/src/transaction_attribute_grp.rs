
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TransactionAttributeGrp {
	/// NoTransactionAttributes
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2871")]
	pub transaction_attributes: Option<fix_common::RepeatingValues<TransactionAttribute>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TransactionAttribute {
	/// Required if NoTransactionAttributes(2871) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2872")]
	pub transaction_attribute_type: Option<TransactionAttributeType>,
	/// TransactionAttributeValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2873")]
	pub transaction_attribute_value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TransactionAttributeType {
	/// Exclusive arrangement
	#[serde(rename = "0")]
	ExclusiveArrangement,
	/// Collateral reuse
	#[serde(rename = "1")]
	CollateralReuse,
	/// Collateral arrangement type
	#[serde(rename = "2")]
	CollateralArrangementType,
}

impl Default for TransactionAttributeType {
	fn default() -> Self {
		TransactionAttributeType::ExclusiveArrangement
	}
}
