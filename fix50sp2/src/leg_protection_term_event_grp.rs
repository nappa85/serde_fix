
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProtectionTermEventGrp {
	/// NoLegProtectionTermEvents
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41625")]
	pub leg_protection_term_events: Option<crate::entities::RepeatingValues<LegProtectionTermEvent>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProtectionTermEvent {
	/// Required if NoLegProtectionTermEvents(41625) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41626")]
	pub leg_protection_term_event_type: Option<String>,
	/// LegProtectionTermEventValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41627")]
	pub leg_protection_term_event_value: Option<String>,
	/// LegProtectionTermEventCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41628")]
	pub leg_protection_term_event_currency: Option<String>,
	/// Conditionally required when LegProtectionTermEventUnit(41630).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41629")]
	pub leg_protection_term_event_period: Option<i32>,
	/// Conditionally required when LegProtectionTermEventPeriod(41629).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41630")]
	pub leg_protection_term_event_unit: Option<LegProtectionTermEventUnit>,
	/// LegProtectionTermEventDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41631")]
	pub leg_protection_term_event_day_type: Option<LegProtectionTermEventDayType>,
	/// LegProtectionTermEventRateSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41632")]
	pub leg_protection_term_event_rate_source: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegProtectionTermEventUnit {
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
pub enum LegProtectionTermEventDayType {
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
