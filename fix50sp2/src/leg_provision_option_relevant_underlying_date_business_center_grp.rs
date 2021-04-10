
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProvisionOptionRelevantUnderlyingDateBusinessCenterGrp {
	/// NoLegProvisionOptionRelevantUnderlyingDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40938")]
	pub leg_provision_option_relevant_underlying_date_business_centers: Option<crate::entities::RepeatingValues<LegProvisionOptionRelevantUnderlyingDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProvisionOptionRelevantUnderlyingDateBusinessCenter {
	/// Required if NoLegProvisionOptionRelevantUnderlyingDateBusinessCenters(40938) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40510")]
	pub leg_provision_option_relevant_underlying_date_business_center: Option<String>,
}
