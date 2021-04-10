
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BidDescReqGrp {
	/// Used if BidType="Non Disclosed"
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "398")]
	pub bid_descriptors: Option<fix_common::RepeatingValues<BidDescriptor>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BidDescriptor {
	/// Required if <a href="tag_398_NoBidDescriptors.html" target="bottom">NoBidDescriptors&nbsp;(398)</a> &gt; 0. Must be first field in repeating group.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "399")]
	pub bid_descriptor_type: Option<BidDescriptorType>,
	/// BidDescriptor
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "400")]
	pub bid_descriptor: Option<String>,
	/// Refers to the <a href="tag_396_SideValue1.html" target="bottom">SideValue1&nbsp;(396)</a> or SideValue2. These are used as opposed to Buy or Sell so that the basket can be quoted either way as Buy or Sell.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "401")]
	pub side_value_ind: Option<SideValueInd>,
	/// Value between <a href="tag_402_LiquidityPctLow.html" target="bottom">LiquidityPctLow&nbsp;(402)</a> and <a href="tag_403_LiquidityPctHigh.html" target="bottom">LiquidityPctHigh&nbsp;(403)</a> in Currency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "404")]
	pub liquidity_value: Option<f64>,
	/// Number of Securites between <a href="tag_402_LiquidityPctLow.html" target="bottom">LiquidityPctLow&nbsp;(402)</a> and <a href="tag_403_LiquidityPctHigh.html" target="bottom">LiquidityPctHigh&nbsp;(403)</a> in Currency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "441")]
	pub liquidity_num_securities: Option<i32>,
	/// Liquidity indicator or lower limit if <a href="tag_441_LiquidityNumSecurities.html" target="bottom">LiquidityNumSecurities&nbsp;(441)</a> &gt; 1
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "402")]
	pub liquidity_pct_low: Option<f32>,
	/// Upper liquidity indicator if <a href="tag_441_LiquidityNumSecurities.html" target="bottom">LiquidityNumSecurities&nbsp;(441)</a> &gt; 1
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "403")]
	pub liquidity_pct_high: Option<f32>,
	/// Eg Used in EFP (Exchange For Physical) trades 12%
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "405")]
	pub efp_tracking_error: Option<f32>,
	/// Used in EFP trades
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "406")]
	pub fair_value: Option<f64>,
	/// Used in EFP trades
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "407")]
	pub outside_index_pct: Option<f32>,
	/// Used in EFP trades
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "408")]
	pub value_of_futures: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BidDescriptorType {
	/// Sector
	#[serde(rename = "1")]
	Sector,
	/// Country
	#[serde(rename = "2")]
	Country,
	/// Index
	#[serde(rename = "3")]
	Index,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SideValueInd {
	/// Side Value 1
	#[serde(rename = "1")]
	SideValue1,
	/// Side Value 2
	#[serde(rename = "2")]
	SideValue2,
}
