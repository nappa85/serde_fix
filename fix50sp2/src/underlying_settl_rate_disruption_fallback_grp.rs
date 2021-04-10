
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingSettlRateDisruptionFallbackGrp {
	/// NoUnderlyingSettlRateFallbacks
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40659")]
	pub underlying_settl_rate_fallbacks: Option<crate::entities::RepeatingValues<UnderlyingSettlRateFallback>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingSettlRateFallback {
	/// Required if NoUnderlyingSettlRateFallbacks(40659) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40660")]
	pub underlying_settl_rate_postponement_maximum_days: Option<i32>,
	/// UnderlyingSettlRatePostponementSurvey
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40662")]
	pub underlying_settl_rate_postponement_survey: Option<crate::entities::Boolean>,
	/// UnderlyingSettlRatePostponementCalculationAgent
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40663")]
	pub underlying_settl_rate_postponement_calculation_agent: Option<i32>,
}
