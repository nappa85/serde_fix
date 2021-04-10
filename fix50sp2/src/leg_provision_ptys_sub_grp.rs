
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProvisionPtysSubGrp {
	/// NoLegProvisionPartySubIDs
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40537")]
	pub leg_provision_party_sub_i_ds: Option<crate::entities::RepeatingValues<LegProvisionPartySubID>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProvisionPartySubID {
	/// Required if NoLegProvisionPartySubIDs(40537) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40538")]
	pub leg_provision_party_sub_id: Option<String>,
	/// Required if NoLegProvisionPartySubIDs(40537) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40539")]
	pub leg_provision_party_sub_id_type: Option<i32>,
}
