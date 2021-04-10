
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamFormulaImage {
	/// Conditionally required when PaymentStreamFormulaImage(42653) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42652")]
	pub payment_stream_formula_image_length: Option<usize>,
	/// Conditionally required when PaymentStreamFormulaImageLength(42652) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42653")]
	pub payment_stream_formula_image: Option<String>,
}
