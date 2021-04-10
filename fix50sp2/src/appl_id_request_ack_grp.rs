
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApplIDRequestAckGrp {
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
	/// RefApplReqID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1433")]
	pub ref_appl_req_id: Option<String>,
	/// ApplBegSeqNum
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1182")]
	pub appl_beg_seq_num: Option<usize>,
	/// ApplEndSeqNum
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1183")]
	pub appl_end_seq_num: Option<usize>,
	/// RefApplLastSeqNum
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1357")]
	pub ref_appl_last_seq_num: Option<usize>,
	/// ApplResponseError
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1354")]
	pub appl_response_error: Option<ApplResponseError>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ApplResponseError {
	/// Application does not exist
	#[serde(rename = "0")]
	ApplicationDoesNotExist,
	/// Messages requested are not available
	#[serde(rename = "1")]
	MessagesRequestedAreNotAvailable,
	/// User not authorized for application
	#[serde(rename = "2")]
	UserNotAuthorizedForApplication,
}
