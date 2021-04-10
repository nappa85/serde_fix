
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NewsRefGrp {
	/// Number of news item references
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1475")]
	pub news_ref_i_ds: Option<crate::entities::RepeatingValues<NewsRefID>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NewsRefID {
	/// Required if <a href="tag_1475_NoNewsRefIDs.html" target="bottom">NoNewsRefIDs(1475)&nbsp;(1475)</a> &gt; 0. News item being referenced.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1476")]
	pub news_ref_id: Option<String>,
	/// Type of reference.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1477")]
	pub news_ref_type: Option<NewsRefType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NewsRefType {
	/// Replacement
	#[serde(rename = "0")]
	Replacement,
	/// Other Language
	#[serde(rename = "1")]
	OtherLanguage,
	/// Complimentary
	#[serde(rename = "2")]
	Complimentary,
	/// Withdrawal (Withdrawal of the referenced news item, e.g. to correct an error)
	#[serde(rename = "3")]
	Withdrawal,
}
