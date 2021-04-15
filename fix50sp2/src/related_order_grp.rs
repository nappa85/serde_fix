
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RelatedOrderGrp {
	/// NoOrders
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "73")]
	pub orders: Option<fix_common::RepeatingValues<Order>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Order {
	/// Required if NoOrders(73) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2887")]
	pub related_order_id: Option<String>,
	/// The same value must be used for all orders having the same OrderRelationship(2890) value.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2888")]
	pub related_order_id_source: Option<RelatedOrderIDSource>,
	/// RelatedOrderTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2836")]
	pub related_order_time: Option<fix_common::UTCTimestamp>,
	/// RelatedOrderQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2889")]
	pub related_order_qty: Option<f64>,
	/// May be used to explicitly express the type of relationship or to provide orders having different relationships.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2890")]
	pub order_relationship: Option<OrderRelationship>,
	/// May be used when aggregating orders that were originally submitted by different firms, e.g. due to a merger or acquisition.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2835")]
	pub order_origination_firm_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum RelatedOrderIDSource {
	/// Non-FIX Source
	#[serde(rename = "0")]
	NonFixSource,
	/// Order identifier
	#[serde(rename = "1")]
	OrderIdentifier,
	/// Client order identifier
	#[serde(rename = "2")]
	ClientOrderIdentifier,
	/// Secondary order identifier
	#[serde(rename = "3")]
	SecondaryOrderIdentifier,
	/// Secondary client order identifier
	#[serde(rename = "4")]
	SecondaryClientOrderIdentifier,
}

impl Default for RelatedOrderIDSource {
	fn default() -> Self {
		RelatedOrderIDSource::NonFixSource
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum OrderRelationship {
	/// Not specified
	#[serde(rename = "0")]
	NotSpecified,
	/// Order aggregation
	#[serde(rename = "1")]
	OrderAggregation,
	/// Order split
	#[serde(rename = "2")]
	OrderSplit,
}

impl Default for OrderRelationship {
	fn default() -> Self {
		OrderRelationship::NotSpecified
	}
}
