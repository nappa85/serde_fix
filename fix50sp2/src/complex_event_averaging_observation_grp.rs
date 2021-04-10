
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ComplexEventAveragingObservationGrp {
	/// NoComplexEventAveragingObservations
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40994")]
	pub complex_event_averaging_observations: Option<crate::entities::RepeatingValues<ComplexEventAveragingObservation>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ComplexEventAveragingObservation {
	/// Required if NoComplexEventAveragingObservations(40994) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40995")]
	pub complex_event_averaging_observation_number: Option<i32>,
	/// ComplexEventAveragingWeight
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40996")]
	pub complex_event_averaging_weight: Option<f64>,
}
