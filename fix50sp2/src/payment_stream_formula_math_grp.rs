
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamFormulaMathGrp {
	/// NoPaymentStreamFormulas
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42683")]
	pub payment_stream_formulas: Option<fix_common::RepeatingValues<PaymentStreamFormula>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamFormula {
	/// Required if NoPaymentStreamFormulas(42683) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42684")]
	pub payment_stream_formula: Option<String>,
	/// PaymentStreamFormulaDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42685")]
	pub payment_stream_formula_desc: Option<String>,
	/// Required if NoPaymentStreamFormulas(42683) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43109")]
	pub payment_stream_formula_length: Option<usize>,
}
