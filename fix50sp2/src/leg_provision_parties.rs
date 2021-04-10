
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProvisionParties {
	/// NoLegProvisionPartyIDs
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40533")]
	pub leg_provision_party_i_ds: Option<fix_common::RepeatingValues<LegProvisionPartyID>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProvisionPartyID {
	/// Required if NoLegProvisionPartyIDs(40533) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40534")]
	pub leg_provision_party_id: Option<String>,
	/// Required if NoLegProvisionPartyIDs(40533) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40535")]
	pub leg_provision_party_id_source: Option<char>,
	/// Required if NoLegProvisionPartyIDs(40533) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40536")]
	pub leg_provision_party_role: Option<i32>,
	/// LegProvisionPartyRoleQualifier
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2380")]
	pub leg_provision_party_role_qualifier: Option<i32>,
}
