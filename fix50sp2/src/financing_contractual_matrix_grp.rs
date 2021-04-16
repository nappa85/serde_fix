
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FinancingContractualMatrixGrp {
	/// NoContractualMatrices
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40042")]
	pub contractual_matrices: Option<fix_common::RepeatingValues<ContractualMatrice>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ContractualMatrice {
	/// Required if NoContractualMatrices(40042) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40043")]
	pub contractual_matrix_source: Option<String>,
	/// ContractualMatrixDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40044")]
	pub contractual_matrix_date: Option<fix_common::LocalMktDate>,
	/// ContractualMatrixTerm
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40045")]
	pub contractual_matrix_term: Option<String>,
}
