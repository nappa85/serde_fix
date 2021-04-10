
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamPaymentDateGrp {
	/// NoPaymentStreamPaymentDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41220")]
	pub payment_stream_payment_dates: Option<fix_common::RepeatingValues<PaymentStreamPaymentDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamPaymentDate {
	/// Required if NoPaymentStreamPaymentDates(41220) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41221")]
	pub payment_stream_payment_date: Option<fix_common::LocalMktDate>,
	/// PaymentStreamPaymentDateType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41222")]
	pub payment_stream_payment_date_type: Option<PaymentStreamPaymentDateType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PaymentStreamPaymentDateType {
	/// Unadjusted
	#[serde(rename = "0")]
	Unadjusted,
	/// Adjusted
	#[serde(rename = "1")]
	Adjusted,
}
