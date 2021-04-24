
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
    #[serde(flatten)]
    pub price_movement_value_grp: Option<super::price_movement_value_grp::PriceMovementValueGrp>,
    #[serde(flatten)]
    pub clearing_account_Type_grp: Option<super::clearing_account_Type_grp::ClearingAccountTypeGrp>,
}
