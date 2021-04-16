
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InstrmtLegGrp {
	/// Number of legs
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "555")]
	pub legs: Option<fix_common::RepeatingValues<Leg>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Leg {
}
