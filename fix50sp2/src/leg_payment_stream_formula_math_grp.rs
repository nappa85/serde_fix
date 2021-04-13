
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamFormulaMathGrp {
	/// NoLegPaymentStreamFormulas
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42485")]
	pub leg_payment_stream_formulas: Option<fix_common::RepeatingValues<LegPaymentStreamFormula>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamFormula {
	/// Required if NoLegPaymentStreamFormulas(42485) &gt; 0.
	#[serde(rename = "43110")]
	/// Required if NoLegPaymentStreamFormulas(42485) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "42486")]
	pub leg_payment_stream_formula: Option<fix_common::EncodedText<42486>>,
	/// LegPaymentStreamFormulaDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42487")]
	pub leg_payment_stream_formula_desc: Option<String>,
}
