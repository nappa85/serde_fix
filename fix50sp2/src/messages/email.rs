
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Email {
	/// MsgType = C
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'C', ' '>,
	/// Unique identifier for the email message thread
	#[serde(rename = "164")]
	pub email_thread_id: String,
	/// EmailType
	#[serde(rename = "94")]
	pub email_type: EmailType,
	/// OrigTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42")]
	pub orig_time: Option<fix_common::UTCTimestamp>,
	/// Specifies the <a href="tag_147_Subject.html" target="bottom">Subject&nbsp;(147)</a> text
	#[serde(rename = "147")]
	pub subject: String,
	/// Must be set if <a href="tag_357_EncodedSubject.html" target="bottom">EncodedSubject&nbsp;(357)</a> field is specified and must immediately precede it.
	#[serde(rename = "356")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_147_Subject.html" target="bottom">Subject&nbsp;(147)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "357")]
	pub encoded_subject: Option<fix_common::EncodedText<357>>,
	/// Required if any RoutingType and RoutingIDs are specified. Indicates the number within repeating group.
	#[serde(flatten)]
	pub routing_grp: Option<super::super::routing_grp::RoutingGrp>,
	/// Specifies the number of repeating symbols (instruments) specified.
	#[serde(flatten)]
	pub instrmt_grp: Option<super::super::instrmt_grp::InstrmtGrp>,
	/// Number of underlyings.
	#[serde(flatten)]
	pub und_instrmt_grp: Option<super::super::und_instrmt_grp::UndInstrmtGrp>,
	/// InstrmtLegGrp
	#[serde(flatten)]
	pub instrmt_leg_grp: Option<super::super::instrmt_leg_grp::InstrmtLegGrp>,
	/// OrderID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "37")]
	pub order_id: Option<String>,
	/// ClOrdID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "11")]
	pub cl_ord_id: Option<String>,
	/// Specifies the number of repeating lines of text specified.
	#[serde(flatten)]
	pub lines_of_text_grp: super::super::lines_of_text_grp::LinesOfTextGrp,
	/// RawDataLength
	#[serde(rename = "95")]
	/// RawData
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "96")]
	pub raw_data: Option<fix_common::EncodedText<96>>,
	/// AttachmentGrp
	#[serde(flatten)]
	pub attachment_grp: Option<super::super::attachment_grp::AttachmentGrp>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum EmailType {
	/// New
	#[serde(rename = "0")]
	New,
	/// Reply
	#[serde(rename = "1")]
	Reply,
	/// Admin Reply
	#[serde(rename = "2")]
	AdminReply,
}

impl Default for EmailType {
	fn default() -> Self {
		EmailType::New
	}
}
