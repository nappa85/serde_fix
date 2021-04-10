
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingReturnRateFXConversionGrp {
	/// NoUnderlyingReturnRateFXConversions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43030")]
	pub underlying_return_rate_fx_conversions: Option<crate::entities::RepeatingValues<UnderlyingReturnRateFXConversion>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingReturnRateFXConversion {
	/// Required if NoUnderlyingReturnRateFXConversions(43030) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43031")]
	pub underlying_return_rate_fx_currency_symbol: Option<String>,
	/// Required if NoUnderlyingReturnRateFXConversions(43030) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43032")]
	pub underlying_return_rate_fx_rate: Option<f64>,
	/// UnderlyingReturnRateFXRateCalc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43033")]
	pub underlying_return_rate_fx_rate_calc: Option<char>,
}
