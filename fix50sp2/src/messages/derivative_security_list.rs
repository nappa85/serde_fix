
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Derivative {
	/// MsgType = AA
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// ApplicationSequenceControl
	#[serde(flatten)]
	pub application_sequence_control: Option<super::super::application_sequence_control::ApplicationSequenceControl>,
	/// SecurityReportID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "964")]
	pub security_report_id: Option<i32>,
	/// SecurityReqID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "320")]
	pub security_req_id: Option<String>,
	/// Identifier for the Derivative <a href="message_Security_List_y.html" target="main">Security List&nbsp;(y)</a> message
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "322")]
	pub security_response_id: Option<String>,
	/// Result of the Security Request identified by SecurityReqID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "560")]
	pub security_request_result: Option<SecurityRequestResult>,
	/// ClearingBusinessDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "715")]
	pub clearing_business_date: Option<crate::entities::LocalMktDate>,
	/// Underlying security for which derivatives are being returned
	#[serde(flatten)]
	pub underlying_instrument: Option<super::super::underlying_instrument::UnderlyingInstrument>,
	/// Group block which contains all information for an option family. If provided DerivativeSecurityDefinition qualifies the strikes
	/// specified in the Instrument block.
	#[serde(flatten)]
	pub derivative_security_definition: Option<super::super::derivative_security_definition::DerivativeSecurityDefinition>,
	/// TransactTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<crate::entities::UTCTimestamp>,
	/// Used to indicate the total number of securities being returned for this request. Used in the event that message fragmentation
	/// is required.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "393")]
	pub tot_no_related_sym: Option<i32>,
	/// Indicates whether this is the last fragment in a sequence of message fragments. Only required where message has been fragmented.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "893")]
	pub last_fragment: Option<LastFragment>,
	/// Specifies the number of repeating symbols (instruments) specified
	#[serde(flatten)]
	pub rel_sym_deriv_sec_grp: Option<super::super::rel_sym_deriv_sec_grp::RelSymDerivSecGrp>,
	/// Represents the time at which a security was last updated.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "779")]
	pub last_update_time: Option<crate::entities::UTCTimestamp>,
	/// Used to specify a rejection reason when SecurityResponseType (323) is equal to 5 (Reject Security Proposal).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1607")]
	pub security_reject_reason: Option<SecurityRejectReason>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LastFragment {
	/// Not Last Message
	#[serde(rename = "N")]
	NotLastMessage,
	/// Last Message
	#[serde(rename = "Y")]
	LastMessage,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SecurityRejectReason {
	/// Invalid instrument requested
	#[serde(rename = "1")]
	InvalidInstrumentRequested,
	/// Instrument already exists
	#[serde(rename = "2")]
	InstrumentAlreadyExists,
	/// Request type not supported
	#[serde(rename = "3")]
	RequestTypeNotSupported,
	/// System unavailable for instrument creation
	#[serde(rename = "4")]
	SystemUnavailableForInstrumentCreation,
	/// Ineligible instrument group
	#[serde(rename = "5")]
	IneligibleInstrumentGroup,
	/// Instrument ID unavailable
	#[serde(rename = "6")]
	InstrumentIdUnavailable,
	/// Invalid or missing data on option leg
	#[serde(rename = "7")]
	InvalidOrMissingDataOnOptionLeg,
	/// Invalid or missing data on future leg
	#[serde(rename = "8")]
	InvalidOrMissingDataOnFutureLeg,
	/// Invalid or missing data on FX leg
	#[serde(rename = "10")]
	InvalidOrMissingDataOnFxLeg,
	/// Invalid leg price specified
	#[serde(rename = "11")]
	InvalidLegPriceSpecified,
	/// Invalid instrument structure specified
	#[serde(rename = "12")]
	InvalidInstrumentStructureSpecified,
}
