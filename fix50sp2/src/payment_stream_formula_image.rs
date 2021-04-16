
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamFormulaImage {
	/// Conditionally required when PaymentStreamFormulaImage(42653) is specified.
	#[serde(rename = "42652")]
	/// Conditionally required when PaymentStreamFormulaImageLength(42652) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "42653")]
	pub payment_stream_formula_image: Option<fix_common::EncodedText<42653>>,
}
