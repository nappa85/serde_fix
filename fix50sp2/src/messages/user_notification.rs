
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct User {
	/// MsgType = CB
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// List of users to which the notification is directed
	#[serde(flatten)]
	pub username_grp: Option<super::super::username_grp::UsernameGrp>,
	/// Reason for notification - when possible provide an explanation.
	#[serde(rename = "926")]
	pub user_status: UserStatus,
	/// Explanation for user notification.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if <a href="tag_355_EncodedText.html" target="bottom">EncodedText&nbsp;(355)</a> field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "354")]
	pub encoded_text_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the <a href="tag_58_Text.html" target="bottom">Text&nbsp;(58)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "355")]
	pub encoded_text: Option<String>,
	/// Indicates throttle limits
	#[serde(flatten)]
	pub throttle_params_grp: Option<super::super::throttle_params_grp::ThrottleParamsGrp>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UserStatus {
	/// Logged In
	#[serde(rename = "1")]
	LoggedIn,
	/// Not Logged In
	#[serde(rename = "2")]
	NotLoggedIn,
	/// User Not Recognised
	#[serde(rename = "3")]
	UserNotRecognised,
	/// Password Incorrect
	#[serde(rename = "4")]
	PasswordIncorrect,
	/// Password Changed
	#[serde(rename = "5")]
	PasswordChanged,
	/// Other
	#[serde(rename = "6")]
	Other,
	/// Forced user logout by Exchange
	#[serde(rename = "7")]
	ForcedUserLogoutByExchange,
	/// Session shutdown warning
	#[serde(rename = "8")]
	SessionShutdownWarning,
	/// ThrottleParametersChanged
	#[serde(rename = "9")]
	ThrottleParametersChanged,
}
