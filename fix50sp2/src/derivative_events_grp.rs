
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DerivativeEventsGrp {
	/// NoDerivativeEvents
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1286")]
	pub derivative_events: Option<fix_common::RepeatingValues<DerivativeEvent>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DerivativeEvent {
	/// Indicates type of event describing security.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1287")]
	pub derivative_event_type: Option<DerivativeEventType>,
	/// DerivativeEventDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1288")]
	pub derivative_event_date: Option<fix_common::LocalMktDate>,
	/// Specific time of event. To be used in combination with <a href="tag_1288_DerivativeEventDate.html" target="bottom">EventDate&nbsp;(1288)</a> .
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1289")]
	pub derivative_event_time: Option<fix_common::UTCTimestamp>,
	/// DerivativeEventPx
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1290")]
	pub derivative_event_px: Option<f64>,
	/// DerivativeEventText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1291")]
	pub derivative_event_text: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum DerivativeEventType {
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
	/// Swap Start Date
	#[serde(rename = "8")]
	SwapStartDate,
	/// Swap End Date
	#[serde(rename = "9")]
	SwapEndDate,
	/// Swap Roll Date
	#[serde(rename = "10")]
	SwapRollDate,
	/// Swap Next Start Date
	#[serde(rename = "11")]
	SwapNextStartDate,
	/// Swap Next Roll Date
	#[serde(rename = "12")]
	SwapNextRollDate,
	/// First Delivery Date
	#[serde(rename = "13")]
	FirstDeliveryDate,
	/// Last Delivery Date
	#[serde(rename = "14")]
	LastDeliveryDate,
	/// Initial Inventory Due Date
	#[serde(rename = "15")]
	InitialInventoryDueDate,
	/// Final Inventory Due Date
	#[serde(rename = "16")]
	FinalInventoryDueDate,
	/// First Intent Date
	#[serde(rename = "17")]
	FirstIntentDate,
	/// Last Intent Date
	#[serde(rename = "18")]
	LastIntentDate,
	/// Position Removal Date
	#[serde(rename = "19")]
	PositionRemovalDate,
	/// Other
	#[serde(rename = "99")]
	Other,
}

impl Default for DerivativeEventType {
	fn default() -> Self {
		DerivativeEventType::Put
	}
}
