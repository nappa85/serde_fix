
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClrInstGrp {
	/// NoClearingInstructions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "576")]
	pub clearing_instructions: Option<crate::entities::RepeatingValues<ClearingInstruction>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClearingInstruction {
	/// Required if NoClearingInstructions &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "577")]
	pub clearing_instruction_item: Option<ClearingInstructionItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClearingInstructionItem {
	/// Process normally
	#[serde(rename = "0")]
	ProcessNormally,
	/// Exclude from all netting
	#[serde(rename = "1")]
	ExcludeFromAllNetting,
	/// Bilateral netting only
	#[serde(rename = "2")]
	BilateralNettingOnly,
	/// Ex clearing
	#[serde(rename = "3")]
	ExClearing,
	/// Special trade
	#[serde(rename = "4")]
	SpecialTrade,
	/// Multilateral netting
	#[serde(rename = "5")]
	MultilateralNetting,
	/// Clear against central counterparty
	#[serde(rename = "6")]
	ClearAgainstCentralCounterparty,
	/// Exclude from central counterparty
	#[serde(rename = "7")]
	ExcludeFromCentralCounterparty,
	/// Manual mode (pre-posting and/or pre-giveup)
	#[serde(rename = "8")]
	ManualMode,
	/// Automatic posting mode (trade posting to the position account number specified)
	#[serde(rename = "9")]
	AutomaticPostingMode,
	/// Automatic give-up mode (trade give-up to the give-up destination number specified)
	#[serde(rename = "10")]
	AutomaticGiveUpMode,
	/// Qualified Service Representative QSR
	#[serde(rename = "11")]
	QualifiedServiceRepresentativeQsr,
	/// Customer trade
	#[serde(rename = "12")]
	CustomerTrade,
	/// Self clearing
	#[serde(rename = "13")]
	SelfClearing,
	/// Buy-in
	#[serde(rename = "14")]
	BuyIn,
}
