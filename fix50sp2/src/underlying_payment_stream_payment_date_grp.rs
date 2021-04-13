
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamPaymentDateGrp {
	/// NoUnderlyingPaymentStreamPaymentDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41937")]
	pub underlying_payment_stream_payment_dates: Option<fix_common::RepeatingValues<UnderlyingPaymentStreamPaymentDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamPaymentDate {
	/// Required if NoUnderlyingPaymentStreamPaymentDates(41937) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41938")]
	pub underlying_payment_stream_payment_date: Option<fix_common::LocalMktDate>,
	/// UnderlyingPaymentStreamPaymentDateType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41939")]
	pub underlying_payment_stream_payment_date_type: Option<UnderlyingPaymentStreamPaymentDateType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingPaymentStreamPaymentDateType {
	/// Unadjusted
	#[serde(rename = "0")]
	Unadjusted,
	/// Adjusted
	#[serde(rename = "1")]
	Adjusted,
}

impl Default for UnderlyingPaymentStreamPaymentDateType {
	fn default() -> Self {
		UnderlyingPaymentStreamPaymentDateType::Unadjusted
	}
}
