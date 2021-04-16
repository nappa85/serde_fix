
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingProtectionTermEventNewsSourceGrp {
	/// NoUnderlyingProtectionTermEventNewsSources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42090")]
	pub underlying_protection_term_event_news_sources: Option<fix_common::RepeatingValues<UnderlyingProtectionTermEventNewsSource>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingProtectionTermEventNewsSource {
	/// Required if NoUnderlyingProtectionTermEventNewsSources(42090) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42091")]
	pub underlying_protection_term_event_news_source: Option<String>,
}
