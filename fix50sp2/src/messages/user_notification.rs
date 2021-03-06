
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UserNotification {
	/// MsgType = CB
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'C', 'B'>,
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
	#[serde(rename = "354")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_58_Text.html" target="bottom">Text&nbsp;(58)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
	/// Indicates throttle limits
	#[serde(flatten)]
	pub throttle_params_grp: Option<super::super::throttle_params_grp::ThrottleParamsGrp>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

impl Default for UserStatus {
	fn default() -> Self {
		UserStatus::LoggedIn
	}
}
