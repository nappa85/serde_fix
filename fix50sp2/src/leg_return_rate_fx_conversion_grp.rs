
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegReturnRateFXConversionGrp {
	/// NoLegReturnRateFXConversions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42530")]
	pub leg_return_rate_fx_conversions: Option<fix_common::RepeatingValues<LegReturnRateFXConversion>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegReturnRateFXConversion {
	/// Required if NoLegReturnRateFXConversions(42530) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42531")]
	pub leg_return_rate_fx_currency_symbol: Option<String>,
	/// Required if NoLegReturnRateFXConversions(42530) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42532")]
	pub leg_return_rate_fx_rate: Option<f64>,
	/// LegReturnRateFXRateCalc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42533")]
	pub leg_return_rate_fx_rate_calc: Option<char>,
}
