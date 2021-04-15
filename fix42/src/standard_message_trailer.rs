
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StandardMessageTrailer {
	/// Required when trailer contains signature. Note: Not to be included within <a href="tag_91_SecureData.html" target="bottom">SecureData&nbsp;(91)</a> field
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "93")]
	pub signature_length: Option<i32>,
	/// Note: Not to be included within <a href="tag_91_SecureData.html" target="bottom">SecureData&nbsp;(91)</a> field
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "89")]
	pub signature: Option<String>,
	/// (Always unencrypted, always last field in message)
	#[serde(rename = "10")]
	pub check_sum: String,
}
