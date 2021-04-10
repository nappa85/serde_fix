
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PosUndInstrmtGrp {
	/// NoUnderlyings
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "711")]
	pub underlyings: Option<crate::entities::RepeatingValues<Underlying>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Underlying {
	/// UnderlyingSettlPrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "732")]
	pub underlying_settl_price: Option<f64>,
	/// Values = Final, Theoretical
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "733")]
	pub underlying_settl_price_type: Option<UnderlyingSettlPriceType>,
	/// UnderlyingDeliveryAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1037")]
	pub underlying_delivery_amount: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingSettlPriceType {
	/// Final
	#[serde(rename = "1")]
	Final,
	/// Theoretical
	#[serde(rename = "2")]
	Theoretical,
}
