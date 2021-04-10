
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingRateSpreadSchedule {
	/// UnderlyingRateSpreadInitialValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43004")]
	pub underlying_rate_spread_initial_value: Option<f64>,
	/// UnderlyingRateSpreadStepGrp
	#[serde(flatten)]
	pub underlying_rate_spread_step_grp: Option<super::underlying_rate_spread_step_grp::UnderlyingRateSpreadStepGrp>,
}
