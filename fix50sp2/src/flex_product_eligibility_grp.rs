
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FlexProductEligibilityGrp {
	/// NoFlexProductEligibilities
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2560")]
	pub flex_product_eligibilities: Option<crate::entities::RepeatingValues<FlexProductEligibilitie>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FlexProductEligibilitie {
	/// Required if NoFlexProductEligibilities(2560) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1242")]
	pub flex_product_eligibility_indicator: Option<crate::entities::Boolean>,
	/// Required if NoFlexProductEligibilities(2560) &gt; 0. Used to specify a product suite related to an eligibility indicator.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2561")]
	pub flex_product_eligibility_complex: Option<String>,
}
