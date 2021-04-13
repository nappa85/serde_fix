
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DisclosureInstructionGrp {
	/// NoDisclosureInstructions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1812")]
	pub disclosure_instructions: Option<fix_common::RepeatingValues<DisclosureInstruction>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DisclosureInstruction {
	/// Required when NoDisclosureInstructions (1812) &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1813")]
	pub disclosure_type: Option<DisclosureType>,
	/// DisclosureInstruction
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1814")]
	pub disclosure_instruction_item: Option<DisclosureInstructionItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DisclosureType {
	/// Volume
	#[serde(rename = "1")]
	Volume,
	/// Price
	#[serde(rename = "2")]
	Price,
	/// Side
	#[serde(rename = "3")]
	Side,
	/// AON
	#[serde(rename = "4")]
	Aon,
	/// General
	#[serde(rename = "5")]
	General,
	/// Clearing account
	#[serde(rename = "6")]
	ClearingAccount,
	/// CMTA account
	#[serde(rename = "7")]
	CmtaAccount,
}

impl Default for DisclosureType {
	fn default() -> Self {
		DisclosureType::Volume
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DisclosureInstructionItem {
	/// No
	#[serde(rename = "0")]
	No,
	/// Yes
	#[serde(rename = "1")]
	Yes,
	/// Use default setting
	#[serde(rename = "2")]
	UseDefaultSetting,
}

impl Default for DisclosureInstructionItem {
	fn default() -> Self {
		DisclosureInstructionItem::No
	}
}
