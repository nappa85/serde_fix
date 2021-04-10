
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApplIDRequestGrp {
	/// Specifies number of application id occurrences
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1351")]
	pub appl_i_ds: Option<crate::entities::RepeatingValues<ApplID>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApplID {
	/// RefApplID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1355")]
	pub ref_appl_id: Option<String>,
	/// RefApplReqID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1433")]
	pub ref_appl_req_id: Option<String>,
	/// Message sequence number of first message in range to be resent
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1182")]
	pub appl_beg_seq_num: Option<usize>,
	/// Message sequence number of last message in range to be resent. If request is for a single message ApplBeginSeqNo = ApplEndSeqNo.
	/// If request is for all messages subsequent to a particular message, ApplEndSeqNo = "0" (representing infinity).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1183")]
	pub appl_end_seq_num: Option<usize>,
}
