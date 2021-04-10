
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PhysicalSettlTermGrp {
	/// NoPhysicalSettlTerms
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40204")]
	pub physical_settl_terms: Option<fix_common::RepeatingValues<PhysicalSettlTerm>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PhysicalSettlTerm {
	/// PhysicalSettlCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40205")]
	pub physical_settl_currency: Option<String>,
	/// PhysicalSettlBusinessDays
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40206")]
	pub physical_settl_business_days: Option<i32>,
	/// PhysicalSettlMaximumBusinessDays
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40207")]
	pub physical_settl_maximum_business_days: Option<i32>,
	/// PhysicalSettlTermXID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40208")]
	pub physical_settl_term_xid: Option<String>,
}
