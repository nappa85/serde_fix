
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegComplexEventAveragingObservationGrp {
	/// NoLegComplexEventAveragingObservations
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41363")]
	pub leg_complex_event_averaging_observations: Option<fix_common::RepeatingValues<LegComplexEventAveragingObservation>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegComplexEventAveragingObservation {
	/// Required if NoLegComplexEventAveragingObservations(41363) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41364")]
	pub leg_complex_event_averaging_observation_number: Option<i32>,
	/// LegComplexEventAveragingWeight
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41365")]
	pub leg_complex_event_averaging_weight: Option<f64>,
}
