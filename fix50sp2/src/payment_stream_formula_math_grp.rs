
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
	#[serde(rename = "43109")]
	/// Required if NoPaymentStreamFormulas(42683) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "42684")]
	pub payment_stream_formula: Option<fix_common::EncodedText<42684>>,
	/// PaymentStreamFormulaDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42685")]
	pub payment_stream_formula_desc: Option<String>,
}
