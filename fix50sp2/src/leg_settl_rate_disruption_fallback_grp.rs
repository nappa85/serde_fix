
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegSettlRateDisruptionFallbackGrp {
	/// NoLegSettlRateFallbacks
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40902")]
	pub leg_settl_rate_fallbacks: Option<crate::entities::RepeatingValues<LegSettlRateFallback>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegSettlRateFallback {
	/// Required if NoLegSettlRateFallbacks(40902) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40903")]
	pub leg_settl_rate_postponement_maximum_days: Option<i32>,
	/// LegSettlRatePostponementSurvey
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40905")]
	pub leg_settl_rate_postponement_survey: Option<crate::entities::Boolean>,
	/// LegSettlRatePostponementCalculationAgent
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40906")]
	pub leg_settl_rate_postponement_calculation_agent: Option<i32>,
}
