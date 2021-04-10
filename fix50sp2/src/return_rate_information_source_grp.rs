
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReturnRateInformationSourceGrp {
	/// NoReturnRateInformationSources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42761")]
	pub return_rate_information_sources: Option<crate::entities::RepeatingValues<ReturnRateInformationSource>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReturnRateInformationSource {
	/// Required if NoReturnRateInformationSources(42761) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42762")]
	pub return_rate_information_source: Option<i32>,
	/// ReturnRateReferencePage
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42763")]
	pub return_rate_reference_page: Option<String>,
	/// ReturnRateReferencePageHeading
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42764")]
	pub return_rate_reference_page_heading: Option<String>,
}
