
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MarketDataRequestReject {
	/// MsgType = Y
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'Y', ' '>,
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
	/// Unsupported <a href="tag_336_TradingSessionID.html" target="bottom">TradingSessionID&nbsp;(336)</a>
	#[serde(rename = "9")]
	UnsupportedAHrefTag336TradingSessionIdHtmlTargetBottomTradingSessionIdNbspA,
	/// Unsupported <a href="tag_546_Scope.html" target="bottom">Scope&nbsp;(546)</a>
	#[serde(rename = "A")]
	UnsupportedAHrefTag546ScopeHtmlTargetBottomScopeNbspA,
	/// Unsupported <a href="tag_286_OpenCloseSettlFlag.html" target="bottom">OpenCloseSettleFlag&nbsp;(286)</a>
	#[serde(rename = "B")]
	UnsupportedAHrefTag286OpenCloseSettlFlagHtmlTargetBottomOpenCloseSettleFlagNbspA,
	/// Unsupported <a href="tag_547_MDImplicitDelete.html" target="bottom">MDImplicitDelete&nbsp;(547)</a>
	#[serde(rename = "C")]
	UnsupportedAHrefTag547MdImplicitDeleteHtmlTargetBottomMdImplicitDeleteNbspA,
}

impl Default for MDReqRejReason {
	fn default() -> Self {
		MDReqRejReason::UnknownSymbol
	}
}
