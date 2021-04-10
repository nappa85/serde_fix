
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Market {
	/// MsgType = Y
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// Must refer to the <a href="tag_262_MDReqID.html" target="bottom">MDReqID&nbsp;(262)</a> of the request.
	#[serde(rename = "262")]
	pub md_req_id: String,
	/// Insert here the set of Parties (firm identification) fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// MDReqRejReason
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "281")]
	pub md_req_rej_reason: Option<MDReqRejReason>,
	/// MDRjctGrp
	#[serde(flatten)]
	pub md_rjct_grp: Option<super::super::md_rjct_grp::MDRjctGrp>,
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
pub enum MDReqRejReason {
	/// Unknown symbol
	#[serde(rename = "0")]
	UnknownSymbol,
	/// Duplicate MDReqID
	#[serde(rename = "1")]
	DuplicateMdReqId,
	/// Insufficient Bandwidth
	#[serde(rename = "2")]
	InsufficientBandwidth,
	/// Insufficient Permissions
	#[serde(rename = "3")]
	InsufficientPermissions,
	/// Unsupported SubscriptionRequestType
	#[serde(rename = "4")]
	UnsupportedSubscriptionRequestType,
	/// Unsupported MarketDepth
	#[serde(rename = "5")]
	UnsupportedMarketDepth,
	/// Unsupported MDUpdateType
	#[serde(rename = "6")]
	UnsupportedMdUpdateType,
	/// Unsupported AggregatedBook
	#[serde(rename = "7")]
	UnsupportedAggregatedBook,
	/// Unsupported MDEntryType
	#[serde(rename = "8")]
	UnsupportedMdEntryType,
	/// Unsupported TradingSessionID
	#[serde(rename = "9")]
	UnsupportedTradingSessionId,
	/// Unsupported Scope
	#[serde(rename = "A")]
	UnsupportedScope,
	/// Unsupported OpenCloseSettleFlag
	#[serde(rename = "B")]
	UnsupportedOpenCloseSettleFlag,
	/// Unsupported MDImplicitDelete
	#[serde(rename = "C")]
	UnsupportedMdImplicitDelete,
	/// Insufficient credit
	#[serde(rename = "D")]
	InsufficientCredit,
}
