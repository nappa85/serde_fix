
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingProvisionDateBusinessCenterGrp {
	/// NoUnderlyingProvisionDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42190")]
	pub underlying_provision_date_business_centers: Option<crate::entities::RepeatingValues<UnderlyingProvisionDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingProvisionDateBusinessCenter {
	/// Required if NoUnderlyingProvisionDateBusinessCenters(42190) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42191")]
	pub underlying_provision_date_business_center: Option<String>,
}
