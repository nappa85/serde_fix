
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamPaymentDateGrp {
	/// NoLegPaymentStreamPaymentDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41589")]
	pub leg_payment_stream_payment_dates: Option<fix_common::RepeatingValues<LegPaymentStreamPaymentDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamPaymentDate {
	/// Required if NoLegPaymentStreamPaymentDates(41589) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41590")]
	pub leg_payment_stream_payment_date: Option<fix_common::LocalMktDate>,
	/// LegPaymentStreamPaymentDateType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41591")]
	pub leg_payment_stream_payment_date_type: Option<LegPaymentStreamPaymentDateType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegPaymentStreamPaymentDateType {
	/// Unadjusted
	#[serde(rename = "0")]
	Unadjusted,
	/// Adjusted
	#[serde(rename = "1")]
	Adjusted,
}

impl Default for LegPaymentStreamPaymentDateType {
	fn default() -> Self {
		LegPaymentStreamPaymentDateType::Unadjusted
	}
}
