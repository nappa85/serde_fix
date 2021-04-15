
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PartyEntitlementGrp {
	/// NoPartyEntitlements
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1772")]
	pub party_entitlements: Option<fix_common::RepeatingValues<PartyEntitlement>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PartyEntitlement {
	/// EntitlementStatus
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1883")]
	pub entitlement_status: Option<EntitlementStatus>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum EntitlementStatus {
	/// Accepted
	#[serde(rename = "0")]
	N0,
	/// Accepted with changes
	#[serde(rename = "1")]
	N1,
	/// Rejected
	#[serde(rename = "2")]
	N2,
	/// Pending
	#[serde(rename = "3")]
	N3,
	/// Requested
	#[serde(rename = "4")]
	N4,
	/// Deferred (Entitlement definition request is being postponed or delayed)
	#[serde(rename = "5")]
	N5,
}

impl Default for EntitlementStatus {
	fn default() -> Self {
		EntitlementStatus::N0
	}
}
