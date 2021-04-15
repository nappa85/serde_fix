
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TradeCaptureReportRequest {
	/// MsgType = AD
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'A', 'D'>,
	/// Identifier for the trade request
	#[serde(rename = "568")]
	pub trade_request_id: String,
	/// TradeRequestType
	#[serde(rename = "569")]
	pub trade_request_type: TradeRequestType,
	/// Used to subscribe / unsubscribe for trade capture reports. If the field is absent, the value 0 will be the default (snapshot
	/// only - no subscription)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "263")]
	pub subscription_request_type: Option<SubscriptionRequestType>,
	/// To request a specific trade report
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "571")]
	pub trade_report_id: Option<String>,
	/// To request a specific trade report
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "818")]
	pub secondary_trade_report_id: Option<String>,
	/// ExecID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "17")]
	pub exec_id: Option<String>,
	/// To requst all trades of a specific execution type
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "150")]
	pub exec_type: Option<ExecType>,
	/// OrderID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "37")]
	pub order_id: Option<String>,
	/// ClOrdID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "11")]
	pub cl_ord_id: Option<String>,
	/// MatchStatus
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "573")]
	pub match_status: Option<MatchStatus>,
	/// To request all trades of a specific trade type
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "828")]
	pub trd_type: Option<TrdType>,
	/// To request all trades of a specific trade sub type
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "829")]
	pub trd_sub_type: Option<i32>,
	/// To request all trades for a specific transfer reason
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "830")]
	pub transfer_reason: Option<String>,
	/// To request all trades of a specific trade sub type
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "855")]
	pub secondary_trd_type: Option<SecondaryTrdType>,
	/// To request all trades of a specific trade link id
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "820")]
	pub trade_link_id: Option<String>,
	/// To request a trade matching a specific <a href="tag_880_TrdMatchID.html" target="bottom">TrdMatchID&nbsp;(880)</a>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "880")]
	pub trd_match_id: Option<String>,
	/// Used to specify the parties for the trades to be returned (clearing firm, execution broker, trader id, etc.): ExecutingBroker;
	/// ClearingFirm; ContraBroker; ContraClearingFirm; SettlementLocation - depository, CSD, or other settlement party; ExecutingTrader;
	/// InitiatingTrader; OrderOriginator.
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// Instrument
	#[serde(flatten)]
	pub instrument: Option<super::super::instrument::Instrument>,
	/// Instrument Extension
	#[serde(flatten)]
	pub instrument_extension: Option<super::super::instrument_extension::InstrumentExtension>,
	/// Financing Details
	#[serde(flatten)]
	pub financing_details: Option<super::super::financing_details::FinancingDetails>,
	/// Indicates number of repeating entries.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "711")]
	pub underlyings: Option<fix_common::RepeatingValues<Underlying>>,
	/// Indicates number of repeating entries.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "555")]
	pub legs: Option<fix_common::RepeatingValues<Leg>>,
	/// Number of date ranges provided (must be 1 or 2 if specified)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "580")]
	pub dates: Option<fix_common::RepeatingValues<Date>>,
	/// To request trades for a specific clearing business date.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "715")]
	pub clearing_business_date: Option<fix_common::LocalMktDate>,
	/// To request trades for a specific trading session.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "336")]
	pub trading_session_id: Option<String>,
	/// To request trades for a specific trading session.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "625")]
	pub trading_session_sub_id: Option<String>,
	/// To request trades within a specific time bracket.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "943")]
	pub time_bracket: Option<String>,
	/// To request trades for a specific side of a trade.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "54")]
	pub side: Option<Side>,
	/// Used to indicate if trades are to be returned for the individual legs of a multileg instrument or for the overall instrument.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "442")]
	pub multi_leg_reporting_type: Option<MultiLegReportingType>,
	/// To requests trades that were submitted from a specific trade input source.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "578")]
	pub trade_input_source: Option<String>,
	/// To request trades that were submitted from a specific trade input device.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "579")]
	pub trade_input_device: Option<String>,
	/// Ability to specify whether the response to the request should be delivered inband or via pre-arranged out-of-band transport.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "725")]
	pub response_transport_type: Option<ResponseTransportType>,
	/// URI destination name. Used if <a href="tag_725_ResponseTransportType.html" target="bottom">ResponseTransportType&nbsp;(725)</a> is out-of-band.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "726")]
	pub response_destination: Option<String>,
	/// Used to match specific values within Text fields
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// EncodedTextLen
	#[serde(rename = "354")]
	/// EncodedText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
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

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Date {
	/// Used when reporting other than current day trades. Conditionally required if <a href="tag_580_NoDates.html" target="bottom">NoDates&nbsp;(580)</a> &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "75")]
	pub trade_date: Option<fix_common::LocalMktDate>,
	/// To request trades for a specific time.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<fix_common::UTCTimestamp>,
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
pub enum ExecType {
	/// New
	#[serde(rename = "0")]
	New,
	/// Done for day
	#[serde(rename = "3")]
	DoneForDay,
	/// Canceled
	#[serde(rename = "4")]
	Canceled,
	/// Replace
	#[serde(rename = "5")]
	Replace,
	/// Pending Cancel (e.g. result of <a href="message_Order_Cancel_Request_F.html" target="main">Order Cancel Request&nbsp;(F)</a> )
	#[serde(rename = "6")]
	PendingCancelA,
	/// Stopped
	#[serde(rename = "7")]
	Stopped,
	/// Rejected
	#[serde(rename = "8")]
	Rejected,
	/// Suspended
	#[serde(rename = "9")]
	Suspended,
	/// Pending New
	#[serde(rename = "A")]
	PendingNew,
	/// Calculated
	#[serde(rename = "B")]
	Calculated,
	/// Expired
	#[serde(rename = "C")]
	Expired,
	/// Restated ( <a href="message_Execution_Report_8.html" target="main">ExecutionRpt&nbsp;(8)</a> sent unsolicited by sellside, with <a href="tag_378_ExecRestatementReason.html" target="bottom">ExecRestatementReason&nbsp;(378)</a> set)
	#[serde(rename = "D")]
	RestatedASentUnsolicitedBySellsideWithAHrefTag378ExecRestatementReasonHtmlTargetBottomExecRestatementReasonNbspASet,
	/// Pending Replace (e.g. result of <a href="message_Order_Cancel_Replace_Request_G.html" target="main">Order Cancel/Replace Request&nbsp;(G)</a> )
	#[serde(rename = "E")]
	PendingReplaceA,
	/// Trade (partial fill or fill)
	#[serde(rename = "F")]
	Trade,
	/// Trade Correct (formerly an ExecTransType)
	#[serde(rename = "G")]
	TradeCorrect,
	/// Trade Cancel (formerly an ExecTransType)
	#[serde(rename = "H")]
	TradeCancel,
	/// Order Status (formerly an ExecTransType)
	#[serde(rename = "I")]
	OrderStatus,
}

impl Default for ExecType {
	fn default() -> Self {
		ExecType::New
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MatchStatus {
	/// compared, matched or affirmed
	#[serde(rename = "0")]
	ComparedMatchedOrAffirmed,
	/// uncompared, unmatched, or unaffirmed
	#[serde(rename = "1")]
	UncomparedUnmatchedOrUnaffirmed,
	/// Advisory or alert
	#[serde(rename = "2")]
	AdvisoryOrAlert,
}

impl Default for MatchStatus {
	fn default() -> Self {
		MatchStatus::ComparedMatchedOrAffirmed
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TrdType {
	/// Regular Trade
	#[serde(rename = "0")]
	RegularTrade,
	/// Block Trade
	#[serde(rename = "1")]
	BlockTrade,
	/// EFP (Exchange for physical)
	#[serde(rename = "2")]
	Efp,
	/// Transfer
	#[serde(rename = "3")]
	Transfer,
	/// Late Trade
	#[serde(rename = "4")]
	LateTrade,
	/// T Trade
	#[serde(rename = "5")]
	TTrade,
	/// Weighted Average Price Trade
	#[serde(rename = "6")]
	WeightedAveragePriceTrade,
	/// Bunched Trade
	#[serde(rename = "7")]
	BunchedTrade,
	/// Late Bunched Trade
	#[serde(rename = "8")]
	LateBunchedTrade,
	/// Prior Reference Price Trade
	#[serde(rename = "9")]
	PriorReferencePriceTrade,
	/// After Hours Trade
	#[serde(rename = "10")]
	AfterHoursTrade,
}

impl Default for TrdType {
	fn default() -> Self {
		TrdType::RegularTrade
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SecondaryTrdType {
	/// Regular Trade
	#[serde(rename = "0")]
	RegularTrade,
	/// Block Trade
	#[serde(rename = "1")]
	BlockTrade,
	/// EFP (Exchange for physical)
	#[serde(rename = "2")]
	Efp,
	/// Transfer
	#[serde(rename = "3")]
	Transfer,
	/// Late Trade
	#[serde(rename = "4")]
	LateTrade,
	/// T Trade
	#[serde(rename = "5")]
	TTrade,
	/// Weighted Average Price Trade
	#[serde(rename = "6")]
	WeightedAveragePriceTrade,
	/// Bunched Trade
	#[serde(rename = "7")]
	BunchedTrade,
	/// Late Bunched Trade
	#[serde(rename = "8")]
	LateBunchedTrade,
	/// Prior Reference Price Trade
	#[serde(rename = "9")]
	PriorReferencePriceTrade,
	/// After Hours Trade
	#[serde(rename = "10")]
	AfterHoursTrade,
}

impl Default for SecondaryTrdType {
	fn default() -> Self {
		SecondaryTrdType::RegularTrade
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Side {
	/// Buy
	#[serde(rename = "1")]
	Buy,
	/// Sell
	#[serde(rename = "2")]
	Sell,
	/// Buy minus
	#[serde(rename = "3")]
	BuyMinus,
	/// Sell plus
	#[serde(rename = "4")]
	SellPlus,
	/// Sell short
	#[serde(rename = "5")]
	SellShort,
	/// Sell short exempt
	#[serde(rename = "6")]
	SellShortExempt,
	/// Undisclosed (valid for IOI and List Order messages only)
	#[serde(rename = "7")]
	Undisclosed,
	/// Cross (orders where counterparty is an exchange, valid for all messages except IOIs)
	#[serde(rename = "8")]
	Cross,
	/// Cross short
	#[serde(rename = "9")]
	CrossShort,
	/// Cross short exempt
	#[serde(rename = "A")]
	CrossShortExempt,
	/// "As Defined" (for use with multileg instruments)
	#[serde(rename = "B")]
	AsDefined,
	/// "Opposite" (for use with multileg instruments)
	#[serde(rename = "C")]
	Opposite,
	/// Subscribe (e.g. CIV)
	#[serde(rename = "D")]
	Subscribe,
	/// Redeem (e.g. CIV)
	#[serde(rename = "E")]
	Redeem,
	/// Lend (FINANCING - identifies direction of collateral)
	#[serde(rename = "F")]
	Lend,
	/// Borrow (FINANCING - identifies direction of collateral)
	#[serde(rename = "G")]
	Borrow,
}

impl Default for Side {
	fn default() -> Self {
		Side::Buy
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
	OutOfBandPreArrangedOutOfBandDeliveryMechanismBetweenCounterpartiesDetailsSpecifiedViaAHrefTag726ResponseDestinationHtmlTargetBottomResponseDestinationNbspA,
}

impl Default for ResponseTransportType {
	fn default() -> Self {
		ResponseTransportType::InbandTransportTheRequestWasSentOver
	}
}
