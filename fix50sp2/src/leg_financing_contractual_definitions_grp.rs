
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegFinancingContractualDefinitionsGrp {
	/// NoLegContractualDefinitions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42198")]
	pub leg_contractual_definitions: Option<fix_common::RepeatingValues<LegContractualDefinition>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegContractualDefinition {
	/// Required if <a href="tag_42199_LegContractualDefinition.html" target="bottom">NoLegContractualDefinitions&nbsp;(42199)</a> &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42199")]
	pub leg_contractual_definition: Option<String>,
}
