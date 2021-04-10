
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPhysicalSettlTermGrp {
	/// NoLegPhysicalSettlTerms
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41599")]
	pub leg_physical_settl_terms: Option<fix_common::RepeatingValues<LegPhysicalSettlTerm>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPhysicalSettlTerm {
	/// LegPhysicalSettlCurency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41601")]
	pub leg_physical_settl_curency: Option<String>,
	/// LegPhysicalSettlBusinessDays
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41602")]
	pub leg_physical_settl_business_days: Option<i32>,
	/// LegPhysicalSettlMaximumBusinessDays
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41603")]
	pub leg_physical_settl_maximum_business_days: Option<i32>,
	/// LegPhysicalSettlTermXID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41600")]
	pub leg_physical_settl_term_xid: Option<String>,
}
