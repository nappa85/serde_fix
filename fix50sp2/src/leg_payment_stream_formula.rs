
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamFormula {
	/// LegPaymentStreamFormulaCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42482")]
	pub leg_payment_stream_formula_currency: Option<String>,
	/// LegPaymentStreamFormulaCurrencyDeterminationMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42483")]
	pub leg_payment_stream_formula_currency_determination_method: Option<String>,
	/// LegPaymentStreamFormulaReferenceAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42484")]
	pub leg_payment_stream_formula_reference_amount: Option<i32>,
	/// LegPaymentStreamFormulaMathGrp
	#[serde(flatten)]
	pub leg_payment_stream_formula_math_grp: Option<super::leg_payment_stream_formula_math_grp::LegPaymentStreamFormulaMathGrp>,
	/// LegPaymentStreamFormulaImage
	#[serde(flatten)]
	pub leg_payment_stream_formula_image: Option<super::leg_payment_stream_formula_image::LegPaymentStreamFormulaImage>,
}
