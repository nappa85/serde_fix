
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfirmationAck {
	/// MsgType = AU
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'A', 'U'>,
	/// ConfirmID
	#[serde(rename = "664")]
	pub confirm_id: String,
	/// TradeDate
	#[serde(rename = "75")]
	pub trade_date: fix_common::LocalMktDate,
	/// Date/Time <a href="message_Allocation_Instruction_Ack_P.html" target="main">Allocation Instruction Ack&nbsp;(P)</a> generated
	#[serde(rename = "60")]
	pub transact_time: fix_common::UTCTimestamp,
	/// AffirmStatus
	#[serde(rename = "940")]
	pub affirm_status: AffirmStatus,
	/// Required for <a href="tag_940_AffirmStatus.html" target="bottom">AffirmStatus&nbsp;(940)</a> = 1 (rejected)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "774")]
	pub confirm_rej_reason: Option<ConfirmRejReason>,
	/// Denotes whether the financial details provided on the <a href="message_Confirmation_AK.html" target="main">Confirmation&nbsp;(AK)</a> were successfully matched.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "573")]
	pub match_status: Option<MatchStatus>,
	/// Can include explanation for <a href="tag_88_AllocRejCode.html" target="bottom">AllocRejCode&nbsp;(88)</a> = 7 (other)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if <a href="tag_355_EncodedText.html" target="bottom">EncodedText&nbsp;(355)</a> field is specified and must immediately precede it.
	#[serde(rename = "354")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_58_Text.html" target="bottom">Text&nbsp;(58)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum AffirmStatus {
	/// Received
	#[serde(rename = "1")]
	Received,
	/// Confirm rejected, i.e. not affirmed
	#[serde(rename = "2")]
	ConfirmRejectedIENotAffirmed,
	/// Affirmed
	#[serde(rename = "3")]
	Affirmed,
}

impl Default for AffirmStatus {
	fn default() -> Self {
		AffirmStatus::Received
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ConfirmRejReason {
	/// Mismatched account
	#[serde(rename = "1")]
	MismatchedAccount,
	/// Missing settlement instructions
	#[serde(rename = "2")]
	MissingSettlementInstructions,
	/// Other
	#[serde(rename = "99")]
	Other,
}

impl Default for ConfirmRejReason {
	fn default() -> Self {
		ConfirmRejReason::MismatchedAccount
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MatchStatus {
	/// compared, matched or affirmed
	#[serde(rename = "0")]
	ComparedMatchedOrAffirmed,
	/// uncompared, unmatched, or unaffirmed
	#[serde(rename = "1")]
	UncomparedUnmatchedOrUnaffirmed,
	/// Advisory or alert
	#[serde(rename = "2")]
	AdvisoryOrAlert,
}

impl Default for MatchStatus {
	fn default() -> Self {
		MatchStatus::ComparedMatchedOrAffirmed
	}
}
