
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Email {
	/// MsgType = C
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
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
	/// Required if any <a href="tag_216_RoutingType.html" target="bottom">RoutingType&nbsp;(216)</a> and RoutingIDs are specified. Indicates the number within repeating group.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "215")]
	pub routing_i_ds: Option<fix_common::RepeatingValues<RoutingID>>,
	/// Specifies the number of repeating symbols (instruments) specified
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "146")]
	pub related_sym: Option<fix_common::RepeatingValues<RelatedSy>>,
	/// Number of underlyings
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "711")]
	pub underlyings: Option<fix_common::RepeatingValues<Underlying>>,
	/// Number of legs Identifies a Multi-leg Execution if present and non-zero.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "555")]
	pub legs: Option<fix_common::RepeatingValues<Leg>>,
	/// OrderID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "37")]
	pub order_id: Option<String>,
	/// ClOrdID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "11")]
	pub cl_ord_id: Option<String>,
	/// Specifies the number of repeating lines of text specified
	#[serde(rename = "33")]
	pub lines_of_text: fix_common::RepeatingValues<LinesOfTex>,
	/// RawDataLength
	#[serde(rename = "95")]
	/// RawData
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "96")]
	pub raw_data: Option<fix_common::EncodedText<96>>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RoutingID {
	/// Indicates type of RoutingID. Required if <a href="tag_215_NoRoutingIDs.html" target="bottom">NoRoutingIDs&nbsp;(215)</a> is &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "216")]
	pub routing_type: Option<RoutingType>,
	/// Identifies routing destination. Required if <a href="tag_215_NoRoutingIDs.html" target="bottom">NoRoutingIDs&nbsp;(215)</a> is &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "217")]
	pub routing_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RelatedSy {
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Underlying {
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Leg {
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LinesOfTex {
	/// Repeating field, number of instances defined in LinesOfText
	#[serde(rename = "58")]
	pub text: String,
	/// Must be set if <a href="tag_355_EncodedText.html" target="bottom">EncodedText&nbsp;(355)</a> field is specified and must immediately precede it.
	#[serde(rename = "354")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_58_Text.html" target="bottom">Text&nbsp;(58)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RoutingType {
	/// Target Firm
	#[serde(rename = "1")]
	TargetFirm,
	/// Target List
	#[serde(rename = "2")]
	TargetList,
	/// Block Firm
	#[serde(rename = "3")]
	BlockFirm,
	/// Block List
	#[serde(rename = "4")]
	BlockList,
}

impl Default for RoutingType {
	fn default() -> Self {
		RoutingType::TargetFirm
	}
}
