
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SettlementObligationReport {
	/// MsgType = BQ
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// ApplicationSequenceControl
	#[serde(flatten)]
	pub application_sequence_control: Option<super::super::application_sequence_control::ApplicationSequenceControl>,
	/// ClearingBusinessDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "715")]
	pub clearing_business_date: Option<fix_common::LocalMktDate>,
	/// Settlement cycle in which the settlement obligation was generated
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1153")]
	pub settlement_cycle_no: Option<i32>,
	/// Unique identifier for this message
	#[serde(rename = "1160")]
	pub settl_oblig_msg_id: String,
	/// Used to identify the reporting mode of the settlement obligation which is either preliminary or final
	#[serde(rename = "1159")]
	pub settl_oblig_mode: SettlObligMode,
	/// Can be used to provide any additional rejection text where rejecting a Settlement Instruction Request message.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if <a href="tag_355_EncodedText.html" target="bottom">EncodedText&nbsp;(355)</a> field is specified and must immediately precede it.
	#[serde(rename = "354")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_58_Text.html" target="bottom">Text&nbsp;(58)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
	/// Time when the Settlemnt Obligation Report was created.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<fix_common::UTCTimestamp>,
	/// SettlObligationInstructions
	#[serde(flatten)]
	pub settl_obligation_instructions: super::super::settl_obligation_instructions::SettlObligationInstructions,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SettlObligMode {
	/// Preliminary
	#[serde(rename = "1")]
	Preliminary,
	/// Final
	#[serde(rename = "2")]
	Final,
}

impl Default for SettlObligMode {
	fn default() -> Self {
		SettlObligMode::Preliminary
	}
}
