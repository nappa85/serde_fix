
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamFormula {
	/// UnderlyingPaymentStreamFormulaCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42978")]
	pub underlying_payment_stream_formula_currency: Option<String>,
	/// UnderlyingPaymentStreamFormulaCurrencyDeterminationMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42979")]
	pub underlying_payment_stream_formula_currency_determination_method: Option<String>,
	/// UnderlyingPaymentStreamFormulaReferenceAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42980")]
	pub underlying_payment_stream_formula_reference_amount: Option<i32>,
	/// UnderlyingPaymentStreamFormulaMathGrp
	#[serde(flatten)]
	pub underlying_payment_stream_formula_math_grp: Option<super::underlying_payment_stream_formula_math_grp::UnderlyingPaymentStreamFormulaMathGrp>,
	/// UnderlyingPaymentStreamFormulaImage
	#[serde(flatten)]
	pub underlying_payment_stream_formula_image: Option<super::underlying_payment_stream_formula_image::UnderlyingPaymentStreamFormulaImage>,
}
