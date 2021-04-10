
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegSettlMethodElectionDateBusinessCenterGrp {
	/// NoLegSettlMethodElectionDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42581")]
	pub leg_settl_method_election_date_business_centers: Option<crate::entities::RepeatingValues<LegSettlMethodElectionDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegSettlMethodElectionDateBusinessCenter {
	/// Required if NoLegSettlMethodElectionDateBusinessCenters(42581) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42582")]
	pub leg_settl_method_election_date_business_center: Option<String>,
}
