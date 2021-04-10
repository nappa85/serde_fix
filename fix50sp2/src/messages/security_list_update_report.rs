
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Security {
	/// MsgType = BK
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// ApplicationSequenceControl
	#[serde(flatten)]
	pub application_sequence_control: Option<super::super::application_sequence_control::ApplicationSequenceControl>,
	/// Identifier for the <a href="message_Security_List_y.html" target="main">Security List&nbsp;(y)</a> Update message in a bulk transfer environment (No Request/Response)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "964")]
	pub security_report_id: Option<i32>,
	/// Identifies a specific Security List entity
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1465")]
	pub security_list_id: Option<String>,
	/// Provides a reference to another Security List
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1466")]
	pub security_list_ref_id: Option<String>,
	/// SecurityListDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1467")]
	pub security_list_desc: Option<String>,
	/// EncodedSecurityListDescLen
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1468")]
	pub encoded_security_list_desc_len: Option<usize>,
	/// EncodedSecurityListDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1469")]
	pub encoded_security_list_desc: Option<String>,
	/// Identifies a list type
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1470")]
	pub security_list_type: Option<SecurityListType>,
	/// Identifies the sourec as a listype
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1471")]
	pub security_list_type_source: Option<SecurityListTypeSource>,
	/// SecurityReqID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "320")]
	pub security_req_id: Option<String>,
	/// Identifier for the <a href="message_Security_List_y.html" target="main">Security List&nbsp;(y)</a> message.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "322")]
	pub security_response_id: Option<String>,
	/// Result of the Security Request identified by the SecurityReqID.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "560")]
	pub security_request_result: Option<SecurityRequestResult>,
	/// Used to indicate the total number of securities being returned for this request. Used in the event that message fragmentation
	/// is required.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "393")]
	pub tot_no_related_sym: Option<i32>,
	/// ClearingBusinessDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "715")]
	pub clearing_business_date: Option<fix_common::LocalMktDate>,
	/// SecurityUpdateAction
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "980")]
	pub security_update_action: Option<SecurityUpdateAction>,
	/// Identifies the type of Corporate Action that triggered the update
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "292")]
	pub corporate_action: Option<fix_common::SeparatedValues<CorporateAction>>,
	/// Identifies the market which lists and trades the instrument.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1301")]
	pub market_id: Option<String>,
	/// Identifies the segment of the market to which the specify trading rules and listing rules apply.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1300")]
	pub market_segment_id: Option<String>,
	/// TransactTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<fix_common::UTCTimestamp>,
	/// Indicates whether this is the last fragment in a sequence of message fragments. Only required where message has been fragmented.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "893")]
	pub last_fragment: Option<LastFragment>,
	/// Specifies the number of repeating symbols (instruments) specified
	#[serde(flatten)]
	pub sec_lst_upd_rel_sym_grp: Option<super::super::sec_lst_upd_rel_sym_grp::SecLstUpdRelSymGrp>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum SecurityListType {
	/// Industry Classification
	#[serde(rename = "1")]
	IndustryClassification,
	/// Trading List
	#[serde(rename = "2")]
	TradingList,
	/// Market / Market Segment List
	#[serde(rename = "3")]
	MarketMarketSegmentList,
	/// Newspaper List
	#[serde(rename = "4")]
	NewspaperList,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum SecurityListTypeSource {
	/// ICB (Industry Classification Benchmark) published by Dow Jones and FTSE - <a xmlns="http://www.b2bits.com/FIXProtocol" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" href="http://www.icbenchmark.com" target="_blank">www.icbenchmark.com</a> .
	#[serde(rename = "1")]
	IcbPublishedByDowJonesAndFtseAXmlnsHttpWwwB2BitsComFixProtocolXmlnsXsiHttpWwwW3Org2001XmlSchemaInstanceHrefHttpWwwIcbenchmarkComTargetBlankWwwIcbenchmarkComA,
	/// NAICS (North American Industry Classification System). Replaced SIC (Standard Industry Classification) naics <a xmlns="http://www.b2bits.com/FIXProtocol" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" href="http://www.census.gov/naics" target="_blank">www.census.gov/naics</a> or <a xmlns="http://www.b2bits.com/FIXProtocol" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" href="http://www.naics.com" target="_blank">www.naics.com</a> .
	#[serde(rename = "2")]
	NaicsReplacedSicNaicsAXmlnsHttpWwwB2BitsComFixProtocolXmlnsXsiHttpWwwW3Org2001XmlSchemaInstanceHrefHttpWwwCensusGovNaicsTargetBlankWwwCensusGovNaicsAOrAXmlnsHttpWwwB2BitsComFixProtocolXmlnsXsiHttpWwwW3Org2001XmlSchemaInstanceHrefHttpWwwNaicsComTargetBlankWwwNaicsComA,
	/// GICS (Global Industry Classification Standard) published by Standards and Poor.
	#[serde(rename = "3")]
	GicsPublishedByStandardsAndPoor,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum SecurityRequestResult {
	/// Valid request
	#[serde(rename = "0")]
	ValidRequest,
	/// Invalid or unsupported request
	#[serde(rename = "1")]
	InvalidOrUnsupportedRequest,
	/// No instruments found that match selection criteria
	#[serde(rename = "2")]
	NoInstrumentsFoundThatMatchSelectionCriteria,
	/// Not authorized to retrieve instrument data
	#[serde(rename = "3")]
	NotAuthorizedToRetrieveInstrumentData,
	/// Instrument data temporarily unavailable
	#[serde(rename = "4")]
	InstrumentDataTemporarilyUnavailable,
	/// Request for instrument data not supported
	#[serde(rename = "5")]
	RequestForInstrumentDataNotSupported,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum SecurityUpdateAction {
	/// Add
	#[serde(rename = "A")]
	Add,
	/// Delete
	#[serde(rename = "D")]
	Delete,
	/// Modify
	#[serde(rename = "M")]
	Modify,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum CorporateAction {
	/// Ex-Dividend
	#[serde(rename = "A")]
	ExDividend,
	/// Ex-Distribution
	#[serde(rename = "B")]
	ExDistribution,
	/// Ex-Rights
	#[serde(rename = "C")]
	ExRights,
	/// New
	#[serde(rename = "D")]
	New,
	/// Ex-Interest
	#[serde(rename = "E")]
	ExInterest,
	/// Cash Dividend
	#[serde(rename = "F")]
	CashDividend,
	/// Stock Dividend
	#[serde(rename = "G")]
	StockDividend,
	/// Non-Integer Stock Split
	#[serde(rename = "H")]
	NonIntegerStockSplit,
	/// Reverse Stock Split
	#[serde(rename = "I")]
	ReverseStockSplit,
	/// Standard-Integer Stock Split
	#[serde(rename = "J")]
	StandardIntegerStockSplit,
	/// Position Consolidation
	#[serde(rename = "K")]
	PositionConsolidation,
	/// Liquidation Reorganization
	#[serde(rename = "L")]
	LiquidationReorganization,
	/// Merger Reorganization
	#[serde(rename = "M")]
	MergerReorganization,
	/// Rights Offering
	#[serde(rename = "N")]
	RightsOffering,
	/// Shareholder Meeting
	#[serde(rename = "O")]
	ShareholderMeeting,
	/// Spinoff
	#[serde(rename = "P")]
	Spinoff,
	/// Tender Offer
	#[serde(rename = "Q")]
	TenderOffer,
	/// Warrant
	#[serde(rename = "R")]
	Warrant,
	/// Special Action
	#[serde(rename = "S")]
	SpecialAction,
	/// Symbol Conversion
	#[serde(rename = "T")]
	SymbolConversion,
	/// CUSIP / Name Change
	#[serde(rename = "U")]
	CusipNameChange,
	/// Leap Rollover
	#[serde(rename = "V")]
	LeapRollover,
	/// Succession Event
	#[serde(rename = "W")]
	SuccessionEvent,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum LastFragment {
	/// Not Last Message
	#[serde(rename = "N")]
	NotLastMessage,
	/// Last Message
	#[serde(rename = "Y")]
	LastMessage,
}
