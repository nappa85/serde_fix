
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamFormulaMathGrp {
	/// NoUnderlyingPaymentStreamFormulas
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42981")]
	pub underlying_payment_stream_formulas: Option<fix_common::RepeatingValues<UnderlyingPaymentStreamFormula>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamFormula {
	/// Required if NoUnderlyingPaymentStreamFormulas(42981) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42982")]
	pub underlying_payment_stream_formula: Option<String>,
	/// UnderlyingPaymentStreamFormulaDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42983")]
	pub underlying_payment_stream_formula_desc: Option<String>,
	/// Required if NoUnderlyingPaymentStreamFormulas(42981) &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43111")]
	pub underlying_payment_stream_formula_length: Option<usize>,
}
