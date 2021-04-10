
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FinancingContractualDefinitionGrp {
	/// NoContractualDefinitions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40040")]
	pub contractual_definitions: Option<crate::entities::RepeatingValues<ContractualDefinition>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ContractualDefinition {
	/// Required if NoContractualDefinitions(40040) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40041")]
	pub contractual_definition: Option<String>,
}
