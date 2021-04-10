
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegReturnRateInformationSourceGrp {
	/// NoLegReturnRateInformationSources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42560")]
	pub leg_return_rate_information_sources: Option<fix_common::RepeatingValues<LegReturnRateInformationSource>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegReturnRateInformationSource {
	/// Required if NoLegReturnRateInformationSources(42560) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42561")]
	pub leg_return_rate_information_source: Option<i32>,
	/// LegReturnRateReferencePage
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42562")]
	pub leg_return_rate_reference_page: Option<String>,
	/// LegReturnRateReferencePageHeading
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42563")]
	pub leg_return_rate_reference_page_heading: Option<String>,
}
