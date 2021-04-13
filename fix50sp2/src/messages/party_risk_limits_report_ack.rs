
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Party {
	/// MsgType = DE
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// The identifier of the Party Risk Limit Report (35=CM) or Party Risk Limit Update Report (35=CR) message.
	#[serde(rename = "1667")]
	pub risk_limit_report_id: String,
	/// RiskLimitRequestID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1666")]
	pub risk_limit_request_id: Option<String>,
	/// RiskLimitReportStatus
	#[serde(rename = "2316")]
	pub risk_limit_report_status: RiskLimitReportStatus,
	/// Conditionally required when RiskLimitReportStatus(2316) = 1 (Rejected).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2317")]
	pub risk_limit_report_reject_reason: Option<RiskLimitReportRejectReason>,
	/// PartyRiskLimitsUpdateGrp
	#[serde(flatten)]
	pub party_risk_limits_update_grp: Option<super::super::party_risk_limits_update_grp::PartyRiskLimitsUpdateGrp>,
	/// TransactTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<fix_common::UTCTimestamp>,
	/// RejectText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1328")]
	pub reject_text: Option<String>,
	/// Must be set if EncodedRejectText(1665) field is specified and must immediately precede it.
	#[serde(rename = "1664")]
	/// Encoded (non-ASCII characters) representation of the RejectText(1328) field in the encoded format specified via the MessageEncoding(347)
	/// field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "1665")]
	pub encoded_reject_text: Option<fix_common::EncodedText<1665>>,
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
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RiskLimitReportStatus {
	/// Accepted
	#[serde(rename = "0")]
	Accepted,
	/// Rejected
	#[serde(rename = "1")]
	Rejected,
}

impl Default for RiskLimitReportStatus {
	fn default() -> Self {
		RiskLimitReportStatus::Accepted
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RiskLimitReportRejectReason {
	/// Unknown RiskLimitReportID(1667)
	#[serde(rename = "0")]
	UnknownRiskLimitReportId,
	/// Unknown party
	#[serde(rename = "1")]
	UnknownParty,
	/// Other
	#[serde(rename = "99")]
	Other,
}

impl Default for RiskLimitReportRejectReason {
	fn default() -> Self {
		RiskLimitReportRejectReason::UnknownRiskLimitReportId
	}
}
