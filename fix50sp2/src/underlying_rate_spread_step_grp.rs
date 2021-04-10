
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingRateSpreadStepGrp {
	/// NoUnderlyingRateSpreadSteps
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43005")]
	pub underlying_rate_spread_steps: Option<crate::entities::RepeatingValues<UnderlyingRateSpreadStep>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingRateSpreadStep {
	/// Required if NoUnderlyingRateSpreadSteps(43005) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43006")]
	pub underlying_rate_spread_step_date: Option<crate::entities::LocalMktDate>,
	/// Required if NoUnderlyingRateSpreadSteps(43005) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43007")]
	pub underlying_rate_spread_step_value: Option<f64>,
}
