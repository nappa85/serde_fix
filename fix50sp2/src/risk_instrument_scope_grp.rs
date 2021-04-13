
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RiskInstrumentScopeGrp {
	/// NoRiskInstrumentScopes
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1534")]
	pub risk_instrument_scopes: Option<fix_common::RepeatingValues<RiskInstrumentScope>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RiskInstrumentScope {
	/// <p>Required when <a href="tag_1534_NoRiskInstrumentScopes.html" target="bottom">NoRiskInstrumentScopes (1534)&nbsp;(1534)</a> &gt; 0
	/// </p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1535")]
	pub instrument_scope_operator: Option<InstrumentScopeOperator>,
	/// RiskInstrumentMultiplier
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1558")]
	pub risk_instrument_multiplier: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum InstrumentScopeOperator {
	/// Include
	#[serde(rename = "1")]
	Include,
	/// Exclude
	#[serde(rename = "2")]
	Exclude,
}

impl Default for InstrumentScopeOperator {
	fn default() -> Self {
		InstrumentScopeOperator::Include
	}
}
