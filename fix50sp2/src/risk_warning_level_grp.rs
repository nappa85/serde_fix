
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RiskWarningLevelGrp {
	/// NoRiskWarningLevels
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1559")]
	pub risk_warning_levels: Option<fix_common::RepeatingValues<RiskWarningLevel>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RiskWarningLevel {
	/// <p>Conditionally required when RiskWarningLevelAmount(1768) is not provided</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1560")]
	pub risk_warning_level_percent: Option<f32>,
	/// RiskWarningLevelName
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1561")]
	pub risk_warning_level_name: Option<String>,
	/// <p>Required if NoRiskWarningLevels(1559) &gt; 0.</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1769")]
	pub risk_warning_level_action: Option<RiskWarningLevelAction>,
	/// <p>Conditionally required when RiskWarningLevelPercent(1560) is not provided.</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1768")]
	pub risk_warning_level_amount: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RiskWarningLevelAction {
	/// Queue Inbound
	#[serde(rename = "0")]
	QueueInbound,
	/// Queue Outbound
	#[serde(rename = "1")]
	QueueOutbound,
	/// Reject
	#[serde(rename = "2")]
	Reject,
	/// Disconnect
	#[serde(rename = "3")]
	Disconnect,
	/// Warning
	#[serde(rename = "4")]
	Warning,
}

impl Default for RiskWarningLevelAction {
	fn default() -> Self {
		RiskWarningLevelAction::QueueInbound
	}
}
