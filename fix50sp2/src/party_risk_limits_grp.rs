
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PartyRiskLimitsGrp {
	/// NoPartyRiskLimits
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1677")]
	pub party_risk_limits: Option<fix_common::RepeatingValues<PartyRiskLimit>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PartyRiskLimit {
	/// RiskLimitID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1670")]
	pub risk_limit_id: Option<String>,
	/// RiskLimitCheckModelType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2339")]
	pub risk_limit_check_model_type: Option<RiskLimitCheckModelType>,
	/// PartyRiskLimitStatus
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2355")]
	pub party_risk_limit_status: Option<PartyRiskLimitStatus>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RiskLimitCheckModelType {
	/// None (default if not specified)
	#[serde(rename = "0")]
	None,
	/// PlusOne model
	#[serde(rename = "1")]
	PlusOneModel,
	/// Ping model
	#[serde(rename = "2")]
	PingModel,
	/// Push model
	#[serde(rename = "3")]
	PushModel,
}

impl Default for RiskLimitCheckModelType {
	fn default() -> Self {
		RiskLimitCheckModelType::None
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PartyRiskLimitStatus {
	/// Disabled
	#[serde(rename = "0")]
	Disabled,
	/// Enabled
	#[serde(rename = "1")]
	Enabled,
}

impl Default for PartyRiskLimitStatus {
	fn default() -> Self {
		PartyRiskLimitStatus::Disabled
	}
}
