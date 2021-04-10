
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamTerminationDateBusinessCenterGrp {
	/// NoStreamTerminationDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40961")]
	pub stream_termination_date_business_centers: Option<crate::entities::RepeatingValues<StreamTerminationDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamTerminationDateBusinessCenter {
	/// Required if NoStreamTerminationDateBusinessCenters(40961) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40067")]
	pub stream_termination_date_business_center: Option<String>,
}
