
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProtectionTermObligationGrp {
	/// NoProtectionTermObligations
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40201")]
	pub protection_term_obligations: Option<crate::entities::RepeatingValues<ProtectionTermObligation>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProtectionTermObligation {
	/// Required if NoProtectionTermObligations(40201) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40202")]
	pub protection_term_obligation_type: Option<String>,
	/// ProtectionTermObligationValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40203")]
	pub protection_term_obligation_value: Option<String>,
}
