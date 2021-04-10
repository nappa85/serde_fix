
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CompIDReqGrp {
	/// Used to restrict updates/request to a list of specific CompID/SubID/LocationID/DeskID combinations. If not present request
	/// applies to all applicable available counterparties. EG Unless one sell side broker was a customer of another you would not
	/// expect to see information about other brokers, similarly one fund manager etc.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "936")]
	pub comp_i_ds: Option<crate::entities::RepeatingValues<CompID>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CompID {
	/// Used to restrict updates/request to specific CompID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "930")]
	pub ref_comp_id: Option<String>,
	/// Used to restrict updates/request to specific SubID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "931")]
	pub ref_sub_id: Option<String>,
	/// Used to restrict updates/request to specific LocationID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "283")]
	pub location_id: Option<String>,
	/// Used to restrict updates/request to specific DeskID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "284")]
	pub desk_id: Option<String>,
}
