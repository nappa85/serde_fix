
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AttachmentKeywordGrp {
	/// NoAttachmentKeywords
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2113")]
	pub attachment_keywords: Option<crate::entities::RepeatingValues<AttachmentKeyword>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AttachmentKeyword {
	/// Required if <a href="tag_2113_NoAttachmentKeywords.html" target="bottom">NoAttachmentKeywords(2113)&nbsp;(2113)</a> &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2114")]
	pub attachment_keyword: Option<String>,
}
