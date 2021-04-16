
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UsernameGrp {
	/// Number of usernames
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "809")]
	pub usernames: Option<fix_common::RepeatingValues<Username>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Username {
	/// Recipient of the notification
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "553")]
	pub username: Option<String>,
}
