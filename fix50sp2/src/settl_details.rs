
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SettlDetails {
	/// Number of settlement parties
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1158")]
	pub settl_details: Option<fix_common::RepeatingValues<SettlDetail>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SettlDetail {
	/// Indicates the Source of the Settlement Instructions.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1164")]
	pub settl_oblig_source: Option<SettlObligSource>,
	/// StandInstDbType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "169")]
	pub stand_inst_db_type: Option<StandInstDbType>,
	/// StandInstDbName
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "170")]
	pub stand_inst_db_name: Option<String>,
	/// StandInstDbID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "171")]
	pub stand_inst_db_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SettlObligSource {
	/// Instructions of Broker
	#[serde(rename = "1")]
	InstructionsOfBroker,
	/// Instructions for Institution
	#[serde(rename = "2")]
	InstructionsForInstitution,
	/// Investor
	#[serde(rename = "3")]
	Investor,
	/// Buyer's settlement instructions
	#[serde(rename = "4")]
	BuyerSSettlementInstructions,
	/// Seller's settlement instructions
	#[serde(rename = "5")]
	SellerSSettlementInstructions,
}

impl Default for SettlObligSource {
	fn default() -> Self {
		SettlObligSource::InstructionsOfBroker
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum StandInstDbType {
	/// Other
	#[serde(rename = "0")]
	Other,
	/// DTC SID
	#[serde(rename = "1")]
	DtcSid,
	/// Thomson ALERT
	#[serde(rename = "2")]
	ThomsonAlert,
	/// A Global Custodian ( <a href="tag_170_StandInstDbName.html" target="bottom">StandInstDBName&nbsp;(170)</a> must be provided)
	#[serde(rename = "3")]
	AGlobalCustodianAMustBeProvided,
	/// AccountNet
	#[serde(rename = "4")]
	AccountNet,
}

impl Default for StandInstDbType {
	fn default() -> Self {
		StandInstDbType::Other
	}
}
