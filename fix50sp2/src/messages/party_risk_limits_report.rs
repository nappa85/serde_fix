
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PartyRiskLimitsReport {
	/// MsgType = CM
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// ApplicationSequenceControl
	#[serde(flatten)]
	pub application_sequence_control: Option<super::super::application_sequence_control::ApplicationSequenceControl>,
	/// RiskLimitReportID
	#[serde(rename = "1667")]
	pub risk_limit_report_id: String,
	/// Conditionally required when responding to Party Risk Limits Request.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1666")]
	pub risk_limit_request_id: Option<String>,
	/// Conditionally required when responding to Party Risk Limits Request.
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
	/// <p>Optionally includes utilization (consumption) information.</p>
	#[serde(flatten)]
	pub party_risk_limits_grp: Option<super::super::party_risk_limits_grp::PartyRiskLimitsGrp>,
	/// TransactTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<fix_common::UTCTimestamp>,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// <p>Must be set if <a href="tag_355_EncodedText.html" target="bottom">EncodedText&nbsp;(355)</a> field is specified and must immediately precede it.
	/// </p>
	#[serde(rename = "354")]
	/// <p>Encoded (non-ASCII characters) representation of the <a href="tag_58_Text.html" target="bottom">Text&nbsp;(58)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	/// </p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
	/// RejectText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1328")]
	pub reject_text: Option<String>,
	/// Must be set if <a href="tag_1665_EncodedRejectText.html" target="bottom">EncodedRejectTextLen(1665)&nbsp;(1665)</a> field is specified and must immediately precede it.
	#[serde(rename = "1664")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_1328_RejectText.html" target="bottom">RejectedText(1328)&nbsp;(1328)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding(347)&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "1665")]
	pub encoded_reject_text: Option<fix_common::EncodedText<1665>>,
	/// <p>Scope of risk limit information</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1760")]
	pub risk_limit_request_type: Option<RiskLimitRequestType>,
	/// UnsolicitedIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "325")]
	pub unsolicited_indicator: Option<UnsolicitedIndicator>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnsolicitedIndicator {
	/// Message is being sent as a result of a prior request
	#[serde(rename = "N")]
	MessageIsBeingSentAsAResultOfAPriorRequest,
	/// Message is being sent unsolicited
	#[serde(rename = "Y")]
	MessageIsBeingSentUnsolicited,
}

impl Default for UnsolicitedIndicator {
	fn default() -> Self {
		UnsolicitedIndicator::MessageIsBeingSentAsAResultOfAPriorRequest
	}
}
