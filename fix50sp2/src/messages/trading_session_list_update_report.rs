
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Trading {
	/// MsgType = BS
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// ApplicationSequenceControl
	#[serde(flatten)]
	pub application_sequence_control: Option<super::super::application_sequence_control::ApplicationSequenceControl>,
	/// Provided for a response to a specific Trading Session List Request message (snapshot).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "335")]
	pub trad_ses_req_id: Option<String>,
	/// TrdSessLstGrp
	#[serde(flatten)]
	pub trd_sess_lst_grp: super::super::trd_sess_lst_grp::TrdSessLstGrp,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}
