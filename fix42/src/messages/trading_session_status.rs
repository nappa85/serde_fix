
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TradingSessionStatus {
	/// MsgType = h
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'h'>,
	/// Provided for a response to a specific <a href="message_Trading_Session_Status_Request_g.html" target="main">Trading Session Status Request&nbsp;(g)</a> message (snapshot).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "335")]
	pub trad_ses_req_id: Option<String>,
	/// Identifier for Trading Session
	#[serde(rename = "336")]
	pub trading_session_id: String,
	/// Method of trading:
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "338")]
	pub trad_ses_method: Option<TradSesMethod>,
	/// Trading Session Mode
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "339")]
	pub trad_ses_mode: Option<TradSesMode>,
	/// 'Y' if message is sent unsolicited as a result of a previous subscription request.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "325")]
	pub unsolicited_indicator: Option<UnsolicitedIndicator>,
	/// State of the trading session
	#[serde(rename = "340")]
	pub trad_ses_status: TradSesStatus,
	/// Starting time of the trading session
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "341")]
	pub trad_ses_start_time: Option<fix_common::UTCTimestamp>,
	/// Time of the opening of the trading session
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "342")]
	pub trad_ses_open_time: Option<fix_common::UTCTimestamp>,
	/// Time of the pre-close of the trading session
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "343")]
	pub trad_ses_pre_close_time: Option<fix_common::UTCTimestamp>,
	/// Closing time of the trading session
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "344")]
	pub trad_ses_close_time: Option<fix_common::UTCTimestamp>,
	/// End time of the trading session
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "345")]
	pub trad_ses_end_time: Option<fix_common::UTCTimestamp>,
	/// TotalVolumeTraded
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "387")]
	pub total_volume_traded: Option<f64>,
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
	/// Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "355")]
	pub encoded_text: Option<String>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
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
pub enum UnsolicitedIndicator {
	/// Message is being sent unsolicited
	#[serde(rename = "Y")]
	MessageIsBeingSentUnsolicited,
	/// Message is being sent as a result of a prior request
	#[serde(rename = "N")]
	MessageIsBeingSentAsAResultOfAPriorRequest,
}

impl Default for UnsolicitedIndicator {
	fn default() -> Self {
		UnsolicitedIndicator::MessageIsBeingSentUnsolicited
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TradSesStatus {
	/// Halted
	#[serde(rename = "1")]
	Halted,
	/// Open
	#[serde(rename = "2")]
	Open,
	/// Closed
	#[serde(rename = "3")]
	Closed,
	/// Pre-Open
	#[serde(rename = "4")]
	PreOpen,
	/// Pre-Close
	#[serde(rename = "5")]
	PreClose,
}

impl Default for TradSesStatus {
	fn default() -> Self {
		TradSesStatus::Halted
	}
}
