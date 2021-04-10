
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingComplexEventAveragingObservationGrp {
	/// NoUnderlyingComplexEventAveragingObservations
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41713")]
	pub underlying_complex_event_averaging_observations: Option<crate::entities::RepeatingValues<UnderlyingComplexEventAveragingObservation>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingComplexEventAveragingObservation {
	/// Required if NoUnderlyingComplexEventAveragingObservations(41713) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41714")]
	pub underlying_complex_event_averaging_observation_number: Option<i32>,
	/// UnderlyingComplexEventAveragingWeight
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41715")]
	pub underlying_complex_event_averaging_weight: Option<f64>,
}
