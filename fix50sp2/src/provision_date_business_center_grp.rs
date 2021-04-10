
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProvisionDateBusinessCenterGrp {
	/// NoProvisionDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40957")]
	pub provision_date_business_centers: Option<crate::entities::RepeatingValues<ProvisionDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProvisionDateBusinessCenter {
	/// Required if NoProvisionDateBusinessCenters(40957) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40094")]
	pub provision_date_business_center: Option<String>,
}
