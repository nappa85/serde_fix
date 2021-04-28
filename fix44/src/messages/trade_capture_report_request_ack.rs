
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TradeCaptureReportRequestAck {
	/// MsgType = AQ
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'A', 'Q'>,
	/// Identifier for the trade request
	#[serde(rename = "568")]
	pub trade_request_id: String,
	/// TradeRequestType
	#[serde(rename = "569")]
	pub trade_request_type: TradeRequestType,
	/// Used to subscribe / unsubscribe for trade capture reports. If the field is absent, the value 0 will be the default.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "263")]
	pub subscription_request_type: Option<SubscriptionRequestType>,
	/// Number of trade reports returned.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "748")]
	pub total_num_trade_reports: Option<i32>,
	/// Result of Trade Request: 0 - Accepted, 1 - Rejected.
	#[serde(rename = "749")]
	pub trade_request_result: TradeRequestResult,
	/// Status of Trade Request.
	#[serde(rename = "750")]
	pub trade_request_status: TradeRequestStatus,
	/// Instrument
	#[serde(flatten)]
	pub instrument: super::super::instrument::Instrument,
	/// NoUnderlyings
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "711")]
	pub underlyings: Option<fix_common::RepeatingValues<super::super::underlying_instrument::UnderlyingInstrument>>,
	/// Number of legs. <a href="tag_555_NoLegs.html" target="bottom">NoLegs&nbsp;(555)</a> &gt; 0 identifies a Multi-leg Execution
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "555")]
	pub legs: Option<fix_common::RepeatingValues<super::super::instrument_leg::InstrumentLeg>>,
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
	/// May be used by the executing market to record any execution. Details that are particular to that market.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if <a href="tag_355_EncodedText.html" target="bottom">EncodedText&nbsp;(355)</a> field is specified and must immediately precede it.
	#[serde(rename = "354")]
	/// Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TradeRequestType {
	/// All trades
	#[serde(rename = "0")]
	AllTrades,
	/// Matched trades matching Criteria provided on request (parties, order id,instrument, input source, etc.)
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
	/// Invalid Transport Type requested
	#[serde(rename = "4")]
	InvalidTransportTypeRequested,
	/// Invalid Destination requested
	#[serde(rename = "5")]
	InvalidDestinationRequested,
	/// <a href="tag_569_TradeRequestType.html" target="bottom">TradeRequestType&nbsp;(569)</a> not supported
	#[serde(rename = "8")]
	TradeRequestTypeNotSupported,
	/// Unauthorized for <a href="message_Trade_Capture_Report_Request_AD.html" target="main">Trade Capture Report Request&nbsp;(AD)</a>
	#[serde(rename = "9")]
	UnauthorizedForTradeCaptureReportRequest,
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
	/// Inband: transport the request was sent over (Default)
	#[serde(rename = "0")]
	InbandTransportTheRequestWasSentOver,
	/// Out-of-Band: pre-arranged out of band delivery mechanism (i.e. FTP, HTTP, NDM, etc) between counterparties. Details specified
	/// via <a href="tag_726_ResponseDestination.html" target="bottom">ResponseDestination&nbsp;(726)</a> .
	#[serde(rename = "1")]
	OutOfBandPreArrangedOutOfBandDeliveryMechanismBetweenCounterpartiesDetailsSpecifiedViaResponseDestination,
}

impl Default for ResponseTransportType {
	fn default() -> Self {
		ResponseTransportType::InbandTransportTheRequestWasSentOver
	}
}
