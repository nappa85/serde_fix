
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProtectionTermEventGrp {
	/// NoProtectionTermEvents
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40191")]
	pub protection_term_events: Option<fix_common::RepeatingValues<ProtectionTermEvent>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProtectionTermEvent {
	/// Required if NoProtectionTermEvents(40191) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40192")]
	pub protection_term_event_type: Option<String>,
	/// ProtectionTermEventValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40193")]
	pub protection_term_event_value: Option<String>,
	/// ProtectionTermEventCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40194")]
	pub protection_term_event_currency: Option<String>,
	/// Conditionally required when ProtectionTermEventUnit(40196) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40195")]
	pub protection_term_event_period: Option<i32>,
	/// Conditionally required when ProtectionTermEventPeriod(40195) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40196")]
	pub protection_term_event_unit: Option<ProtectionTermEventUnit>,
	/// ProtectionTermEventDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40197")]
	pub protection_term_event_day_type: Option<ProtectionTermEventDayType>,
	/// ProtectionTermEventRateSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40198")]
	pub protection_term_event_rate_source: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ProtectionTermEventUnit {
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

impl Default for ProtectionTermEventUnit {
	fn default() -> Self {
		ProtectionTermEventUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ProtectionTermEventDayType {
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

impl Default for ProtectionTermEventDayType {
	fn default() -> Self {
		ProtectionTermEventDayType::Business
	}
}
