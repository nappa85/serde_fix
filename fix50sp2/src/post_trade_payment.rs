
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PostTradePayment {
	/// PostTradePaymentType
	#[serde(rename = "2824")]
	pub post_trade_payment_type: String,
	/// PostTradePaymentAmount
	#[serde(deserialize_with = "crate::entities::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "2817")]
	pub post_trade_payment_amount: f64,
	/// PostTradePaymentCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2818")]
	pub post_trade_payment_currency: Option<String>,
	/// Only PayRequestStuats(2813)=0 (Received) is applicable in this message.
	#[serde(rename = "2825")]
	pub post_trade_payment_calculation_date: crate::entities::LocalMktDate,
	/// The date the payment is legally confirmed to settle.
	#[serde(rename = "2826")]
	pub post_trade_payment_value_date: crate::entities::LocalMktDate,
	/// The actual payment date in the event the payment differs from the date specified in PostTradePaymentValueDate(2826).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2827")]
	pub post_trade_payment_final_value_date: Option<crate::entities::LocalMktDate>,
	/// PostTradePaymentDebitOrCredit
	#[serde(rename = "2819")]
	pub post_trade_payment_debit_or_credit: PostTradePaymentDebitOrCredit,
	/// PostTradePaymentAccount
	#[serde(rename = "2816")]
	pub post_trade_payment_account: String,
	/// PostTradePaymentID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2821")]
	pub post_trade_payment_id: Option<String>,
	/// PostTradePaymentDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2820")]
	pub post_trade_payment_desc: Option<String>,
	/// Must be set if EncodedPostTradePaymentDesc(2814) field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2815")]
	pub encoded_post_trade_payment_desc_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the PostTradePaymentDesc(2820) field in the encoded format specified via
	/// the MessageEncoding(347) field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2814")]
	pub encoded_post_trade_payment_desc: Option<String>,
	/// PostTradePaymentLinkID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2822")]
	pub post_trade_payment_link_id: Option<String>,
	/// Used when PayReportTransType(2804)=2 (Status) to report actual payment status from payment service (i.e. after payment or
	/// remittance instruction with payment service).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2823")]
	pub post_trade_payment_status: Option<PostTradePaymentStatus>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PostTradePaymentDebitOrCredit {
	/// Debit / Pay
	#[serde(rename = "0")]
	DebitPay,
	/// Credit / Receive
	#[serde(rename = "1")]
	CreditReceive,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PostTradePaymentStatus {
	/// New
	#[serde(rename = "0")]
	New,
	/// Initiated
	#[serde(rename = "1")]
	Initiated,
	/// Pending
	#[serde(rename = "2")]
	Pending,
	/// Confirmed
	#[serde(rename = "3")]
	Confirmed,
	/// Rejected
	#[serde(rename = "4")]
	Rejected,
}
