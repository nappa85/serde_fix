
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamFormulaImage {
	/// Conditionally required when LegPaymentStreamFormulaImage(42452) is specified.
	#[serde(rename = "42451")]
	/// Conditionally required when LegPaymentStreamFormulaImageLength(42451) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "42452")]
	pub leg_payment_stream_formula_image: Option<fix_common::EncodedText<42452>>,
}
