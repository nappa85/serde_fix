
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MandatoryClearingJurisdictionGrp {
	/// NoMandatoryClearingJurisdictions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41312")]
	pub mandatory_clearing_jurisdictions: Option<crate::entities::RepeatingValues<MandatoryClearingJurisdiction>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MandatoryClearingJurisdiction {
	/// Required if NoNoMandatoryClearingJurisdictions(41312) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41313")]
	pub mandatory_clearing_jurisdiction: Option<String>,
}
