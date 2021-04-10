
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamFormulaMathGrp {
	/// NoLegPaymentStreamFormulas
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42485")]
	pub leg_payment_stream_formulas: Option<crate::entities::RepeatingValues<LegPaymentStreamFormula>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamFormula {
	/// Required if NoLegPaymentStreamFormulas(42485) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42486")]
	pub leg_payment_stream_formula: Option<String>,
	/// LegPaymentStreamFormulaDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42487")]
	pub leg_payment_stream_formula_desc: Option<String>,
	/// Required if NoLegPaymentStreamFormulas(42485) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43110")]
	pub leg_payment_stream_formula_length: Option<usize>,
}
