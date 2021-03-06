
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PartyEntitlementsReport {
	/// MsgType = CV
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'C', 'V'>,
	/// ApplicationSequenceControl
	#[serde(flatten)]
	pub application_sequence_control: Option<super::super::application_sequence_control::ApplicationSequenceControl>,
	/// EntitlementReportID
	#[serde(rename = "1771")]
	pub entitlement_report_id: String,
	/// Conditionally required when responding to PartyEntitlementsRequest(35=CU).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1770")]
	pub entitlement_request_id: Option<String>,
	/// Conditionally required when responding to PartyEntitlementsRequest(35=CU).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1511")]
	pub request_result: Option<RequestResult>,
	/// TotNoParties
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1512")]
	pub tot_no_parties: Option<i32>,
	/// LastFragment
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "893")]
	pub last_fragment: Option<LastFragment>,
	/// PartyEntitlementGrp
	#[serde(flatten)]
	pub party_entitlement_grp: Option<super::super::party_entitlement_grp::PartyEntitlementGrp>,
	/// TransactTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<fix_common::UTCTimestamp>,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// EncodedTextLen
	#[serde(rename = "354")]
	/// EncodedText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
	/// RejectText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1328")]
	pub reject_text: Option<String>,
	/// EncodedRejectTextLen
	#[serde(rename = "1664")]
	/// EncodedRejectText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "1665")]
	pub encoded_reject_text: Option<fix_common::EncodedText<1665>>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum RequestResult {
	/// Valid request
	#[serde(rename = "0")]
	ValidRequest,
	/// Invalid or unsupported request
	#[serde(rename = "1")]
	InvalidOrUnsupportedRequest,
	/// No data found that match selection criteria
	#[serde(rename = "2")]
	NoDataFoundThatMatchSelectionCriteria,
	/// Not authorized to retrieve data
	#[serde(rename = "3")]
	NotAuthorizedToRetrieveData,
	/// Data temporarily unavailable
	#[serde(rename = "4")]
	DataTemporarilyUnavailable,
	/// Request for data not supported
	#[serde(rename = "5")]
	RequestForDataNotSupported,
	/// Other (further information in RejectText (1328) field)
	#[serde(rename = "99")]
	OtherField,
}

impl Default for RequestResult {
	fn default() -> Self {
		RequestResult::ValidRequest
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LastFragment {
	/// Not Last Message
	#[serde(rename = "N")]
	NotLastMessage,
	/// Last Message
	#[serde(rename = "Y")]
	LastMessage,
}

impl Default for LastFragment {
	fn default() -> Self {
		LastFragment::NotLastMessage
	}
}
