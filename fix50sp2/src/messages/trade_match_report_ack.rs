
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Trade {
	/// MsgType = DD
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// ApplicationSequenceControl
	#[serde(flatten)]
	pub application_sequence_control: Option<super::super::application_sequence_control::ApplicationSequenceControl>,
	/// Indicates the Match ID of the Trade Match Report being acknowledged. Identifier of the TradeMatchReport (35=DC) being acknowledged.
	#[serde(rename = "880")]
	pub trd_match_id: String,
	/// TradeMatchAckStatus
	#[serde(rename = "1896")]
	pub trade_match_ack_status: TradeMatchAckStatus,
	/// Conditionally required when TradeMatchAckStatus(1896) = Rejected(2).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1897")]
	pub trade_match_reject_reason: Option<TradeMatchRejectReason>,
	/// RejectText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1328")]
	pub reject_text: Option<String>,
	/// EncodedRejectTextLen
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1664")]
	pub encoded_reject_text_len: Option<usize>,
	/// EncodedRejectText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1665")]
	pub encoded_reject_text: Option<String>,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// EncodedTextLen
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "354")]
	pub encoded_text_len: Option<usize>,
	/// EncodedText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "355")]
	pub encoded_text: Option<String>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TradeMatchAckStatus {
	/// Received, not yet processed
	#[serde(rename = "0")]
	ReceivedNotYetProcessed,
	/// Accepted
	#[serde(rename = "1")]
	Accepted,
	/// Rejected
	#[serde(rename = "2")]
	Rejected,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TradeMatchRejectReason {
	/// Successful(default)
	#[serde(rename = "0")]
	Successful,
	/// Invalid party information
	#[serde(rename = "1")]
	InvalidPartyInformation,
	/// Unknown instrument
	#[serde(rename = "2")]
	UnknownInstrument,
	/// Not authorized to report trades
	#[serde(rename = "3")]
	NotAuthorizedToReportTrades,
	/// Invalid trade type
	#[serde(rename = "4")]
	InvalidTradeType,
	/// Other
	#[serde(rename = "99")]
	Other,
}
