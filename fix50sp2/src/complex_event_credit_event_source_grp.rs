
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ComplexEventCreditEventSourceGrp {
	/// NoComplexEventCreditEventSources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41029")]
	pub complex_event_credit_event_sources: Option<fix_common::RepeatingValues<ComplexEventCreditEventSource>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ComplexEventCreditEventSource {
	/// Required if NoComplexEventCreditEventSources(41029) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41030")]
	pub complex_event_credit_event_source: Option<String>,
}
