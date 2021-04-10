
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SettlMethodElectionDateBusinessCenterGrp {
	/// NoSettlMethodElectionDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42775")]
	pub settl_method_election_date_business_centers: Option<crate::entities::RepeatingValues<SettlMethodElectionDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SettlMethodElectionDateBusinessCenter {
	/// Required if NoSettlMethodElectionDateBusinessCenters(42775) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42776")]
	pub settl_method_election_date_business_center: Option<String>,
}
