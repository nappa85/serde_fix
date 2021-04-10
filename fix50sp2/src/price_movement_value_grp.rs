
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PriceMovementValueGrp {
	/// NoPriceMovementValues
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1920")]
	pub price_movement_values: Option<crate::entities::RepeatingValues<PriceMovementValue>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PriceMovementValue {
	/// Value at the price movement point. Required if NoPriceMovementValues(1919) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1921")]
	pub price_movement_value: Option<f64>,
	/// Price movement point up or down
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1922")]
	pub price_movement_point: Option<i32>,
	/// Format of the PriceMoveValue(percent or amount are initial options)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1923")]
	pub price_movement_type: Option<PriceMovementType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PriceMovementType {
	/// Amount
	#[serde(rename = "0")]
	Amount,
	/// Percentage
	#[serde(rename = "1")]
	Percentage,
}
