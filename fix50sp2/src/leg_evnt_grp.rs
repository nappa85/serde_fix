
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegEvntGrp {
	/// NoLegEvents
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2059")]
	pub leg_events: Option<fix_common::RepeatingValues<LegEvent>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegEvent {
	/// Required if NoLegEvents(2059) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2060")]
	pub leg_event_type: Option<LegEventType>,
	/// Conditionally required when LegEventTime(2062) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2061")]
	pub leg_event_date: Option<fix_common::LocalMktDate>,
	/// LegEventTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2062")]
	pub leg_event_time: Option<fix_common::UTCTimestamp>,
	/// Conditionally required when LegEventTimePeriod(2064) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2063")]
	pub leg_event_time_unit: Option<LegEventTimeUnit>,
	/// Conditionally required when LegEventTimeUnit(2063) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2064")]
	pub leg_event_time_period: Option<i32>,
	/// LegEventMonthYear
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2341")]
	pub leg_event_month_year: Option<fix_common::MonthYear>,
	/// LegEventPx
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2065")]
	pub leg_event_px: Option<f64>,
	/// LegEventText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2066")]
	pub leg_event_text: Option<String>,
	/// Must be set if EncodedLegEventText(2075) field is specified and must immediately precede it.
	#[serde(rename = "2074")]
	/// Encoded (non-ASCII characters) representation of the LegEventText(2066) field in the encoded format specified via the MessageEncoding(347)
	/// field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "2075")]
	pub encoded_leg_event_text: Option<fix_common::EncodedText<2075>>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegEventType {
	/// Put
	#[serde(rename = "1")]
	Put,
	/// Call
	#[serde(rename = "2")]
	Call,
	/// Tender
	#[serde(rename = "3")]
	Tender,
	/// Sinking Fund Call
	#[serde(rename = "4")]
	SinkingFundCall,
	/// Activation
	#[serde(rename = "5")]
	Activation,
	/// Inactiviation
	#[serde(rename = "6")]
	Inactiviation,
	/// Last Eligible Trade Date
	#[serde(rename = "7")]
	LastEligibleTradeDate,
	/// Swap start date
	#[serde(rename = "8")]
	SwapStartDate,
	/// Swap end date
	#[serde(rename = "9")]
	SwapEndDate,
	/// Swap roll date
	#[serde(rename = "10")]
	SwapRollDate,
	/// Swap next start date
	#[serde(rename = "11")]
	SwapNextStartDate,
	/// Swap next roll date
	#[serde(rename = "12")]
	SwapNextRollDate,
	/// First delivery date
	#[serde(rename = "13")]
	FirstDeliveryDate,
	/// Last delivery date
	#[serde(rename = "14")]
	LastDeliveryDate,
	/// Initiatl inventory due date
	#[serde(rename = "15")]
	InitiatlInventoryDueDate,
	/// Final inventory due date
	#[serde(rename = "16")]
	FinalInventoryDueDate,
	/// First intent date
	#[serde(rename = "17")]
	FirstIntentDate,
	/// Last intent date
	#[serde(rename = "18")]
	LastIntentDate,
	/// Position removal date
	#[serde(rename = "19")]
	PositionRemovalDate,
	/// Other
	#[serde(rename = "99")]
	Other,
}

impl Default for LegEventType {
	fn default() -> Self {
		LegEventType::Put
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegEventTimeUnit {
	/// Hour
	#[serde(rename = "H")]
	Hour,
	/// Minute
	#[serde(rename = "Min")]
	Minute,
	/// Second
	#[serde(rename = "S")]
	Second,
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
	/// Quarter
	#[serde(rename = "Q")]
	Quarter,
}

impl Default for LegEventTimeUnit {
	fn default() -> Self {
		LegEventTimeUnit::Hour
	}
}
