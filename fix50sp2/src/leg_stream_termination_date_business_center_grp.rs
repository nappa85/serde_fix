
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegStreamTerminationDateBusinessCenterGrp {
	/// NoLegStreamTerminationDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40943")]
	pub leg_stream_termination_date_business_centers: Option<fix_common::RepeatingValues<LegStreamTerminationDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegStreamTerminationDateBusinessCenter {
	/// Required if NoLegStreamTerminationDateBusinessCenters(40943) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40259")]
	pub leg_stream_termination_date_business_center: Option<String>,
}
