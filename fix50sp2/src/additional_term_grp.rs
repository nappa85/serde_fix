
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AdditionalTermGrp {
	/// NoAdditionalTerms
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40019")]
	pub additional_terms: Option<fix_common::RepeatingValues<AdditionalTerm>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AdditionalTerm {
	/// Required if NoAdditionalTerms(40019) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40020")]
	pub additional_term_condition_precedent_bond_indicator: Option<fix_common::Boolean>,
	/// AdditionalTermDiscrepancyClauseIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40021")]
	pub additional_term_discrepancy_clause_indicator: Option<fix_common::Boolean>,
}
