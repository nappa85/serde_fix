
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingComplexEventCreditEventSourceGrp {
	/// NoUnderlyingComplexEventCreditEventSources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41748")]
	pub underlying_complex_event_credit_event_sources: Option<fix_common::RepeatingValues<UnderlyingComplexEventCreditEventSource>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingComplexEventCreditEventSource {
	/// Required if NoUnderlyingCreditEventCreditEventSources(41748) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41749")]
	pub underlying_complex_event_credit_event_source: Option<String>,
}
