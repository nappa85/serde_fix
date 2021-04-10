
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingProtectionTermObligationGrp {
	/// NoUnderlyingProtectionTermObligations
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42087")]
	pub underlying_protection_term_obligations: Option<crate::entities::RepeatingValues<UnderlyingProtectionTermObligation>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingProtectionTermObligation {
	/// Required if NoUnderlyingProtectionTermObligations(42087) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42088")]
	pub underlying_protection_term_obligation_type: Option<String>,
	/// UnderlyingProtectionTermObligationValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42089")]
	pub underlying_protection_term_obligation_value: Option<String>,
}
