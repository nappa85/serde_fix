
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EvntGrp {
	/// NoEvents
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "864")]
	pub events: Option<fix_common::RepeatingValues<Event>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Event {
	/// Required if NoEvents(864) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "865")]
	pub event_type: Option<EventType>,
	/// Conditionally required when EventTime(1145) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "866")]
	pub event_date: Option<fix_common::LocalMktDate>,
	/// <p></p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1145")]
	pub event_time: Option<fix_common::UTCTimestamp>,
	/// EventPx
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "867")]
	pub event_px: Option<f64>,
	/// EventText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "868")]
	pub event_text: Option<String>,
	/// Conditionally required when EventTimePeriod(1826) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1827")]
	pub event_time_unit: Option<EventTimeUnit>,
	/// Conditionally required when EventTimeUnit(1827) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1826")]
	pub event_time_period: Option<i32>,
	/// EventMonthYear
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2340")]
	pub event_month_year: Option<fix_common::MonthYear>,
	/// Must be set if EncodedEventText(1579) field is specified and must immediately precede it.
	#[serde(rename = "1578")]
	/// Encoded (non-ASCII characters) representation of the EventText(868) field in the encoded format specified via the MessageEncoding(347)
	/// field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "1579")]
	pub encoded_event_text: Option<fix_common::EncodedText<1579>>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum EventType {
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
	/// Minimum notice
	#[serde(rename = "20")]
	MinimumNotice,
	/// Delivery start time
	#[serde(rename = "21")]
	DeliveryStartTime,
	/// Delivery end time
	#[serde(rename = "22")]
	DeliveryEndTime,
	/// First notice date (The first day that a notice of intent to deliver a commodity can be made by a clearing house to a buyer
	/// in fulfillment of a given month's futures contract)
	#[serde(rename = "23")]
	FirstNoticeDate,
	/// Last notice date (The last day on which a clearing house may inform an investor that a seller intends to make delivery of
	/// a commodity that the investor previously bought in a futures contract. The date is governed by the rules of different exchanges
	/// and clearing houses, but may also be stated in the futures contract itself)
	#[serde(rename = "24")]
	LastNoticeDate,
	/// First exercise date
	#[serde(rename = "25")]
	FirstExerciseDate,
	/// Redemption date
	#[serde(rename = "26")]
	RedemptionDate,
	/// Trade continuation effective date
	#[serde(rename = "27")]
	TradeContinuationEffectiveDate,
}

impl Default for EventType {
	fn default() -> Self {
		EventType::Put
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum EventTimeUnit {
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

impl Default for EventTimeUnit {
	fn default() -> Self {
		EventTimeUnit::Hour
	}
}
