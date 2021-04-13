
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SettlRateDisruptionFallbackGrp {
	/// NoSettlRateFallbacks
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40085")]
	pub settl_rate_fallbacks: Option<fix_common::RepeatingValues<SettlRateFallback>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SettlRateFallback {
	/// Required if NoSettlRateFallbacks(40085) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40086")]
	pub settl_rate_postponement_maximum_days: Option<i32>,
	/// SettlRatePostponementSurvey
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40088")]
	pub settl_rate_postponement_survey: Option<fix_common::Boolean>,
	/// SettlRatePostponementCalculationAgent
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40089")]
	pub settl_rate_postponement_calculation_agent: Option<SettlRatePostponementCalculationAgent>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SettlRatePostponementCalculationAgent {
	/// Exercising party
	#[serde(rename = "0")]
	ExercisingParty,
	/// Non-exercising party
	#[serde(rename = "1")]
	NonExercisingParty,
	/// As specified in the master agreement
	#[serde(rename = "2")]
	AsSpecifiedInTheMasterAgreement,
	/// As specified in the standard terms supplement
	#[serde(rename = "3")]
	AsSpecifiedInTheStandardTermsSupplement,
}

impl Default for SettlRatePostponementCalculationAgent {
	fn default() -> Self {
		SettlRatePostponementCalculationAgent::ExercisingParty
	}
}
