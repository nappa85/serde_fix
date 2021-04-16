
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegAdditionalTermGrp {
	/// NoLegAdditionalTerms
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41335")]
	pub leg_additional_terms: Option<fix_common::RepeatingValues<LegAdditionalTerm>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegAdditionalTerm {
	/// Required if NoLegAdditionalTerms(41335) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41336")]
	pub leg_additional_term_condition_precedent_bond_indicator: Option<fix_common::Boolean>,
	/// LegAdditionalTermDiscrepancyClauseIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41337")]
	pub leg_additional_term_discrepancy_clause_indicator: Option<fix_common::Boolean>,
}
