
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StandardMessageTrailer {
	/// Required when trailer contains signature. Note: Not to be included within <a href="tag_91_SecureData.html" target="bottom">SecureData&nbsp;(91)</a> field
	#[serde(rename = "93")]
	/// Note: Not to be included within <a href="tag_91_SecureData.html" target="bottom">SecureData&nbsp;(91)</a> field
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "89")]
	pub signature: Option<fix_common::EncodedText<89>>,
	/// (Always unencrypted, always last field in message)
	#[serde(rename = "10")]
    #[serde(skip_serializing)]
	pub check_sum: String,
}
