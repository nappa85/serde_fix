
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PartyRiskLimitsUpdateReport {
	/// MsgType = CR
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'C', 'R'>,
	/// ApplicationSequenceControl
	#[serde(flatten)]
	pub application_sequence_control: Option<super::super::application_sequence_control::ApplicationSequenceControl>,
	/// RiskLimitReportID
	#[serde(rename = "1667")]
	pub risk_limit_report_id: String,
	/// <p>Conditionally required when sent as part of a subscription requested by a PartyRiskLimitsRequest(35=CL).</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1666")]
	pub risk_limit_request_id: Option<String>,
	/// Can be used if sent as part of a subscription started by a PartyRiskLimitsRequest(35=CL).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1760")]
	pub risk_limit_request_type: Option<RiskLimitRequestType>,
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
	/// PartyRiskLimitsUpdateGrp
	#[serde(flatten)]
	pub party_risk_limits_update_grp: Option<super::super::party_risk_limits_update_grp::PartyRiskLimitsUpdateGrp>,
	/// TransactTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<fix_common::UTCTimestamp>,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if EncodedText(355) field is specified and must immediately precede it.
	#[serde(rename = "354")]
	/// Encoded (non-ASCII characters) representation of the Text(58) field in the encoded format specified via the MessageEncoding(347)
	/// field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
	/// May be used to specify the requesting party in the event the request was made verbally or via other means.
	#[serde(flatten)]
	pub requesting_party_grp: Option<super::super::requesting_party_grp::RequestingPartyGrp>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum RiskLimitRequestType {
	/// Definitions (default)
	#[serde(rename = "1")]
	Definitions,
	/// Utilization
	#[serde(rename = "2")]
	Utilization,
	/// Definitions and utilizations
	#[serde(rename = "3")]
	DefinitionsAndUtilizations,
}

impl Default for RiskLimitRequestType {
	fn default() -> Self {
		RiskLimitRequestType::Definitions
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
