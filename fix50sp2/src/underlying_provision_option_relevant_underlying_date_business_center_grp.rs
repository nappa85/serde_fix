
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingProvisionOptionRelevantUnderlyingDateBusinessCenterGrp {
	/// NoUnderlyingProvisionOptionRelevantUnderlyingDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42188")]
	pub underlying_provision_option_relevant_underlying_date_business_centers: Option<crate::entities::RepeatingValues<UnderlyingProvisionOptionRelevantUnderlyingDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingProvisionOptionRelevantUnderlyingDateBusinessCenter {
	/// Required if NoUnderlyingProvisionOptionRelevantUnderlyingDateBusinessCenters(42188) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42189")]
	pub underlying_provision_option_relevant_underlying_date_business_center: Option<String>,
}
