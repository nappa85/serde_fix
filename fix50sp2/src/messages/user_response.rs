
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UserResponse {
	/// MsgType = BF
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// UserRequestID
	#[serde(rename = "923")]
	pub user_request_id: String,
	/// Username
	#[serde(rename = "553")]
	pub username: String,
	/// UserStatus
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "926")]
	pub user_status: Option<UserStatus>,
	/// Reason a request was not carried out
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "927")]
	pub user_status_text: Option<String>,
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

impl Default for UserStatus {
	fn default() -> Self {
		UserStatus::LoggedIn
	}
}
