
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegFinancingContractualMatrixGrp {
	/// NoLegContractualMatrices
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42203")]
	pub leg_contractual_matrices: Option<fix_common::RepeatingValues<LegContractualMatrice>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegContractualMatrice {
	/// Required if <a href="tag_42203_NoLegContractualMatrices.html" target="bottom">NoLegContractualMatrices&nbsp;(42203)</a> &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42204")]
	pub leg_contractual_matrix_source: Option<String>,
	/// LegContractualMatrixDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42205")]
	pub leg_contractual_matrix_date: Option<fix_common::LocalMktDate>,
	/// LegContractualMatrixTerm
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42206")]
	pub leg_contractual_matrix_term: Option<String>,
}
