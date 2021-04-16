
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct News {
	/// MsgType = B
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'B', ' '>,
	/// OrigTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42")]
	pub orig_time: Option<fix_common::UTCTimestamp>,
	/// Urgency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "61")]
	pub urgency: Option<Urgency>,
	/// Specifies the headline text
	#[serde(rename = "148")]
	pub headline: String,
	/// Must be set if <a href="tag_359_EncodedHeadline.html" target="bottom">EncodedHeadline&nbsp;(359)</a> field is specified and must immediately precede it.
	#[serde(rename = "358")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_148_Headline.html" target="bottom">Headline&nbsp;(148)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "359")]
	pub encoded_headline: Option<fix_common::EncodedText<359>>,
	/// Required if any <a href="tag_216_RoutingType.html" target="bottom">RoutingType&nbsp;(216)</a> and RoutingIDs are specified. Indicates the number within repeating group.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "215")]
	pub routing_i_ds: Option<fix_common::RepeatingValues<RoutingID>>,
	/// Specifies the number of repeating symbols (instruments) specified
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "146")]
	pub related_sym: Option<fix_common::RepeatingValues<super::super::instrument::Instrument>>,
	/// Number of legs. Identifies a Multi-leg Execution if present and non-zero.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "555")]
	pub legs: Option<fix_common::RepeatingValues<super::super::instrument_leg::InstrumentLeg>>,
	/// Number of underlyings
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "711")]
	pub underlyings: Option<fix_common::RepeatingValues<super::super::underlying_instrument::UnderlyingInstrument>>,
	/// Specifies the number of repeating lines of text specified
	#[serde(rename = "33")]
	pub lines_of_text: fix_common::RepeatingValues<LinesOfTex>,
	/// A URL (Uniform Resource Locator) link to additional information (i.e. http://en.wikipedia.org/wiki/Uniform_Resource_Locator)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "149")]
	pub url_link: Option<String>,
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
pub struct LinesOfTex {
	/// Repeating field, number of instances defined in <a href="tag_33_LinesOfText.html" target="bottom">LinesOfText&nbsp;(33)</a>
	#[serde(rename = "58")]
	pub text: String,
	/// Must be set if <a href="tag_355_EncodedText.html" target="bottom">EncodedText&nbsp;(355)</a> field is specified and must immediately precede it.
	#[serde(rename = "354")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_58_Text.html" target="bottom">Text&nbsp;(58)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Urgency {
	/// Normal
	#[serde(rename = "0")]
	Normal,
	/// Flash
	#[serde(rename = "1")]
	Flash,
	/// Background
	#[serde(rename = "2")]
	Background,
}

impl Default for Urgency {
	fn default() -> Self {
		Urgency::Normal
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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
