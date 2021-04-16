
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TradeCaptureReportRequestAck {
	/// MsgType = AQ
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'A', 'Q'>,
	/// Identifier for the trade request
	#[serde(rename = "568")]
	pub trade_request_id: String,
	/// TradeID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1003")]
	pub trade_id: Option<String>,
	/// SecondaryTradeID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1040")]
	pub secondary_trade_id: Option<String>,
	/// FirmTradeID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1041")]
	pub firm_trade_id: Option<String>,
	/// SecondaryFirmTradeID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1042")]
	pub secondary_firm_trade_id: Option<String>,
	/// TradeRequestType
	#[serde(rename = "569")]
	pub trade_request_type: TradeRequestType,
	/// Used to subscribe / unsubscribe for trade capture reports If the field is absent, the value 0 will be the default
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "263")]
	pub subscription_request_type: Option<SubscriptionRequestType>,
	/// Number of trade reports returned
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "748")]
	pub tot_num_trade_reports: Option<i32>,
	/// Result of Trade Request
	#[serde(rename = "749")]
	pub trade_request_result: TradeRequestResult,
	/// Status of Trade Request
	#[serde(rename = "750")]
	pub trade_request_status: TradeRequestStatus,
	/// Insert here the set of "Instrument" fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub instrument: super::super::instrument::Instrument,
	/// Number of underlyings
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "711")]
	pub underlyings: Option<fix_common::RepeatingValues<Underlying>>,
	/// Number of legs Identifies a Multi-leg Execution if present and non-zero.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "555")]
	pub legs: Option<fix_common::RepeatingValues<Leg>>,
	/// Specify type of multileg reporting to be returned.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "442")]
	pub multi_leg_reporting_type: Option<MultiLegReportingType>,
	/// Ability to specify whether the response to the request should be delivered inband or via pre-arranged out-of-band transport.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "725")]
	pub response_transport_type: Option<ResponseTransportType>,
	/// URI destination name. Used if <a href="tag_725_ResponseTransportType.html" target="bottom">ResponseTransportType&nbsp;(725)</a> is out-of-band.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "726")]
	pub response_destination: Option<String>,
	/// May be used by the executing market to record any execution Details that are particular to that market
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if <a href="tag_355_EncodedText.html" target="bottom">EncodedText&nbsp;(355)</a> field is specified and must immediately precede it.
	#[serde(rename = "354")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_58_Text.html" target="bottom">Text&nbsp;(58)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
	/// Used to identify the event or source which gave rise to a message
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1011")]
	pub message_event_source: Option<String>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Underlying {
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Leg {
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TradeRequestType {
	/// All Trades
	#[serde(rename = "0")]
	AllTrades,
	/// Matched trades matching criteria provided on request (Parties, ExecID, TradeID, OrderID, Instrument, InputSource, etc.)
	#[serde(rename = "1")]
	MatchedTradesMatchingCriteriaProvidedOnRequest,
	/// Unmatched trades that match criteria
	#[serde(rename = "2")]
	UnmatchedTradesThatMatchCriteria,
	/// Unreported trades that match criteria
	#[serde(rename = "3")]
	UnreportedTradesThatMatchCriteria,
	/// Advisories that match criteria
	#[serde(rename = "4")]
	AdvisoriesThatMatchCriteria,
}

impl Default for TradeRequestType {
	fn default() -> Self {
		TradeRequestType::AllTrades
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TradeRequestResult {
	/// Successful (default)
	#[serde(rename = "0")]
	Successful,
	/// Invalid or unknown instrument
	#[serde(rename = "1")]
	InvalidOrUnknownInstrument,
	/// Invalid type of trade requested
	#[serde(rename = "2")]
	InvalidTypeOfTradeRequested,
	/// Invalid parties
	#[serde(rename = "3")]
	InvalidParties,
	/// Invalid transport type requested
	#[serde(rename = "4")]
	InvalidTransportTypeRequested,
	/// Invalid destination requested
	#[serde(rename = "5")]
	InvalidDestinationRequested,
	/// TradeRequestType not supported
	#[serde(rename = "8")]
	TradeRequestTypeNotSupported,
	/// Unauthorized for <a href="message_Trade_Capture_Report_Request_AD.html" target="main">Trade Capture Report Request&nbsp;(AD)</a>
	#[serde(rename = "9")]
	UnauthorizedForAHrefMessageTradeCaptureReportRequestAdHtmlTargetMainTradeCaptureReportRequestNbspA,
	/// Other
	#[serde(rename = "99")]
	Other,
}

impl Default for TradeRequestResult {
	fn default() -> Self {
		TradeRequestResult::Successful
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TradeRequestStatus {
	/// Accepted
	#[serde(rename = "0")]
	Accepted,
	/// Completed
	#[serde(rename = "1")]
	Completed,
	/// Rejected
	#[serde(rename = "2")]
	Rejected,
}

impl Default for TradeRequestStatus {
	fn default() -> Self {
		TradeRequestStatus::Accepted
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MultiLegReportingType {
	/// Single Security (default if not specified)
	#[serde(rename = "1")]
	SingleSecurity,
	/// Individual leg of a multi-leg security
	#[serde(rename = "2")]
	IndividualLegOfAMultiLegSecurity,
	/// Multi-leg security
	#[serde(rename = "3")]
	MultiLegSecurity,
}

impl Default for MultiLegReportingType {
	fn default() -> Self {
		MultiLegReportingType::SingleSecurity
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ResponseTransportType {
	/// Inband - transport the request was sent over (default)
	#[serde(rename = "0")]
	InbandTransportTheRequestWasSentOver,
	/// Out of Band - pre-arranged out-of-band delivery mechanizm (i.e. FTP, HTTP, NDM, etc.) between counterparties. Details specified
	/// via ResponseDestination (726).
	#[serde(rename = "1")]
	OutOfBandPreArrangedOutOfBandDeliveryMechanizmBetweenCounterpartiesDetailsSpecifiedViaResponseDestination,
}

impl Default for ResponseTransportType {
	fn default() -> Self {
		ResponseTransportType::InbandTransportTheRequestWasSentOver
	}
}
