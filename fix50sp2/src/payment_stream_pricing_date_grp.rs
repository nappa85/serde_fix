
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamPricingDateGrp {
	/// NoPaymentStreamPricingDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41224")]
	pub payment_stream_pricing_dates: Option<fix_common::RepeatingValues<PaymentStreamPricingDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamPricingDate {
	/// Required if NoPaymentStreamPricingDates(41224) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41225")]
	pub payment_stream_pricing_date: Option<fix_common::LocalMktDate>,
	/// PaymentStreamPricingDateType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41226")]
	pub payment_stream_pricing_date_type: Option<PaymentStreamPricingDateType>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PaymentStreamPricingDateType {
	/// Unadjusted
	#[serde(rename = "0")]
	Unadjusted,
	/// Adjusted
	#[serde(rename = "1")]
	Adjusted,
}

impl Default for PaymentStreamPricingDateType {
	fn default() -> Self {
		PaymentStreamPricingDateType::Unadjusted
	}
}
