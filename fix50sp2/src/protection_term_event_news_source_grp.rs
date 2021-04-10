
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProtectionTermEventNewsSourceGrp {
	/// NoProtectionTermEventNewsSources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40951")]
	pub protection_term_event_news_sources: Option<crate::entities::RepeatingValues<ProtectionTermEventNewsSource>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProtectionTermEventNewsSource {
	/// Required if NoProtectionTermEventNewsSources(40951) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40189")]
	pub protection_term_event_news_source: Option<String>,
}
