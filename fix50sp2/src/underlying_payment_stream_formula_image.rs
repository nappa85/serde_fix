
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamFormulaImage {
	/// Conditionally required when UnderlyingPaymentStreamFormulaImage(42948) is specified.
	#[serde(rename = "42947")]
	/// Conditionally required when UnderlyingPaymentStreamFormulaImageLength(42947) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "42948")]
	pub underlying_payment_stream_formula_image: Option<fix_common::EncodedText<42948>>,
}
