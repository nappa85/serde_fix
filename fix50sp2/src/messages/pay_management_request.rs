
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PayManagementRequest {
	/// MsgType = EA
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'E', 'A'>,
	/// PayRequestID
	#[serde(rename = "2812")]
	pub pay_request_id: String,
	/// PyRequestTransType
	#[serde(rename = "2811")]
	pub py_request_trans_type: PyRequestTransType,
	/// Required for PayRequestTransType(2811)=1 (Cancel).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2810")]
	pub pay_request_ref_id: Option<String>,
	/// May be used to provide reason for PayRequestTransType(2811)=1 (Cancel).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2807")]
	pub cancel_text: Option<String>,
	/// Must be set if EncodedCancelText(2808) field is specified and must immediately precede it.
	#[serde(rename = "2809")]
	/// Encoded (non-ASCII characters) representation of the CancelText(2807) field in the encoded format specified via the MessageEncoding(347)
	/// field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "2808")]
	pub encoded_cancel_text: Option<fix_common::EncodedText<2808>>,
	/// The business date of the request. This may carry the same date as the payment calculation date in PostTradePaymentCalculationDate(2825).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "715")]
	pub clearing_business_date: Option<fix_common::LocalMktDate>,
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PyRequestTransType {
	/// New
	#[serde(rename = "0")]
	New,
	/// Cancel
	#[serde(rename = "1")]
	Cancel,
}

impl Default for PyRequestTransType {
	fn default() -> Self {
		PyRequestTransType::New
	}
}
