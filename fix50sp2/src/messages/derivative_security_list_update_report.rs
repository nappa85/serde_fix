
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Derivative {
	/// MsgType = BR
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// ApplicationSequenceControl
	#[serde(flatten)]
	pub application_sequence_control: Option<super::super::application_sequence_control::ApplicationSequenceControl>,
	/// SecurityReqID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "320")]
	pub security_req_id: Option<String>,
	/// Identifier for the Derivative Security List message.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "322")]
	pub security_response_id: Option<String>,
	/// Result of the Security Request identified by SecurityReqID.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "560")]
	pub security_request_result: Option<SecurityRequestResult>,
	/// Updates can be applied to Underlying or option class. If Series information provided, then Series has explicitly changed.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "980")]
	pub security_update_action: Option<SecurityUpdateAction>,
	/// Underlying security for which derivatives are being returned.
	#[serde(flatten)]
	pub underlying_instrument: Option<super::super::underlying_instrument::UnderlyingInstrument>,
	/// Group block which contains all information for an option family. If provided DerivativeSecurityDefinition qualifies the strikes
	/// specified in the Instrument block. DerivativeSecurityDefinition contains the following components: DerivativeInstrument. DerivativeInstrumentExtension,
	/// MarketSegmentGrp
	#[serde(flatten)]
	pub derivative_security_definition: Option<super::super::derivative_security_definition::DerivativeSecurityDefinition>,
	/// TransactTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<fix_common::UTCTimestamp>,
	/// Used to indicate the total number of securities being returned for this request. Used in the event that message fragmentation
	/// is required.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "393")]
	pub tot_no_related_sym: Option<i32>,
	/// Indicates whether this is the last fragment in a sequence of message fragments. Only required where message has been fragmented.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "893")]
	pub last_fragment: Option<LastFragment>,
	/// RelSymDerivSecUpdGrp
	#[serde(flatten)]
	pub rel_sym_deriv_sec_upd_grp: Option<super::super::rel_sym_deriv_sec_upd_grp::RelSymDerivSecUpdGrp>,
	/// Represents the time at which a security was last updated.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "779")]
	pub last_update_time: Option<fix_common::UTCTimestamp>,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LastFragment {
	/// Not Last Message
	#[serde(rename = "N")]
	NotLastMessage,
	/// Last Message
	#[serde(rename = "Y")]
	LastMessage,
}
