
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InstrmtLegIOIGrp {
	/// Required for multileg IOIs
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "555")]
	pub legs: Option<fix_common::RepeatingValues<Leg>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Leg {
	/// Required for multileg IOIs
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "682")]
	pub leg_ioi_qty: Option<LegIOIQty>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegIOIQty {
	/// Small
	#[serde(rename = "S")]
	Small,
	/// Medium
	#[serde(rename = "M")]
	Medium,
	/// Large
	#[serde(rename = "L")]
	Large,
	/// Undisclosed Quantity
	#[serde(rename = "U")]
	UndisclosedQuantity,
}

impl Default for LegIOIQty {
	fn default() -> Self {
		LegIOIQty::Small
	}
}
