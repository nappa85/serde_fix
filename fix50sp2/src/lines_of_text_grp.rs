
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LinesOfTextGrp {
	/// Specifies the number of repeating lines of text specified
	#[serde(rename = "33")]
	pub lines_of_text: fix_common::RepeatingValues<LinesOfTex>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LinesOfTex {
	/// Repeating field, number of instances defined in LinesOfText
	#[serde(rename = "58")]
	pub text: String,
	/// Must be set if EncodedText field is specified and must immediately precede it.
	#[serde(rename = "354")]
	/// Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
}
