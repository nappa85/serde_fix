
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Collateral {
	/// MsgType = DQ
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// Identifer of the <a href="message_Collateral_Report_BA.html" target="main">CollateralReport(35=BA)&nbsp;(BA)</a> being acknowledged
	#[serde(rename = "908")]
	pub coll_rpt_id: String,
	/// TransactTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<fix_common::UTCTimestamp>,
	/// CollRptStatus
	#[serde(rename = "2488")]
	pub coll_rpt_status: CollRptStatus,
	/// Conditionally required when <a href="tag_2488_CollRptStatus.html" target="bottom">CollRptStatus(2488)&nbsp;(2488)</a> = 2 (Rejected).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2487")]
	pub coll_rpt_reject_reason: Option<CollRptRejectReason>,
	/// Conditionally required when <a href="tag_2488_CollRptStatus.html" target="bottom">CollRptStatus(2488)&nbsp;(2488)</a> = 2 (Rejected).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1328")]
	pub reject_text: Option<String>,
	/// Must be set if <a href="tag_1665_EncodedRejectText.html" target="bottom">EncodedRejectTextLen(1665)&nbsp;(1665)</a> field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1664")]
	pub encoded_reject_text_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the <a href="tag_1328_RejectText.html" target="bottom">RejectedText(1328)&nbsp;(1328)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding(347)&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1665")]
	pub encoded_reject_text: Option<String>,
	/// Parties
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if <a href="tag_355_EncodedText.html" target="bottom">EncodedText&nbsp;(355)</a> field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "354")]
	pub encoded_text_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the <a href="tag_58_Text.html" target="bottom">Text&nbsp;(58)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "355")]
	pub encoded_text: Option<String>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CollRptStatus {
	/// Accepted (successfullly processed)
	#[serde(rename = "0")]
	Accepted,
	/// Received (not yet processed)
	#[serde(rename = "1")]
	Received,
	/// Rejected
	#[serde(rename = "2")]
	Rejected,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CollRptRejectReason {
	/// Unknown trade or transaction
	#[serde(rename = "0")]
	UnknownTradeOrTransaction,
	/// Unknown or invalid instrument
	#[serde(rename = "1")]
	UnknownOrInvalidInstrument,
	/// Unknown or invalid counterparty
	#[serde(rename = "2")]
	UnknownOrInvalidCounterparty,
	/// Unknown of invalid position
	#[serde(rename = "3")]
	UnknownOfInvalidPosition,
	/// Unacceptable or invalid type of collateral
	#[serde(rename = "4")]
	UnacceptableOrInvalidTypeOfCollateral,
	/// Other
	#[serde(rename = "99")]
	Other,
	/// Reserved
	#[serde(rename = "100")]
	Reserved,
}
