
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamFormulaImage {
	/// Conditionally required when UnderlyingPaymentStreamFormulaImage(42948) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42947")]
	pub underlying_payment_stream_formula_image_length: Option<usize>,
	/// Conditionally required when UnderlyingPaymentStreamFormulaImageLength(42947) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42948")]
	pub underlying_payment_stream_formula_image: Option<String>,
}
