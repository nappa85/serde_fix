
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamPricingDayGrp {
	/// NoUnderlyingPaymentStreamPricingDays
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41944")]
	pub underlying_payment_stream_pricing_days: Option<crate::entities::RepeatingValues<UnderlyingPaymentStreamPricingDay>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamPricingDay {
	/// Required if NoUnderlyingPaymentStreamPricingDays(41944) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41945")]
	pub underlying_payment_stream_pricing_day_of_week: Option<UnderlyingPaymentStreamPricingDayOfWeek>,
	/// UnderlyingPaymentStreamPricingDayNumber
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41946")]
	pub underlying_payment_stream_pricing_day_number: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingPaymentStreamPricingDayOfWeek {
	/// Every day (the default if not specified)
	#[serde(rename = "0")]
	EveryDay,
	/// Monday
	#[serde(rename = "1")]
	Monday,
	/// Tuesday
	#[serde(rename = "2")]
	Tuesday,
	/// Wednesday
	#[serde(rename = "3")]
	Wednesday,
	/// Thursday
	#[serde(rename = "4")]
	Thursday,
	/// Friday
	#[serde(rename = "5")]
	Friday,
	/// Saturday
	#[serde(rename = "6")]
	Saturday,
	/// Sunday
	#[serde(rename = "7")]
	Sunday,
}
