
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamFormulaImage {
	/// Conditionally required when LegPaymentStreamFormulaImage(42452) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42451")]
	pub leg_payment_stream_formula_image_length: Option<usize>,
	/// Conditionally required when LegPaymentStreamFormulaImageLength(42451) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42452")]
	pub leg_payment_stream_formula_image: Option<String>,
}
