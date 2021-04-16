
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OrderAggregationGrp {
	/// NoOrders
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "73")]
	pub orders: Option<fix_common::RepeatingValues<Order>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Order {
	/// Required if NoOrders(73) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "11")]
	pub cl_ord_id: Option<String>,
	/// OrderID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "37")]
	pub order_id: Option<String>,
	/// Required if NoOrders(73) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "38")]
	pub order_qty: Option<f64>,
	/// OrderAvgPx
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "799")]
	pub order_avg_px: Option<f64>,
	/// RelatedOrderTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2836")]
	pub related_order_time: Option<fix_common::UTCTimestamp>,
	/// May be used when aggregating orders that were originally submitted by different firms, e.g. due to a merger or acquisition.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2835")]
	pub order_origination_firm_id: Option<String>,
}
