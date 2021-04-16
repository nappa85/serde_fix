
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RateSource {
	/// Number of rate sources being specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1445")]
	pub rate_sources: Option<fix_common::RepeatingValues<RateSourceElement>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RateSourceElement {
	/// RateSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1446")]
	pub rate_source_item: Option<RateSourceItem>,
	/// RateSourceType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1447")]
	pub rate_source_type: Option<RateSourceType>,
	/// May be used when RateSource(1446) = 99 (Other).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1448")]
	pub reference_page: Option<String>,
	/// RateSourcePageHeading
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2412")]
	pub rate_source_page_heading: Option<String>,
	/// FXBenchmarkRateFix
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2796")]
	pub fx_benchmark_rate_fix: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum RateSourceItem {
	/// Bloomberg
	#[serde(rename = "0")]
	Bloomberg,
	/// Reuters
	#[serde(rename = "1")]
	Reuters,
	/// Telerate
	#[serde(rename = "2")]
	Telerate,
	/// Other
	#[serde(rename = "99")]
	Other,
}

impl Default for RateSourceItem {
	fn default() -> Self {
		RateSourceItem::Bloomberg
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum RateSourceType {
	/// Primary
	#[serde(rename = "0")]
	Primary,
	/// Secondary
	#[serde(rename = "1")]
	Secondary,
}

impl Default for RateSourceType {
	fn default() -> Self {
		RateSourceType::Primary
	}
}
