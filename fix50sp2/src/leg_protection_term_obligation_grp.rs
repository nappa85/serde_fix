
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProtectionTermObligationGrp {
	/// NoLegProtectionTermObligations
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41635")]
	pub leg_protection_term_obligations: Option<crate::entities::RepeatingValues<LegProtectionTermObligation>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProtectionTermObligation {
	/// Required if NoLegProtectionTermObligations(41635) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41636")]
	pub leg_protection_term_obligation_type: Option<String>,
	/// LegProtectionTermObligationValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41637")]
	pub leg_protection_term_obligation_value: Option<String>,
}
