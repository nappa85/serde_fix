
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApplIDReportGrp {
	/// Number of applications
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1351")]
	pub appl_i_ds: Option<fix_common::RepeatingValues<ApplID>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApplID {
	/// RefApplID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1355")]
	pub ref_appl_id: Option<String>,
	/// ApplNewSeqNum
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1399")]
	pub appl_new_seq_num: Option<usize>,
	/// RefApplLastSeqNum
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1357")]
	pub ref_appl_last_seq_num: Option<usize>,
}
