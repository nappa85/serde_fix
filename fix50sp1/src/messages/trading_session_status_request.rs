
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TradingSessionStatusRequest {
	/// MsgType = g
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'g', ' '>,
	/// Must be unique, or the ID of previous <a href="message_Trading_Session_Status_Request_g.html" target="main">Trading Session Status Request&nbsp;(g)</a> to disable if <a href="tag_263_SubscriptionRequestType.html" target="bottom">SubscriptionRequestType&nbsp;(263)</a> = Disable previous Snapshot + Updates Request (2).
	#[serde(rename = "335")]
	pub trad_ses_req_id: String,
	/// Market for which Trading Session applies
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1301")]
	pub market_id: Option<String>,
	/// Market Segment for which Trading Session applies
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1300")]
	pub market_segment_id: Option<String>,
	/// Trading Session for which status is being requested
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "336")]
	pub trading_session_id: Option<TradingSessionID>,
	/// TradingSessionSubID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "625")]
	pub trading_session_sub_id: Option<TradingSessionSubID>,
	/// Method of trading
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "338")]
	pub trad_ses_method: Option<TradSesMethod>,
	/// Trading Session Mode
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "339")]
	pub trad_ses_mode: Option<TradSesMode>,
	/// SubscriptionRequestType
	#[serde(rename = "263")]
	pub subscription_request_type: SubscriptionRequestType,
	/// SecurityExchange
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "207")]
	pub security_exchange: Option<String>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TradingSessionID {
	/// Day
	#[serde(rename = "1")]
	Day,
	/// HalfDay
	#[serde(rename = "2")]
	HalfDay,
	/// Morning
	#[serde(rename = "3")]
	Morning,
	/// Afternoon
	#[serde(rename = "4")]
	Afternoon,
	/// Evening
	#[serde(rename = "5")]
	Evening,
	/// After-hours
	#[serde(rename = "6")]
	AfterHours,
}

impl Default for TradingSessionID {
	fn default() -> Self {
		TradingSessionID::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TradingSessionSubID {
	/// Pre-Trading
	#[serde(rename = "1")]
	PreTrading,
	/// Opening or opening auction
	#[serde(rename = "2")]
	OpeningOrOpeningAuction,
	/// (Continuous) Trading
	#[serde(rename = "3")]
	Trading,
	/// Closing or closing auction
	#[serde(rename = "4")]
	ClosingOrClosingAuction,
	/// Post-Trading
	#[serde(rename = "5")]
	PostTrading,
	/// Intraday Auction
	#[serde(rename = "6")]
	IntradayAuction,
	/// Quiescent
	#[serde(rename = "7")]
	Quiescent,
}

impl Default for TradingSessionSubID {
	fn default() -> Self {
		TradingSessionSubID::PreTrading
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TradSesMethod {
	/// Electronic
	#[serde(rename = "1")]
	Electronic,
	/// Open Outcry
	#[serde(rename = "2")]
	OpenOutcry,
	/// Two Party
	#[serde(rename = "3")]
	TwoParty,
}

impl Default for TradSesMethod {
	fn default() -> Self {
		TradSesMethod::Electronic
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TradSesMode {
	/// Testing
	#[serde(rename = "1")]
	Testing,
	/// Simulated
	#[serde(rename = "2")]
	Simulated,
	/// Production
	#[serde(rename = "3")]
	Production,
}

impl Default for TradSesMode {
	fn default() -> Self {
		TradSesMode::Testing
	}
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
