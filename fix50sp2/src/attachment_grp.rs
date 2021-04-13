
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AttachmentGrp {
	/// NoAttachments
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2104")]
	pub attachments: Option<fix_common::RepeatingValues<Attachment>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Attachment {
	/// Required if <a href="tag_2104_NoAttachments.html" target="bottom">NoAttachements(2104)&nbsp;(2104)</a> &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2105")]
	pub attachment_name: Option<String>,
	/// Required if <a href="tag_2112_EncodedAttachment.html" target="bottom">EncodedAttachment(2112)&nbsp;(2112)</a> is present
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2106")]
	pub attachment_media_type: Option<String>,
	/// AttachmentClassification
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2107")]
	pub attachment_classification: Option<String>,
	/// Either <a href="tag_2108_AttachmentExternalURL.html" target="bottom">AttachementExternalURL(2108)&nbsp;(2108)</a> or <a href="tag_2112_EncodedAttachment.html" target="bottom">EncodedAttachment(2112)&nbsp;(2112)</a> must be specified if <a href="tag_2104_NoAttachments.html" target="bottom">NoAttachements(2104)&nbsp;(2104)</a> &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2108")]
	pub attachment_external_url: Option<String>,
	/// Required if <a href="tag_2112_EncodedAttachment.html" target="bottom">EncodedAttachment(2112)&nbsp;(2112)</a> is present
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2109")]
	pub attachment_encoding_type: Option<AttachmentEncodingType>,
	/// UnencodedAttachmentLen
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2110")]
	pub unencoded_attachment_len: Option<i32>,
	/// Must be set if <a href="tag_2112_EncodedAttachment.html" target="bottom">EncodedAttachement(2112)&nbsp;(2112)</a> is specified and must immediately precede it.
	#[serde(rename = "2111")]
	/// Either <a href="tag_2108_AttachmentExternalURL.html" target="bottom">AttachementExternalURL(2108)&nbsp;(2108)</a> or <a href="tag_2112_EncodedAttachment.html" target="bottom">EncodedAttachment(2112)&nbsp;(2112)</a> must be specified if <a href="tag_2104_NoAttachments.html" target="bottom">NoAttachements(2104)&nbsp;(2104)</a> &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "2112")]
	pub encoded_attachment: Option<fix_common::EncodedText<2112>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AttachmentEncodingType {
	/// Base64 encoding
	#[serde(rename = "0")]
	Base64Encoding,
	/// Unencoded binary content
	#[serde(rename = "1")]
	UnencodedBinaryContent,
}

impl Default for AttachmentEncodingType {
	fn default() -> Self {
		AttachmentEncodingType::Base64Encoding
	}
}
