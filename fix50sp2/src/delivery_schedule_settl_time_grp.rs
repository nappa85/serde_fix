
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeliveryScheduleSettlTimeGrp {
	/// NoDeliveryScheduleSettlTimes
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41054")]
	pub delivery_schedule_settl_times: Option<fix_common::RepeatingValues<DeliveryScheduleSettlTime>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeliveryScheduleSettlTime {
	/// Required if NoDeliveryScheduleSettlTimes(41054) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41055")]
	pub delivery_schedule_settl_start: Option<String>,
	/// Required if NoDeliveryScheduleSettlTimes(41054) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41056")]
	pub delivery_schedule_settl_end: Option<String>,
	/// May be defaulted to market convention or bilaterally agreed if not specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41057")]
	pub delivery_schedule_settl_time_type: Option<DeliveryScheduleSettlTimeType>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum DeliveryScheduleSettlTimeType {
	/// Hour of the day (Applicable for electricity contracts. Time value is expressed as an integer hour of the day (1-24). The delivery
	/// start/end hour is specified as the end of the included hour. For example, a start hour of ":4" begins at 3 a.m.; an end hour
	/// of "20" ends at 8 p.m.; a start hour of "1" and an end hour of "24" indicates midnight to midnight delivery)
	#[serde(rename = "0")]
	HourOfTheDayTheDeliveryStartEndHourIsSpecifiedAsTheEndOfTheIncludedHourForExampleAStartHourOf4BeginsAt3AMAnEndHourOf20EndsAt8PMAStartHourOf1AndAnEndHourOf24IndicatesMidnightToMidnightDelivery,
	/// HH:MM time format (Applicable for gas contracts. Time is expressed using a 24- hour time format. For example, a time value
	/// of "13:30" is 1:30 p.m.)
	#[serde(rename = "1")]
	HhMmTimeFormat,
}

impl Default for DeliveryScheduleSettlTimeType {
	fn default() -> Self {
		DeliveryScheduleSettlTimeType::HourOfTheDayTheDeliveryStartEndHourIsSpecifiedAsTheEndOfTheIncludedHourForExampleAStartHourOf4BeginsAt3AMAnEndHourOf20EndsAt8PMAStartHourOf1AndAnEndHourOf24IndicatesMidnightToMidnightDelivery
	}
}
