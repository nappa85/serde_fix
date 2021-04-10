
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamPricingDateGrp {
	/// NoLegPaymentStreamPricingDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41593")]
	pub leg_payment_stream_pricing_dates: Option<fix_common::RepeatingValues<LegPaymentStreamPricingDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamPricingDate {
	/// Required if NoPaymentStreamPricingDates(41593) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41594")]
	pub leg_payment_stream_pricing_date: Option<fix_common::LocalMktDate>,
	/// LegPaymentStreamPricingDateType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41595")]
	pub leg_payment_stream_pricing_date_type: Option<LegPaymentStreamPricingDateType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegPaymentStreamPricingDateType {
	/// Unadjusted
	#[serde(rename = "0")]
	Unadjusted,
	/// Adjusted
	#[serde(rename = "1")]
	Adjusted,
}
