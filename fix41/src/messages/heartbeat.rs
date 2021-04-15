
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Heartbeat {
	/// MsgType = 0
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'0'>,
	/// Required when the heartbeat is the result of a <a href="message_Test_Request_1.html" target="main">Test Request&nbsp;(1)</a> message.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "112")]
	pub test_req_id: Option<char>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}
