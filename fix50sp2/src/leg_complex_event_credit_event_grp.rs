
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegComplexEventCreditEventGrp {
	/// NoLegComplexEventCreditEvents
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41366")]
	pub leg_complex_event_credit_events: Option<fix_common::RepeatingValues<LegComplexEventCreditEvent>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegComplexEventCreditEvent {
	/// Required if NoLegComplexEventCreditEvents(41366) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41367")]
	pub leg_complex_event_credit_event_type: Option<String>,
	/// LegComplexEventCreditEventValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41368")]
	pub leg_complex_event_credit_event_value: Option<String>,
	/// LegComplexEventCreditEventCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41369")]
	pub leg_complex_event_credit_event_currency: Option<String>,
	/// Conditionally required when LegComplexEventCreditEventUnit(41371) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41370")]
	pub leg_complex_event_credit_event_period: Option<i32>,
	/// Conditionally required when LegComplexEventCreditEventPeriod(41370) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41371")]
	pub leg_complex_event_credit_event_unit: Option<LegComplexEventCreditEventUnit>,
	/// LegComplexEventCreditEventDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41372")]
	pub leg_complex_event_credit_event_day_type: Option<LegComplexEventCreditEventDayType>,
	/// LegComplexEventCreditEventRateSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41373")]
	pub leg_complex_event_credit_event_rate_source: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegComplexEventCreditEventUnit {
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegComplexEventCreditEventDayType {
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
