
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingProvisionOptionExpirationDateBusinessCenterGrp {
	/// NoUnderlyingProvisionOptionExpirationDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42186")]
	pub underlying_provision_option_expiration_date_business_centers: Option<crate::entities::RepeatingValues<UnderlyingProvisionOptionExpirationDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingProvisionOptionExpirationDateBusinessCenter {
	/// Required if NoUnderlyingProvisionOptionExpirationDateBusinessCenters(42186) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42187")]
	pub underlying_provision_option_expiration_date_business_center: Option<String>,
}
