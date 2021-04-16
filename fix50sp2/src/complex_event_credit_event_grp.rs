
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ComplexEventCreditEventGrp {
	/// NoComplexEventCreditEvents
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40997")]
	pub complex_event_credit_events: Option<fix_common::RepeatingValues<ComplexEventCreditEvent>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ComplexEventCreditEvent {
	/// Required if NoComplexEventCreditEvents(40996) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40998")]
	pub complex_event_credit_event_type: Option<String>,
	/// ComplexEventCreditEventValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40999")]
	pub complex_event_credit_event_value: Option<String>,
	/// ComplexEventCreditEventCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41000")]
	pub complex_event_credit_event_currency: Option<String>,
	/// Conditionally required when ComplexEventCreditEventUnit(41002) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41001")]
	pub complex_event_credit_event_period: Option<i32>,
	/// Conditionally required when ComplexEventCreditEventPeriod(41001) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41002")]
	pub complex_event_credit_event_unit: Option<ComplexEventCreditEventUnit>,
	/// ComplexEventCreditEventDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41003")]
	pub complex_event_credit_event_day_type: Option<i32>,
	/// ComplexEventCreditEventRateSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41004")]
	pub complex_event_credit_event_rate_source: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ComplexEventCreditEventUnit {
	/// Day
	#[serde(rename = "D")]
	Day,
	/// Week
	#[serde(rename = "Wk")]
	Week,
	/// Month
	#[serde(rename = "Mo")]
	Month,
	/// Year
	#[serde(rename = "Yr")]
	Year,
}

impl Default for ComplexEventCreditEventUnit {
	fn default() -> Self {
		ComplexEventCreditEventUnit::Day
	}
}
