
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DlvyInstGrp {
	/// NoDlvyInst
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "85")]
	pub dlvy_inst: Option<fix_common::RepeatingValues<DlvyIns>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DlvyIns {
	/// SettlInstSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "165")]
	pub settl_inst_source: Option<SettlInstSource>,
	/// DlvyInstType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "787")]
	pub dlvy_inst_type: Option<DlvyInstType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SettlInstSource {
	/// Broker's Instructions
	#[serde(rename = "1")]
	BrokerSInstructions,
	/// Institution's Instructions
	#[serde(rename = "2")]
	InstitutionSInstructions,
	/// Investor (e.g. CIV use)
	#[serde(rename = "3")]
	Investor,
}

impl Default for SettlInstSource {
	fn default() -> Self {
		SettlInstSource::BrokerSInstructions
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DlvyInstType {
	/// Cash
	#[serde(rename = "C")]
	Cash,
	/// Securities
	#[serde(rename = "S")]
	Securities,
}

impl Default for DlvyInstType {
	fn default() -> Self {
		DlvyInstType::Cash
	}
}
