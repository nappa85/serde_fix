
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingSettlRateFallbackRateSource {
	/// UnderlyingSettlRateFallbackRateSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40904")]
	pub underlying_settl_rate_fallback_rate_source: Option<i32>,
	/// Conditionally required when UnderlyingSettlRateFallbackRateSource(40904) = 3 (ISDA Settlement Rate Option) or 99 (Other).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40915")]
	pub underlying_settl_rate_fallback_reference_page: Option<String>,
}
