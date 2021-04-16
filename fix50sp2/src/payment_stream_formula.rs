
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamFormula {
	/// PaymentStreamFormulaCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42686")]
	pub payment_stream_formula_currency: Option<String>,
	/// PaymentStreamFormulaCurrencyDeterminationMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42687")]
	pub payment_stream_formula_currency_determination_method: Option<String>,
	/// PaymentStreamFormulaReferenceAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42688")]
	pub payment_stream_formula_reference_amount: Option<i32>,
	/// PaymentStreamFormulaMathGrp
	#[serde(flatten)]
	pub payment_stream_formula_math_grp: Option<super::payment_stream_formula_math_grp::PaymentStreamFormulaMathGrp>,
	/// PaymentStreamFormulaImage
	#[serde(flatten)]
	pub payment_stream_formula_image: Option<super::payment_stream_formula_image::PaymentStreamFormulaImage>,
}
