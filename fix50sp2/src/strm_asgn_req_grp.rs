
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StrmAsgnReqGrp {
	/// Stream Assignment Requests.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1499")]
	pub asgn_reqs: Option<fix_common::RepeatingValues<AsgnReq>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AsgnReq {
    #[serde(flatten)]
    pub parties: Option<super::parties::Parties>,
    #[serde(flatten)]
    pub strm_asgn_req_instrmt_grp: Option<super::strm_asgn_req_instrmt_grp::StrmAsgnReqInstrmtGrp>,
}
