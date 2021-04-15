
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PartyEntitlementsDefinitionRequestAck {
	/// MsgType = DB
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'D', 'B'>,
	/// EntitlementRequestID
	#[serde(rename = "1770")]
	pub entitlement_request_id: String,
	/// EntitlementRequestStatus
	#[serde(rename = "1882")]
	pub entitlement_request_status: EntitlementRequestStatus,
	/// EntitlementRequestResult
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1881")]
	pub entitlement_request_result: Option<EntitlementRequestResult>,
	/// RequestingPartyGrp
	#[serde(flatten)]
	pub requesting_party_grp: Option<super::super::requesting_party_grp::RequestingPartyGrp>,
	/// PartyEntitlementAckGrp
	#[serde(flatten)]
	pub party_entitlement_ack_grp: Option<super::super::party_entitlement_ack_grp::PartyEntitlementAckGrp>,
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
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum EntitlementRequestStatus {
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

impl Default for EntitlementRequestStatus {
	fn default() -> Self {
		EntitlementRequestStatus::ValidRequest
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum EntitlementRequestResult {
	/// Successful(default)
	#[serde(rename = "0")]
	Successful,
	/// Invalid party(-ies)
	#[serde(rename = "1")]
	InvalidParty,
	/// Invalid related party(-ies)
	#[serde(rename = "2")]
	InvalidRelatedParty,
	/// Invalid entitlement type(s)
	#[serde(rename = "3")]
	InvalidEntitlementType,
	/// Invalid entitlement ID(s)/ref ID(s)
	#[serde(rename = "4")]
	InvalidEntitlementIdRefId,
	/// Invalid entitlement attribute(s)
	#[serde(rename = "5")]
	InvalidEntitlementAttribute,
	/// Invalid instrument scope(s)
	#[serde(rename = "6")]
	InvalidInstrumentScope,
	/// Invalid market segment scope(s)
	#[serde(rename = "7")]
	InvalidMarketSegmentScope,
	/// Invalid start date
	#[serde(rename = "8")]
	InvalidStartDate,
	/// Invalid end date
	#[serde(rename = "9")]
	InvalidEndDate,
	/// Instrument scope not supported
	#[serde(rename = "10")]
	InstrumentScopeNotSupported,
	/// Market segment scope not supported
	#[serde(rename = "11")]
	MarketSegmentScopeNotSupported,
	/// Entitlement not approved for party(-ies)
	#[serde(rename = "12")]
	EntitlementNotApprovedForParty,
	/// Entitlement already defined for party(-ies)
	#[serde(rename = "13")]
	EntitlementAlreadyDefinedForParty,
	/// Instrument not approved for party(-ies)
	#[serde(rename = "14")]
	InstrumentNotApprovedForParty,
	/// Not authorized
	#[serde(rename = "98")]
	NotAuthorized,
	/// Other
	#[serde(rename = "99")]
	Other,
}

impl Default for EntitlementRequestResult {
	fn default() -> Self {
		EntitlementRequestResult::Successful
	}
}
