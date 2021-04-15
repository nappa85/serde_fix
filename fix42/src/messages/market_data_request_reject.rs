
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MarketDataRequestReject {
	/// MsgType = Y
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'Y'>,
	/// Must refer to the <a href="tag_262_MDReqID.html" target="bottom">MDReqID&nbsp;(262)</a> of the request.
	#[serde(rename = "262")]
	pub md_req_id: String,
	/// MDReqRejReason
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "281")]
	pub md_req_rej_reason: Option<MDReqRejReason>,
	/// Text
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
pub enum MDReqRejReason {
	/// Unknown symbol
	#[serde(rename = "0")]
	UnknownSymbol,
	/// Duplicate <a href="tag_262_MDReqID.html" target="bottom">MDReqID&nbsp;(262)</a>
	#[serde(rename = "1")]
	DuplicateAHrefTag262MdReqIdHtmlTargetBottomMdReqIdNbspA,
	/// Insufficient Bandwidth
	#[serde(rename = "2")]
	InsufficientBandwidth,
	/// Insufficient Permissions
	#[serde(rename = "3")]
	InsufficientPermissions,
	/// Unsupported <a href="tag_263_SubscriptionRequestType.html" target="bottom">SubscriptionRequestType&nbsp;(263)</a>
	#[serde(rename = "4")]
	UnsupportedAHrefTag263SubscriptionRequestTypeHtmlTargetBottomSubscriptionRequestTypeNbspA,
	/// Unsupported <a href="tag_264_MarketDepth.html" target="bottom">MarketDepth&nbsp;(264)</a>
	#[serde(rename = "5")]
	UnsupportedAHrefTag264MarketDepthHtmlTargetBottomMarketDepthNbspA,
	/// Unsupported <a href="tag_265_MDUpdateType.html" target="bottom">MDUpdateType&nbsp;(265)</a>
	#[serde(rename = "6")]
	UnsupportedAHrefTag265MdUpdateTypeHtmlTargetBottomMdUpdateTypeNbspA,
	/// Unsupported <a href="tag_266_AggregatedBook.html" target="bottom">AggregatedBook&nbsp;(266)</a>
	#[serde(rename = "7")]
	UnsupportedAHrefTag266AggregatedBookHtmlTargetBottomAggregatedBookNbspA,
	/// Unsupported <a href="tag_269_MDEntryType.html" target="bottom">MDEntryType&nbsp;(269)</a>
	#[serde(rename = "8")]
	UnsupportedAHrefTag269MdEntryTypeHtmlTargetBottomMdEntryTypeNbspA,
}

impl Default for MDReqRejReason {
	fn default() -> Self {
		MDReqRejReason::UnknownSymbol
	}
}
