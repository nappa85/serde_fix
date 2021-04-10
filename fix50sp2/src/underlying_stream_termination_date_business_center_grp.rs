
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingStreamTerminationDateBusinessCenterGrp {
	/// NoUnderlyingStreamTerminationDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40976")]
	pub underlying_stream_termination_date_business_centers: Option<crate::entities::RepeatingValues<UnderlyingStreamTerminationDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingStreamTerminationDateBusinessCenter {
	/// Required if NoUnderlyingStreamTerminationDateBusinessCenters(40976) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40550")]
	pub underlying_stream_termination_date_business_center: Option<String>,
}
