
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApplicationSequenceControl {
	/// Identifies the application with which a message is associated. Used only if application sequencing is in effect.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1180")]
	pub appl_id: Option<String>,
	/// Application sequence number assigned to the message by the application generating the message. Used only if application sequencing
	/// is in effect. Conditionally required if ApplID has been specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1181")]
	pub appl_seq_num: Option<usize>,
	/// The previous sequence number in the application sequence stream. Permits an application to publish messages with sequence
	/// gaps where it cannot be avoided. Used only if application sequencing is in effect. Conditionally required if ApplID has been
	/// specified
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1350")]
	pub appl_last_seq_num: Option<usize>,
	/// Used to indicate that a message is being sent in response to an Application Message Request. Used only if application sequencing
	/// is in effect. It is possible for both ApplResendFlag and PossDupFlag to be set on the same message if the Sender's cache size
	/// is greater than zero and the message is being resent due to a session level resend request.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1352")]
	pub appl_resend_flag: Option<fix_common::Boolean>,
}
