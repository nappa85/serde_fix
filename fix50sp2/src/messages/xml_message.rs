
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct XmlMessage {
	/// MsgType = n
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'n', ' '>,
	/// AttachmentGrp
	#[serde(flatten)]
	pub attachment_grp: Option<super::super::attachment_grp::AttachmentGrp>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}
