
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProvisionOptionExpirationDateBusinessCenterGrp {
	/// NoProvisionOptionExpirationDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40955")]
	pub provision_option_expiration_date_business_centers: Option<crate::entities::RepeatingValues<ProvisionOptionExpirationDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProvisionOptionExpirationDateBusinessCenter {
	/// Required if NoProvisionOptionExpirationDateBusinessCenters(40955) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40147")]
	pub provision_option_expiration_date_business_center: Option<String>,
}
