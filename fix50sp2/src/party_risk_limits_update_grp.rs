
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PartyRiskLimitsUpdateGrp {
	/// NoPartyRiskLimits
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1677")]
	pub party_risk_limits: Option<fix_common::RepeatingValues<PartyRiskLimit>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PartyRiskLimit {
	/// <p>Required if NoPartyRiskLimits(1677) &gt; 0</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1324")]
	pub list_update_action: Option<ListUpdateAction>,
	/// <p>Conditionally required when PartyDetailGrp component is not provided</p>
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ListUpdateAction {
	/// Add
	#[serde(rename = "A")]
	Add,
	/// Delete
	#[serde(rename = "D")]
	Delete,
	/// Modify
	#[serde(rename = "M")]
	Modify,
	/// Snapshot
	#[serde(rename = "S")]
	Snapshot,
}

impl Default for ListUpdateAction {
	fn default() -> Self {
		ListUpdateAction::Add
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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
