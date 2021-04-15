
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingEvntGrp {
	/// NoUnderlyingEvents
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1981")]
	pub underlying_events: Option<fix_common::RepeatingValues<UnderlyingEvent>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingEvent {
	/// Required if NoUnderlyingEvents(1982) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1982")]
	pub underlying_event_type: Option<UnderlyingEventType>,
	/// Conditionally required when UnderlyingEventTime(1984) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1983")]
	pub underlying_event_date: Option<fix_common::LocalMktDate>,
	/// UnderlyingEventTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1984")]
	pub underlying_event_time: Option<fix_common::UTCTimestamp>,
	/// Conditionally required when UnderlyingEventTimePeriod(1986) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1985")]
	pub underlying_event_time_unit: Option<String>,
	/// Conditionally required when UnderlyingEventTimeUnit(1985) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1986")]
	pub underlying_event_time_period: Option<i32>,
	/// UnderlyingEventMonthYear
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2342")]
	pub underlying_event_month_year: Option<fix_common::MonthYear>,
	/// UnderlyingEventPx
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1987")]
	pub underlying_event_px: Option<f64>,
	/// UnderlyingEventText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2071")]
	pub underlying_event_text: Option<String>,
	/// Must be set if EncodedUnderlyingEventText(2073) field is specified and must immediately precede it.
	#[serde(rename = "2072")]
	/// Encoded (non-ASCII characters) representation of the UnderlyingEventText(2071) field in the encoded format specified via the
	/// MessageEncoding(347) field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "2073")]
	pub encoded_underlying_event_text: Option<fix_common::EncodedText<2073>>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingEventType {
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
	/// Minimum notice
	#[serde(rename = "20")]
	MinimumNotice,
	/// Delivery start time
	#[serde(rename = "21")]
	DeliveryStartTime,
	/// Delivery end time
	#[serde(rename = "22")]
	DeliveryEndTime,
	/// First notice date
	#[serde(rename = "23")]
	FirstNoticeDate,
	/// Last notice date
	#[serde(rename = "24")]
	LastNoticeDate,
	/// First exercise date
	#[serde(rename = "25")]
	FirstExerciseDate,
	/// Redemption date
	#[serde(rename = "26")]
	RedemptionDate,
	/// Other
	#[serde(rename = "99")]
	Other,
}

impl Default for UnderlyingEventType {
	fn default() -> Self {
		UnderlyingEventType::Put
	}
}
