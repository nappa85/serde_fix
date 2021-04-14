
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MarketDataRequestReject {
	/// MsgType = Y
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// Must refer to the <a href="tag_262_MDReqID.html" target="bottom">MDReqID&nbsp;(262)</a> of the request.
	#[serde(rename = "262")]
	pub md_req_id: String,
	/// MDReqRejReason
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "281")]
	pub md_req_rej_reason: Option<MDReqRejReason>,
	/// NoAltMDSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "816")]
	pub alt_md_source: Option<fix_common::RepeatingValues<AltMDSourc>>,
	/// Text
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

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AltMDSourc {
	/// Alternative Market Data Source
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "817")]
	pub alt_md_source_id: Option<String>,
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

impl Default for MDReqRejReason {
	fn default() -> Self {
		MDReqRejReason::UnknownSymbol
	}
}
