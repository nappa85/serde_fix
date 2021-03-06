
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SettlInstructionsData {
	/// Required if <a href="tag_780_AllocSettlInstType.html" target="bottom">AllocSettlInstType&nbsp;(780)</a> = 1 or 2
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "172")]
	pub settl_delivery_type: Option<SettlDeliveryType>,
	/// Required if <a href="tag_780_AllocSettlInstType.html" target="bottom">AllocSettlInstType&nbsp;(780)</a> = 3 (should not be populated otherwise)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "169")]
	pub stand_inst_db_type: Option<StandInstDbType>,
	/// Required if <a href="tag_780_AllocSettlInstType.html" target="bottom">AllocSettlInstType&nbsp;(780)</a> = 3 (should not be populated otherwise)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "170")]
	pub stand_inst_db_name: Option<String>,
	/// Identifier used within the StandInstDbType Required if <a href="tag_780_AllocSettlInstType.html" target="bottom">AllocSettlInstType&nbsp;(780)</a> = 3 (should not be populated otherwise)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "171")]
	pub stand_inst_db_id: Option<String>,
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SettlDeliveryType {
	/// "Versus. Payment": Deliver (if Sell) or Receive (if Buy) vs. (Against) Payment
	#[serde(rename = "0")]
	VersusPaymentDeliverOrReceiveVsPayment,
	/// "Free": Deliver (if Sell) or Receive (if Buy) Free"
	#[serde(rename = "1")]
	FreeDeliverOrReceiveFree,
	/// Tri-Party
	#[serde(rename = "2")]
	TriParty,
	/// Hold In Custody
	#[serde(rename = "3")]
	HoldInCustody,
}

impl Default for SettlDeliveryType {
	fn default() -> Self {
		SettlDeliveryType::VersusPaymentDeliverOrReceiveVsPayment
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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
