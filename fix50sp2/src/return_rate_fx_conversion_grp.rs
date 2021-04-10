
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReturnRateFXConversionGrp {
	/// NoReturnRateFXConversions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42731")]
	pub return_rate_fx_conversions: Option<crate::entities::RepeatingValues<ReturnRateFXConversion>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReturnRateFXConversion {
	/// Required if NoReturnRateFXConversions(42731) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42732")]
	pub return_rate_fx_currency_symbol: Option<String>,
	/// Required if NoReturnRateFXConversions(42731) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42733")]
	pub return_rate_fx_rate: Option<f64>,
	/// ReturnRateFXRateCalc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42734")]
	pub return_rate_fx_rate_calc: Option<char>,
}
