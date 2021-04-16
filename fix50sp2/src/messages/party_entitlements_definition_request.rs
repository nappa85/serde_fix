
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PartyEntitlementsDefinitionRequest {
	/// MsgType = DA
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'D', 'A'>,
	/// EntitlementRequestID
	#[serde(rename = "1770")]
	pub entitlement_request_id: String,
	/// Can be used to identify the party making the request and their role.
	#[serde(flatten)]
	pub requesting_party_grp: Option<super::super::requesting_party_grp::RequestingPartyGrp>,
	/// Specifies the entitlements to be defined, modified or deleted for the given party(-ies) and related party(-ies).
	#[serde(flatten)]
	pub party_entitlement_update_grp: super::super::party_entitlement_update_grp::PartyEntitlementUpdateGrp,
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
