
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DisplayInstruction {
	/// DisplayQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1138")]
	pub display_qty: Option<f64>,
	/// SecondaryDisplayQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1082")]
	pub secondary_display_qty: Option<f64>,
	/// DisplayWhen
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1083")]
	pub display_when: Option<DisplayWhen>,
	/// DisplayMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1084")]
	pub display_method: Option<DisplayMethod>,
	/// Required when <a href="tag_1084_DisplayMethod.html" target="bottom">DisplayMethod&nbsp;(1084)</a> = 3
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1085")]
	pub display_low_qty: Option<f64>,
	/// Required when <a href="tag_1084_DisplayMethod.html" target="bottom">DisplayMethod&nbsp;(1084)</a> = 3
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1086")]
	pub display_high_qty: Option<f64>,
	/// Can be used to specify larger increments than the standard increment provided by the market. Optionally used when <a href="tag_1084_DisplayMethod.html" target="bottom">DisplayMethod&nbsp;(1084)</a> = 3
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1087")]
	pub display_min_incr: Option<f64>,
	/// Required when <a href="tag_1084_DisplayMethod.html" target="bottom">DisplayMethod&nbsp;(1084)</a> = 2
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1088")]
	pub refresh_qty: Option<f64>,
	/// Only to be used in the ExecutionReport
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1608")]
	pub initial_display_qty: Option<f64>,
	/// CurrentDisplayPrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2828")]
	pub current_display_price: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DisplayWhen {
	/// Immediate (after each fill)
	#[serde(rename = "1")]
	Immediate,
	/// Exhaust (when DisplayQty = 0)
	#[serde(rename = "2")]
	Exhaust,
}

impl Default for DisplayWhen {
	fn default() -> Self {
		DisplayWhen::Immediate
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DisplayMethod {
	/// Initial (use original DisplayQty)
	#[serde(rename = "1")]
	Initial,
	/// New (use RefreshQty)
	#[serde(rename = "2")]
	New,
	/// Random (randomize value)
	#[serde(rename = "3")]
	Random,
	/// Undisclosed (invisible order)
	#[serde(rename = "4")]
	Undisclosed,
}

impl Default for DisplayMethod {
	fn default() -> Self {
		DisplayMethod::Initial
	}
}
