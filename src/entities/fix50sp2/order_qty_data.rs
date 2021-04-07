
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OrderQtyData {
    /// One of CashOrderQty, OrderQty, or (for CIV only) <a href="tag_516_OrderPercent_.html">OrderPercent&nbsp;(516)</a> is required. Note that unless otherwise specified, only one of CashOrderQty, OrderQty, or <a href="tag_516_OrderPercent_.html">OrderPercent&nbsp;(516)</a> should be specified.
    #[serde(rename = "38")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
    pub order_qty: Option<f64>,
    /// One of CashOrderQty, OrderQty, or (for CIV only) <a href="tag_516_OrderPercent_.html">OrderPercent&nbsp;(516)</a> is required. Note that unless otherwise specified, only one of CashOrderQty, OrderQty, or <a href="tag_516_OrderPercent_.html">OrderPercent&nbsp;(516)</a> should be specified. Specifies the approximate "monetary quantity" for the order. Broker is responsible for converting and
    /// calculating <a href="tag_38_OrderQty_.html">OrderQty&nbsp;(38)</a> in tradeable units (e.g. shares) for subsequent messages.
    #[serde(rename = "152")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
    pub cash_order_qty: Option<f64>,
    /// For CIV - Optional. One of CashOrderQty, <a href="tag_38_OrderQty_.html">OrderQty&nbsp;(38)</a> or (for CIV only) <a href="tag_516_OrderPercent_.html">OrderPercent&nbsp;(516)</a> is required. Note that unless otherwise specified, only one of CashOrderQty, OrderQty, or <a href="tag_516_OrderPercent_.html">OrderPercent&nbsp;(516)</a> should be specified.
    #[serde(rename = "516")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
    pub order_percent: Option<f32>,
    /// For CIV - Optional
    #[serde(rename = "468")]
	#[serde(skip_serializing_if = "Option::is_none")]
    pub rounding_direction: Option<RoundingDirection>,
    /// For CIV - Optional
    #[serde(rename = "469")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
    pub rounding_modulus: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RoundingDirection {
    /// Round to nearest
    #[serde(rename = "0")]
    RoundToNearest,
    /// Round down
    #[serde(rename = "1")]
    RoundDown,
    /// Round up
    #[serde(rename = "2")]
    RoundUp,
}
