
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProvisionOptionRelevantUnderlyingDateBusinessCenterGrp {
	/// NoProvisionOptionRelevantUnderlyingDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40956")]
	pub provision_option_relevant_underlying_date_business_centers: Option<crate::entities::RepeatingValues<ProvisionOptionRelevantUnderlyingDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProvisionOptionRelevantUnderlyingDateBusinessCenter {
	/// Required if NoProvisionOptionRelevantUnderlyingDateBusinessCenters(40956) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40157")]
	pub provision_option_relevant_underlying_date_business_center: Option<String>,
}
