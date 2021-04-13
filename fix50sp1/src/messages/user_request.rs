
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
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
	/// RawDataLength
	#[serde(rename = "95")]
	/// Can be used to hand structures etc to other APIs etc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "96")]
	pub raw_data: Option<fix_common::EncodedText<96>>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
}

impl Default for UserRequestType {
	fn default() -> Self {
		UserRequestType::LogOnUser
	}
}
