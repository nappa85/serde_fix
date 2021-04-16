
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProvisionDateBusinessCenterGrp {
	/// NoLegProvisionDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40939")]
	pub leg_provision_date_business_centers: Option<fix_common::RepeatingValues<LegProvisionDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProvisionDateBusinessCenter {
	/// Required if NoLegProvisionDateBusinessCenters(40939) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40452")]
	pub leg_provision_date_business_center: Option<String>,
}
