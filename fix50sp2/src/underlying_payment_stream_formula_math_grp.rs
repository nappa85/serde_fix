
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
	/// Required if NoUnderlyingPaymentStreamFormulas(42981) &gt; 0
	#[serde(rename = "43111")]
	/// Required if NoUnderlyingPaymentStreamFormulas(42981) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "42982")]
	pub underlying_payment_stream_formula: Option<fix_common::EncodedText<42982>>,
	/// UnderlyingPaymentStreamFormulaDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42983")]
	pub underlying_payment_stream_formula_desc: Option<String>,
}
