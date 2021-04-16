
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamPricingDateGrp {
	/// NoUnderlyingPaymentStreamPricingDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41941")]
	pub underlying_payment_stream_pricing_dates: Option<fix_common::RepeatingValues<UnderlyingPaymentStreamPricingDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamPricingDate {
	/// Required if NoUnderlyingPaymentStreamPricingDates(41941) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41942")]
	pub underlying_payment_stream_pricing_date: Option<fix_common::LocalMktDate>,
	/// UnderlyingPaymentStreamPricingDateType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41943")]
	pub underlying_payment_stream_pricing_date_type: Option<UnderlyingPaymentStreamPricingDateType>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingPaymentStreamPricingDateType {
	/// Unadjusted
	#[serde(rename = "0")]
	Unadjusted,
	/// Adjusted
	#[serde(rename = "1")]
	Adjusted,
}

impl Default for UnderlyingPaymentStreamPricingDateType {
	fn default() -> Self {
		UnderlyingPaymentStreamPricingDateType::Unadjusted
	}
}
