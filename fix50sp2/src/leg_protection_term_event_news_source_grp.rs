
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProtectionTermEventNewsSourceGrp {
	/// NoLegProtectionTermEventNewsSources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41614")]
	pub leg_protection_term_event_news_sources: Option<fix_common::RepeatingValues<LegProtectionTermEventNewsSource>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProtectionTermEventNewsSource {
	/// Required if NoLegProtectionTermEventNewsSources(41614) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41615")]
	pub leg_protection_term_event_news_source: Option<String>,
}
