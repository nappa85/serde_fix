
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PriceMovementGrp {
	/// NoPriceMovements
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1919")]
	pub price_movements: Option<fix_common::RepeatingValues<PriceMovement>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PriceMovement {
}
