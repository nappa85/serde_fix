
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Position {
	/// MsgType = DM
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// <p>The identifier of the <a href="message_Position_Transfer_Instruction_DL.html" target="main">PositionTransfer Instruction&nbsp;(DL)</a> this message is responding to.
	/// </p>
	#[serde(rename = "2436")]
	pub transfer_instruction_id: String,
	/// <p>Optional when responding to a "new" transfer. When responding to a <a href="message_Position_Transfer_Instruction_Ack_DM.html" target="main">Position Transfer Instruction&nbsp;(DM)</a> accepting, declining, or cancelling a transfer already initiated, this field can echo the <a href="tag_2437_TransferID.html" target="bottom">TransferID&nbsp;(2437)</a> sent.
	/// </p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2437")]
	pub transfer_id: Option<String>,
	/// TransferTransType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2439")]
	pub transfer_trans_type: Option<TransferTransType>,
	/// TransferType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2440")]
	pub transfer_type: Option<TransferType>,
	/// TransferStatus
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2442")]
	pub transfer_status: Option<TransferStatus>,
	/// <p>Conditionally required when <a href="tag_2442_TransferStatus.html" target="bottom">TransferStatus&nbsp;(2442)</a> = 1(Rejected by intermediary).
	/// </p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2443")]
	pub transfer_reject_reason: Option<TransferRejectReason>,
	/// TransferScope
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2441")]
	pub transfer_scope: Option<TransferScope>,
	/// <p>Specifies the source of the position transfer, e.g. the transferor</p>
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// <p>Specifies the target of the position transfer, e.g. the transferor</p>
	#[serde(flatten)]
	pub target_parties: Option<super::super::target_parties::TargetParties>,
	/// TransactTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<fix_common::UTCTimestamp>,
	/// RejectText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1328")]
	pub reject_text: Option<String>,
	/// Must be set if <a href="tag_1665_EncodedRejectText.html" target="bottom">EncodedRejectTextLen&nbsp;(1665)</a> field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1664")]
	pub encoded_reject_text_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the <a href="tag_1328_RejectText.html" target="bottom">RejectedText&nbsp;(1328)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1665")]
	pub encoded_reject_text: Option<String>,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// <p>Must be set if <a href="tag_355_EncodedText.html" target="bottom">EncodedText&nbsp;(355)</a> field is specified and must immediately precede it.
	/// </p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "354")]
	pub encoded_text_len: Option<usize>,
	/// <p>Encoded (non-ASCII characters) representation of the <a href="tag_58_Text.html" target="bottom">Text&nbsp;(58)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	/// </p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "355")]
	pub encoded_text: Option<String>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum TransferTransType {
	/// New
	#[serde(rename = "0")]
	New,
	/// Replace
	#[serde(rename = "1")]
	Replace,
	/// Cancel
	#[serde(rename = "2")]
	Cancel,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum TransferType {
	/// Request transfer
	#[serde(rename = "0")]
	RequestTransfer,
	/// Accept transfer
	#[serde(rename = "1")]
	AcceptTransfer,
	/// Decline transfer
	#[serde(rename = "2")]
	DeclineTransfer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum TransferStatus {
	/// Received
	#[serde(rename = "0")]
	Received,
	/// Rejected by intermediary
	#[serde(rename = "1")]
	RejectedByIntermediary,
	/// Accept pending
	#[serde(rename = "2")]
	AcceptPending,
	/// Accepted
	#[serde(rename = "3")]
	Accepted,
	/// Declined
	#[serde(rename = "4")]
	Declined,
	/// Cancelled
	#[serde(rename = "5")]
	Cancelled,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum TransferRejectReason {
	/// Success
	#[serde(rename = "0")]
	Success,
	/// Invalid party information
	#[serde(rename = "1")]
	InvalidPartyInformation,
	/// Unknown instrument
	#[serde(rename = "2")]
	UnknownInstrument,
	/// Not authorized to submit transfers
	#[serde(rename = "3")]
	NotAuthorizedToSubmitTransfers,
	/// Unknown position
	#[serde(rename = "4")]
	UnknownPosition,
	/// Other
	#[serde(rename = "99")]
	Other,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum TransferScope {
	/// Inter-firm transfer
	#[serde(rename = "0")]
	InterFirmTransfer,
	/// Intra-firm transfer
	#[serde(rename = "1")]
	IntraFirmTransfer,
	/// Clearing Member Trade Assignment (CMTA)
	#[serde(rename = "2")]
	ClearingMemberTradeAssignment,
}
