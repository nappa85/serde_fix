
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CompIDStatGrp {
	/// Specifies the number of repeating CompId's
	#[serde(rename = "936")]
	pub comp_i_ds: fix_common::RepeatingValues<CompID>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CompID {
	/// CompID that status is being report for. Required if NoCompIDs &gt; 0
	#[serde(rename = "930")]
	pub ref_comp_id: String,
	/// SubID that status is being report for.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "931")]
	pub ref_sub_id: Option<String>,
	/// LocationID that status is being report for.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "283")]
	pub location_id: Option<String>,
	/// DeskID that status is being report for.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "284")]
	pub desk_id: Option<String>,
	/// StatusValue
	#[serde(rename = "928")]
	pub status_value: StatusValue,
	/// Additional Information, i.e."National Holiday"
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "929")]
	pub status_text: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum StatusValue {
	/// Connected
	#[serde(rename = "1")]
	Connected,
	/// Not connected - down expected up
	#[serde(rename = "2")]
	NotConnectedDownExpectedUp,
	/// Not connected - down expected down
	#[serde(rename = "3")]
	NotConnectedDownExpectedDown,
	/// In Process
	#[serde(rename = "4")]
	InProcess,
}

impl Default for StatusValue {
	fn default() -> Self {
		StatusValue::Connected
	}
}
