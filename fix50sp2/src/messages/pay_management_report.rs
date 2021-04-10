
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PayManagementReport {
	/// MsgType = DY
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// PayReportID
	#[serde(rename = "2799")]
	pub pay_report_id: String,
	/// Conditionally required when responding to PayRequest(35=EA). Omitted for an unsolicited report.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2812")]
	pub pay_request_id: Option<String>,
	/// PayReportTransType
	#[serde(rename = "2804")]
	pub pay_report_trans_type: PayReportTransType,
	/// Required for PayReportTransType(2804)=1 (Replace).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2803")]
	pub pay_report_ref_id: Option<String>,
	/// May be used to provide reason for PayReportTransType(2804)=1 (Replace).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2805")]
	pub replace_text: Option<String>,
	/// Must be set if EncodedReplaceText(2801) field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2802")]
	pub encoded_replace_text_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the ReplaceText(2805) field in the encoded format specified via the MessageEncoding(347)
	/// field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2801")]
	pub encoded_replace_text: Option<String>,
	/// PayRequestStatus(2813)=0 (Received) is not applicable in this message.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2813")]
	pub pay_request_status: Option<PayRequestStatus>,
	/// May be used to provide reason for PayRequestStatus(2813)=3 (Disputed).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2800")]
	pub pay_dispute_reason: Option<i32>,
	/// May be used to elaborate the reason for rejection or dispute.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1328")]
	pub reject_text: Option<String>,
	/// Must be set if EncodedRejectText(1665) field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1664")]
	pub encoded_reject_text_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the RejectText(1328) field in the encoded format specified via the MessageEncoding(347)
	/// field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1665")]
	pub encoded_reject_text: Option<String>,
	/// Echos back the business date of the PayRequest(35=EA) message if this report is responding to a request. When the report is
	/// sent unsolicited, this is the business date of the report. This may carry the same date as the payment calculation date in
	/// PostTradePaymentCalculationDate(2825).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "715")]
	pub clearing_business_date: Option<fix_common::LocalMktDate>,
	/// TransactTime
	#[serde(rename = "60")]
	pub transact_time: fix_common::UTCTimestamp,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if EncodedText(355) field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "354")]
	pub encoded_text_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the Text(58) field in the encoded format specified via the MessageEncoding(347)
	/// field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "355")]
	pub encoded_text: Option<String>,
	/// May be included with minimal detail to identify the security or contract for which payments are to be made.
	#[serde(flatten)]
	pub instrument: Option<super::super::instrument::Instrument>,
	/// May be included to identify the trade(s) for which payments are to be made. Each instance identifies a separate trade.
	#[serde(flatten)]
	pub related_trade_grp: Option<super::super::related_trade_grp::RelatedTradeGrp>,
	/// Identifies the parties to the contracts or trades. The account to be debited or credited is identified in the PostTradePayment
	/// component.
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// PostTradePayment
	#[serde(flatten)]
	pub post_trade_payment: super::super::post_trade_payment::PostTradePayment,
	/// SettlDetails
	#[serde(flatten)]
	pub settl_details: Option<super::super::settl_details::SettlDetails>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PayReportTransType {
	/// New
	#[serde(rename = "0")]
	New,
	/// Replace
	#[serde(rename = "1")]
	Replace,
	/// Status
	#[serde(rename = "2")]
	Status,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PayRequestStatus {
	/// Received, not yet processed
	#[serde(rename = "0")]
	ReceivedNotYetProcessed,
	/// Accepted
	#[serde(rename = "1")]
	Accepted,
	/// Rejected
	#[serde(rename = "2")]
	Rejected,
	/// Disputed
	#[serde(rename = "3")]
	Disputed,
}
