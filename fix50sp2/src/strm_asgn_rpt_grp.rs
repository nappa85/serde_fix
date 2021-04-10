
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StrmAsgnRptGrp {
	/// Stream Assignment Requests.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1499")]
	pub asgn_reqs: Option<crate::entities::RepeatingValues<AsgnReq>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AsgnReq {
}
