
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InstrumentScopeGrp {
	/// NoInstrumentScopes
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1656")]
	pub instrument_scopes: Option<fix_common::RepeatingValues<InstrumentScope>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InstrumentScope {
	/// <p>Required when <a href="tag_1656_NoInstrumentScopes.html" target="bottom">NoInstrumentScopes (1656)&nbsp;(1656)</a> &gt; 0
	/// </p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1535")]
	pub instrument_scope_operator: Option<InstrumentScopeOperator>,
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
