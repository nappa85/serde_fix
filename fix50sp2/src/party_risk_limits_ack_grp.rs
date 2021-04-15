
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PartyRiskLimitsAckGrp {
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
	/// <p>Required if NoPartyRiskLimits(1677) &gt; 0</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1763")]
	pub risk_limit_status: Option<RiskLimitStatus>,
	/// RiskLimitResult
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1764")]
	pub risk_limit_result: Option<RiskLimitResult>,
	/// <p>Conditionally required when PartyDetailGrp component is not provided</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1670")]
	pub risk_limit_id: Option<String>,
	/// RejectText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1328")]
	pub reject_text: Option<String>,
	/// Must be set if EncodedRejectText(1665) field is specified and must immediately precede it.
	#[serde(rename = "1664")]
	/// Encoded (non-ASCII characters) representation of the RejectText(1328) field in the encoded format specified via the MessageEncoding(347)
	/// field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "1665")]
	pub encoded_reject_text: Option<fix_common::EncodedText<1665>>,
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
pub enum RiskLimitStatus {
	/// Accepted
	#[serde(rename = "0")]
	Accepted,
	/// Accepted with changes
	#[serde(rename = "1")]
	AcceptedWithChanges,
	/// Rejected
	#[serde(rename = "2")]
	Rejected,
}

impl Default for RiskLimitStatus {
	fn default() -> Self {
		RiskLimitStatus::Accepted
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum RiskLimitResult {
	/// Successful
	#[serde(rename = "0")]
	Successful,
	/// Invalid party
	#[serde(rename = "1")]
	InvalidParty,
	/// Invalid related party
	#[serde(rename = "2")]
	InvalidRelatedParty,
	/// Invalid risk limit type
	#[serde(rename = "3")]
	InvalidRiskLimitType,
	/// Invalid risk limit ID
	#[serde(rename = "4")]
	InvalidRiskLimitId,
	/// Invalid risk limit amount
	#[serde(rename = "5")]
	InvalidRiskLimitAmount,
	/// Invalid risk warning level action
	#[serde(rename = "6")]
	InvalidRiskWarningLevelAction,
	/// Invalid risk instrument scope
	#[serde(rename = "7")]
	InvalidRiskInstrumentScope,
	/// Risk limit actions not supported
	#[serde(rename = "8")]
	RiskLimitActionsNotSupported,
	/// Warning levels not supported
	#[serde(rename = "9")]
	WarningLevelsNotSupported,
	/// Warning level actions not supported
	#[serde(rename = "10")]
	WarningLevelActionsNotSupported,
	/// Risk instrument scope not supported
	#[serde(rename = "11")]
	RiskInstrumentScopeNotSupported,
	/// Risk limit not approved for party
	#[serde(rename = "12")]
	RiskLimitNotApprovedForParty,
	/// Risk limit already defined for party
	#[serde(rename = "13")]
	RiskLimitAlreadyDefinedForParty,
	/// Instrument not approved for party
	#[serde(rename = "14")]
	InstrumentNotApprovedForParty,
	/// Not authorized
	#[serde(rename = "98")]
	NotAuthorized,
	/// Other
	#[serde(rename = "99")]
	Other,
}

impl Default for RiskLimitResult {
	fn default() -> Self {
		RiskLimitResult::Successful
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
