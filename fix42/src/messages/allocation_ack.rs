
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AllocationAck {
	/// MsgType = P
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'P'>,
	/// Used for firm identification in third-party transactions (should not be a substitute for <a href="tag_115_OnBehalfOfCompID.html" target="bottom">OnBehalfOfCompID&nbsp;(115)</a> / <a href="tag_128_DeliverToCompID.html" target="bottom">DeliverToCompID&nbsp;(128)</a> ).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "109")]
	pub client_id: Option<String>,
	/// Used for firm identification in third-party transactions (should not be a substitute for <a href="tag_115_OnBehalfOfCompID.html" target="bottom">OnBehalfOfCompID&nbsp;(115)</a> / <a href="tag_128_DeliverToCompID.html" target="bottom">DeliverToCompID&nbsp;(128)</a> ).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "76")]
	pub exec_broker: Option<String>,
	/// AllocID
	#[serde(rename = "70")]
	pub alloc_id: String,
	/// TradeDate
	#[serde(rename = "75")]
	pub trade_date: fix_common::LocalMktDate,
	/// Date/Time <a href="message_Allocation_ACK_P.html" target="main">Allocation ACK&nbsp;(P)</a> generated
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<fix_common::UTCTimestamp>,
	/// AllocStatus
	#[serde(rename = "87")]
	pub alloc_status: AllocStatus,
	/// Required for <a href="tag_87_AllocStatus.html" target="bottom">AllocStatus&nbsp;(87)</a> = 1 (rejected)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "88")]
	pub alloc_rej_code: Option<AllocRejCode>,
	/// Can include explanation for <a href="tag_88_AllocRejCode.html" target="bottom">AllocRejCode&nbsp;(88)</a> = 7 (other)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if <a href="tag_355_EncodedText.html" target="bottom">EncodedText&nbsp;(355)</a> field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "354")]
	pub encoded_text_len: Option<i32>,
	/// Encoded (non-ASCII characters) representation of the <a href="tag_58_Text.html" target="bottom">Text&nbsp;(58)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "355")]
	pub encoded_text: Option<String>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum AllocStatus {
	/// accepted (successfully processed)
	#[serde(rename = "0")]
	Accepted,
	/// rejected
	#[serde(rename = "1")]
	Rejected,
	/// partial accept
	#[serde(rename = "2")]
	PartialAccept,
	/// received (received, not yet processed)
	#[serde(rename = "3")]
	Received,
}

impl Default for AllocStatus {
	fn default() -> Self {
		AllocStatus::Accepted
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum AllocRejCode {
	/// unknown account(s)
	#[serde(rename = "0")]
	UnknownAccount,
	/// incorrect quantity
	#[serde(rename = "1")]
	IncorrectQuantity,
	/// incorrect average price
	#[serde(rename = "2")]
	IncorrectAveragePrice,
	/// unknown executing broker mnemonic
	#[serde(rename = "3")]
	UnknownExecutingBrokerMnemonic,
	/// commission difference
	#[serde(rename = "4")]
	CommissionDifference,
	/// unknown <a href="tag_37_OrderID.html" target="bottom">OrderID&nbsp;(37)</a>
	#[serde(rename = "5")]
	UnknownAHrefTag37OrderIdHtmlTargetBottomOrderIdNbspA,
	/// unknown <a href="tag_66_ListID.html" target="bottom">ListID&nbsp;(66)</a>
	#[serde(rename = "6")]
	UnknownAHrefTag66ListIdHtmlTargetBottomListIdNbspA,
	/// other
	#[serde(rename = "7")]
	Other,
}

impl Default for AllocRejCode {
	fn default() -> Self {
		AllocRejCode::UnknownAccount
	}
}
