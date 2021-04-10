
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingSettlMethodElectionDateBusinessCenterGrp {
	/// NoUnderlyingSettlMethodElectionDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43074")]
	pub underlying_settl_method_election_date_business_centers: Option<crate::entities::RepeatingValues<UnderlyingSettlMethodElectionDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingSettlMethodElectionDateBusinessCenter {
	/// Required if NoUnderlyingSettlMethodElectionDateBusinessCenters(43074) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43075")]
	pub underlying_settl_method_election_date_business_center: Option<String>,
}
