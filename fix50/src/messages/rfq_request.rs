
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RFQRequest {
	/// MsgType = AH
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'A', 'H'>,
	/// RFQReqID
	#[serde(rename = "644")]
	pub rfq_req_id: String,
	/// Number of related symbols (instruments) in Request
	#[serde(rename = "146")]
	pub related_sym: fix_common::RepeatingValues<RelatedSy>,
	/// Used to subscribe for <a href="message_Quote_S.html" target="main">Quote&nbsp;(S)</a> Requests that are sent into a market
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "263")]
	pub subscription_request_type: Option<SubscriptionRequestType>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RelatedSy {
	/// Number of underlyings
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "711")]
	pub no_underlyings: Option<usize>,
	/// Number of legs Identifies a Multi-leg Execution if present and non-zero.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "555")]
	pub no_legs: Option<usize>,
	/// Useful for verifying security identification
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "140")]
	pub prev_close_px: Option<f64>,
	/// Indicates the type of <a href="message_Quote_Request_R.html" target="main">Quote Request&nbsp;(R)</a> (e.g. Manual vs. Automatic) being generated.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "303")]
	pub quote_request_type: Option<QuoteRequestType>,
	/// Type of quote being requested from counterparty or market (e.g. Indicative, Firm, or Restricted Tradeable)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "537")]
	pub quote_type: Option<QuoteType>,
	/// TradingSessionID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "336")]
	pub trading_session_id: Option<String>,
	/// TradingSessionSubID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "625")]
	pub trading_session_sub_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SubscriptionRequestType {
	/// Snapshot
	#[serde(rename = "0")]
	Snapshot,
	/// Snapshot + Updates (Subscribe)
	#[serde(rename = "1")]
	SnapshotUpdates,
	/// Disable previous Snapshot + Update Request (Unsubscribe)
	#[serde(rename = "2")]
	DisablePreviousSnapshotUpdateRequest,
}

impl Default for SubscriptionRequestType {
	fn default() -> Self {
		SubscriptionRequestType::Snapshot
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum QuoteRequestType {
	/// Manual
	#[serde(rename = "1")]
	Manual,
	/// Automatic
	#[serde(rename = "2")]
	Automatic,
}

impl Default for QuoteRequestType {
	fn default() -> Self {
		QuoteRequestType::Manual
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum QuoteType {
	/// Indicative
	#[serde(rename = "0")]
	Indicative,
	/// Tradeable
	#[serde(rename = "1")]
	Tradeable,
	/// Restricted Tradeable
	#[serde(rename = "2")]
	RestrictedTradeable,
	/// Counter (tradeable)
	#[serde(rename = "3")]
	Counter,
}

impl Default for QuoteType {
	fn default() -> Self {
		QuoteType::Indicative
	}
}
