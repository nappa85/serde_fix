
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamPricingDayGrp {
	/// NoPaymentStreamPricingDays
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41227")]
	pub payment_stream_pricing_days: Option<fix_common::RepeatingValues<PaymentStreamPricingDay>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamPricingDay {
	/// Required if NoPaymentStreamPricingDays(41227) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41228")]
	pub payment_stream_pricing_day_of_week: Option<PaymentStreamPricingDayOfWeek>,
	/// PaymentStreamPricingDayNumber
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41229")]
	pub payment_stream_pricing_day_number: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PaymentStreamPricingDayOfWeek {
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

impl Default for PaymentStreamPricingDayOfWeek {
	fn default() -> Self {
		PaymentStreamPricingDayOfWeek::EveryDay
	}
}
