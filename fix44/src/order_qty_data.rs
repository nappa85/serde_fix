
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OrderQtyData {
	/// One of <a href="tag_152_CashOrderQty.html" target="bottom">CashOrderQty&nbsp;(152)</a> , <a href="tag_38_OrderQty.html" target="bottom">OrderQty&nbsp;(38)</a> , or (for CIV only) <a href="tag_516_OrderPercent.html" target="bottom">OrderPercent&nbsp;(516)</a> is required. Note that unless otherwise specified, only one of <a href="tag_152_CashOrderQty.html" target="bottom">CashOrderQty&nbsp;(152)</a> , <a href="tag_38_OrderQty.html" target="bottom">OrderQty&nbsp;(38)</a> , or <a href="tag_516_OrderPercent.html" target="bottom">OrderPercent&nbsp;(516)</a> should be specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "38")]
	pub order_qty: Option<f64>,
	/// One of <a href="tag_152_CashOrderQty.html" target="bottom">CashOrderQty&nbsp;(152)</a> , <a href="tag_38_OrderQty.html" target="bottom">OrderQty&nbsp;(38)</a> , or (for CIV only) <a href="tag_516_OrderPercent.html" target="bottom">OrderPercent&nbsp;(516)</a> is required. Note that unless otherwise specified, only one of <a href="tag_152_CashOrderQty.html" target="bottom">CashOrderQty&nbsp;(152)</a> , <a href="tag_38_OrderQty.html" target="bottom">OrderQty&nbsp;(38)</a> , or <a href="tag_516_OrderPercent.html" target="bottom">OrderPercent&nbsp;(516)</a> should be specified. Specifies the approximate "monetary quantity" for the order. Broker is responsible for converting and
	/// calculating <a href="tag_38_OrderQty.html" target="bottom">OrderQty&nbsp;(38)</a> in tradeable units (e.g. shares) for subsequent messages.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "152")]
	pub cash_order_qty: Option<f64>,
	/// For CIV - Optional. One of <a href="tag_152_CashOrderQty.html" target="bottom">CashOrderQty&nbsp;(152)</a> , <a href="tag_38_OrderQty.html" target="bottom">OrderQty&nbsp;(38)</a> or (for CIV only) <a href="tag_516_OrderPercent.html" target="bottom">OrderPercent&nbsp;(516)</a> is required. Note that unless otherwise specified, only one of <a href="tag_152_CashOrderQty.html" target="bottom">CashOrderQty&nbsp;(152)</a> , <a href="tag_38_OrderQty.html" target="bottom">OrderQty&nbsp;(38)</a> , or <a href="tag_516_OrderPercent.html" target="bottom">OrderPercent&nbsp;(516)</a> should be specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "516")]
	pub order_percent: Option<f32>,
	/// For CIV - Optional
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "468")]
	pub rounding_direction: Option<RoundingDirection>,
	/// For CIV - Optional
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "469")]
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

impl Default for RoundingDirection {
	fn default() -> Self {
		RoundingDirection::RoundToNearest
	}
}
