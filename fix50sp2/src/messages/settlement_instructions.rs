
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SettlementInstructions {
	/// MsgType = T
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// Unique identifier for this message
	#[serde(rename = "777")]
	pub settl_inst_msg_id: String,
	/// Only used when this message is used to respond to a <a href="message_Settlement_Instruction_Request_AV.html" target="main">Settlement Instruction Request&nbsp;(AV)</a> (to which this ID refers)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "791")]
	pub settl_inst_req_id: Option<String>,
	/// 1=Standing Instructions, 2=Specific Allocation <a href="tag_1_Account.html" target="bottom">Account&nbsp;(1)</a> Overriding, 3=Specific Allocation <a href="tag_1_Account.html" target="bottom">Account&nbsp;(1)</a> Standing , 4=Specific Order, 5=Reject SSI request
	#[serde(rename = "160")]
	pub settl_inst_mode: SettlInstMode,
	/// Required for <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> = 5. Used to provide reason for rejecting a <a href="message_Settlement_Instruction_Request_AV.html" target="main">Settlement Instruction Request&nbsp;(AV)</a> message.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "792")]
	pub settl_inst_req_rej_code: Option<SettlInstReqRejCode>,
	/// Can be used to provide any additional rejection text where rejecting a <a href="message_Settlement_Instruction_Request_AV.html" target="main">Settlement Instruction Request&nbsp;(AV)</a> message.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// EncodedTextLen
	#[serde(rename = "354")]
	/// EncodedText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
	/// Required for <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode(160)&nbsp;(160)</a> = 4 and when referring to orders that where electronically submitted over FIX or otherwise assigned a ClOrdID.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "11")]
	pub cl_ord_id: Option<String>,
	/// Date/time this message was generated
	#[serde(rename = "60")]
	pub transact_time: fix_common::UTCTimestamp,
	/// Required except where <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> is 5=Reject SSI request
	#[serde(flatten)]
	pub settl_inst_grp: Option<super::super::settl_inst_grp::SettlInstGrp>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SettlInstMode {
	/// Default (Replaced)
	#[serde(rename = "0")]
	Default,
	/// Standing Instructions Provided
	#[serde(rename = "1")]
	StandingInstructionsProvided,
	/// Specific Allocation Account Overriding (Replaced)
	#[serde(rename = "2")]
	SpecificAllocationAccountOverriding,
	/// Specific Allocation Account Standing (Replaced)
	#[serde(rename = "3")]
	SpecificAllocationAccountStanding,
	/// Specific Order for a single account (for CIV)
	#[serde(rename = "4")]
	SpecificOrderForASingleAccount,
	/// Request reject
	#[serde(rename = "5")]
	RequestReject,
}

impl Default for SettlInstMode {
	fn default() -> Self {
		SettlInstMode::Default
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SettlInstReqRejCode {
	/// Unable to process request
	#[serde(rename = "0")]
	UnableToProcessRequest,
	/// Unknown account
	#[serde(rename = "1")]
	UnknownAccount,
	/// No matching settlement instructions found
	#[serde(rename = "2")]
	NoMatchingSettlementInstructionsFound,
	/// Other
	#[serde(rename = "99")]
	Other,
}

impl Default for SettlInstReqRejCode {
	fn default() -> Self {
		SettlInstReqRejCode::UnableToProcessRequest
	}
}
