
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingAdditionalTermGrp {
	/// NoUnderlyingAdditionalTerms
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42036")]
	pub underlying_additional_terms: Option<crate::entities::RepeatingValues<UnderlyingAdditionalTerm>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingAdditionalTerm {
	/// Required if NoUnderlyingAdditionalTerms(42036) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42037")]
	pub underlying_additional_term_condition_precedent_bond_indicator: Option<crate::entities::Boolean>,
	/// UnderlyingAdditionalTermDiscrepancyClauseIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42038")]
	pub underlying_additional_term_discrepancy_clause_indicator: Option<crate::entities::Boolean>,
}
