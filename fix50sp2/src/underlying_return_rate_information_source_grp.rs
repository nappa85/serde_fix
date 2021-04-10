
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingReturnRateInformationSourceGrp {
	/// NoUnderlyingReturnRateInformationSources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43060")]
	pub underlying_return_rate_information_sources: Option<crate::entities::RepeatingValues<UnderlyingReturnRateInformationSource>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingReturnRateInformationSource {
	/// Required if NoUnderlyingReturnRateInformationSources(43060) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43061")]
	pub underlying_return_rate_information_source: Option<i32>,
	/// UnderlyingReturnRateReferencePage
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43062")]
	pub underlying_return_rate_reference_page: Option<String>,
	/// UnderlyingReturnRateReferencePageHeading
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43063")]
	pub underlying_return_rate_reference_page_heading: Option<String>,
}
