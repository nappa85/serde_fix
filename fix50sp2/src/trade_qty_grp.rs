
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TradeQtyGrp {
	/// NoTradeQtys
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1841")]
	pub trade_qtys: Option<crate::entities::RepeatingValues<TradeQty>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TradeQty {
	/// Required if NoTradeQtys &gt; 0. Must be first field in repeating group.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1842")]
	pub trade_qty_type: Option<TradeQtyType>,
	/// TradeQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1843")]
	pub trade_qty: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TradeQtyType {
	/// Cleared quantity
	#[serde(rename = "0")]
	ClearedQuantity,
	/// Long side claimed quantity
	#[serde(rename = "1")]
	LongSideClaimedQuantity,
	/// Short side claimed quantity
	#[serde(rename = "2")]
	ShortSideClaimedQuantity,
	/// Long side rejected quantity
	#[serde(rename = "3")]
	LongSideRejectedQuantity,
	/// Short side rejected quantity
	#[serde(rename = "4")]
	ShortSideRejectedQuantity,
	/// Pending quantity
	#[serde(rename = "5")]
	PendingQuantity,
	/// Transaction quantity
	#[serde(rename = "6")]
	TransactionQuantity,
	/// Remaining trade quantity
	#[serde(rename = "7")]
	RemainingTradeQuantity,
	/// Previous remaining trade quantity
	#[serde(rename = "8")]
	PreviousRemainingTradeQuantity,
}
