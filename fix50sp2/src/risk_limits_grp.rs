
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RiskLimitsGrp {
	/// NoRiskLimits
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1669")]
	pub risk_limits: Option<fix_common::RepeatingValues<RiskLimit>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RiskLimit {
    #[serde(flatten)]
    pub risk_limit_types_grp: Option<super::risk_limit_types_grp::RiskLimitTypesGrp>,
    #[serde(flatten)]
    pub risk_instrument_scope_grp: Option<super::risk_instrument_scope_grp::RiskInstrumentScopeGrp>,
}
