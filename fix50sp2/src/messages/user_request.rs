
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct User {
	/// MsgType = BE
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// UserRequestID
	#[serde(rename = "923")]
	pub user_request_id: String,
	/// UserRequestType
	#[serde(rename = "924")]
	pub user_request_type: UserRequestType,
	/// Username
	#[serde(rename = "553")]
	pub username: String,
	/// Password
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "554")]
	pub password: Option<String>,
	/// NewPassword
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "925")]
	pub new_password: Option<String>,
	/// EncryptedPasswordMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1400")]
	pub encrypted_password_method: Option<i32>,
	/// EncryptedPasswordLen
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1401")]
	pub encrypted_password_len: Option<usize>,
	/// EncryptedPassword
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1402")]
	pub encrypted_password: Option<String>,
	/// EncryptedNewPasswordLen
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1403")]
	pub encrypted_new_password_len: Option<usize>,
	/// EncryptedNewPassword
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1404")]
	pub encrypted_new_password: Option<String>,
	/// RawDataLength
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "95")]
	pub raw_data_length: Option<usize>,
	/// Can be used to hand structures etc to other APIs etc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "96")]
	pub raw_data: Option<String>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum UserRequestType {
	/// Log On User
	#[serde(rename = "1")]
	LogOnUser,
	/// Log Off User
	#[serde(rename = "2")]
	LogOffUser,
	/// Change Password For User
	#[serde(rename = "3")]
	ChangePasswordForUser,
	/// Request Individual User Status
	#[serde(rename = "4")]
	RequestIndividualUserStatus,
	/// RequestThrottleLimit
	#[serde(rename = "5")]
	RequestThrottleLimit,
}
