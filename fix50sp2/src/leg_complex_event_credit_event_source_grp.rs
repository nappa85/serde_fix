
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegComplexEventCreditEventSourceGrp {
	/// NoLegComplexEventCreditEventSources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41398")]
	pub leg_complex_event_credit_event_sources: Option<fix_common::RepeatingValues<LegComplexEventCreditEventSource>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegComplexEventCreditEventSource {
	/// Required if NoLegComplexEventCreditEventSources(41398) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41399")]
	pub leg_complex_event_credit_event_source: Option<String>,
}
