
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TradingSessionStatus {
	/// MsgType = h
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'h', ' '>,
	/// ApplicationSequenceControl
	#[serde(flatten)]
	pub application_sequence_control: Option<super::super::application_sequence_control::ApplicationSequenceControl>,
	/// Provided for a response to a specific <a href="message_Trading_Session_Status_Request_g.html" target="main">Trading Session Status Request&nbsp;(g)</a> message (snapshot).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "335")]
	pub trad_ses_req_id: Option<String>,
	/// Market for which Trading Session applies
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1301")]
	pub market_id: Option<String>,
	/// Market Segment for which Trading Session applies
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1300")]
	pub market_segment_id: Option<String>,
	/// Identifier for Trading Session
	#[serde(rename = "336")]
	pub trading_session_id: TradingSessionID,
	/// TradingSessionSubID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "625")]
	pub trading_session_sub_id: Option<TradingSessionSubID>,
	/// Method of trading:
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "338")]
	pub trad_ses_method: Option<TradSesMethod>,
	/// Trading Session Mode
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "339")]
	pub trad_ses_mode: Option<TradSesMode>,
	/// Set to 'Y' if message is sent unsolicited as a result of a previous subscription request.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "325")]
	pub unsolicited_indicator: Option<UnsolicitedIndicator>,
	/// State of the trading session
	#[serde(rename = "340")]
	pub trad_ses_status: TradSesStatus,
	/// Identifies an event related to the trading status of a trading session
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1368")]
	pub trad_ses_event: Option<TradSesEvent>,
	/// Use with <a href="tag_340_TradSesStatus.html" target="bottom">TradSesStatus&nbsp;(340)</a> = "Request Rejected".
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "567")]
	pub trad_ses_status_rej_reason: Option<TradSesStatusRejReason>,
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
	#[serde(rename = "354")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_58_Text.html" target="bottom">Text&nbsp;(58)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
	/// Instrument
	#[serde(flatten)]
	pub instrument: Option<super::super::instrument::Instrument>,
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
pub enum UnsolicitedIndicator {
	/// Message is being sent as a result of a prior request
	#[serde(rename = "N")]
	MessageIsBeingSentAsAResultOfAPriorRequest,
	/// Message is being sent unsolicited
	#[serde(rename = "Y")]
	MessageIsBeingSentUnsolicited,
}

impl Default for UnsolicitedIndicator {
	fn default() -> Self {
		UnsolicitedIndicator::MessageIsBeingSentAsAResultOfAPriorRequest
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TradSesStatus {
	/// Unknown
	#[serde(rename = "0")]
	Unknown,
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
	/// Request Rejected
	#[serde(rename = "6")]
	RequestRejected,
}

impl Default for TradSesStatus {
	fn default() -> Self {
		TradSesStatus::Unknown
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TradSesEvent {
	/// Trading resumes (after Halt)
	#[serde(rename = "0")]
	TradingResumes,
	/// Change of Trading Session
	#[serde(rename = "1")]
	ChangeOfTradingSession,
	/// Change of Trading Subsession
	#[serde(rename = "2")]
	ChangeOfTradingSubsession,
	/// Change of Trading Status
	#[serde(rename = "3")]
	ChangeOfTradingStatus,
}

impl Default for TradSesEvent {
	fn default() -> Self {
		TradSesEvent::TradingResumes
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TradSesStatusRejReason {
	/// Unknown or invalid TradingSessionID
	#[serde(rename = "1")]
	UnknownOrInvalidTradingSessionId,
	/// Other
	#[serde(rename = "99")]
	Other,
}

impl Default for TradSesStatusRejReason {
	fn default() -> Self {
		TradSesStatusRejReason::UnknownOrInvalidTradingSessionId
	}
}
