
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RiskLimitsGrp {
	/// NoRiskLimits
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1669")]
	pub risk_limits: Option<crate::entities::RepeatingValues<RiskLimit>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RiskLimit {
}
