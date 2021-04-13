
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingComplexEventCreditEventGrp {
	/// NoUnderlyingComplexEventCreditEvents
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41716")]
	pub underlying_complex_event_credit_events: Option<fix_common::RepeatingValues<UnderlyingComplexEventCreditEvent>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingComplexEventCreditEvent {
	/// Required if NoUnderlyingComplexEventCreditEvents(41716) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41717")]
	pub underlying_complex_event_credit_event_type: Option<String>,
	/// UnderlyingComplexEventCreditEventValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41718")]
	pub underlying_complex_event_credit_event_value: Option<String>,
	/// UnderlyingComplexEventCreditEventCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41719")]
	pub underlying_complex_event_credit_event_currency: Option<String>,
	/// Conditionally required when UnderlyingComplexEventCreditEventUnit(41721) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41720")]
	pub underlying_complex_event_credit_event_period: Option<i32>,
	/// Conditionally required when UnderlyingComplexEventCreditEventPeriod(41720) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41721")]
	pub underlying_complex_event_credit_event_unit: Option<UnderlyingComplexEventCreditEventUnit>,
	/// UnderlyingComplexEventCreditEventDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41722")]
	pub underlying_complex_event_credit_event_day_type: Option<UnderlyingComplexEventCreditEventDayType>,
	/// UnderlyingComplexEventCreditEventRateSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41723")]
	pub underlying_complex_event_credit_event_rate_source: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingComplexEventCreditEventUnit {
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

impl Default for UnderlyingComplexEventCreditEventUnit {
	fn default() -> Self {
		UnderlyingComplexEventCreditEventUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingComplexEventCreditEventDayType {
	/// Business
	#[serde(rename = "0")]
	Business,
	/// Calendar
	#[serde(rename = "1")]
	Calendar,
	/// Commodity business
	#[serde(rename = "2")]
	CommodityBusiness,
	/// Currency business
	#[serde(rename = "3")]
	CurrencyBusiness,
	/// Exchange business
	#[serde(rename = "4")]
	ExchangeBusiness,
	/// Scheduled trading day
	#[serde(rename = "5")]
	ScheduledTradingDay,
}

impl Default for UnderlyingComplexEventCreditEventDayType {
	fn default() -> Self {
		UnderlyingComplexEventCreditEventDayType::Business
	}
}
