
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RelativeValueGrp {
	/// NoRelativeValues
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2529")]
	pub relative_values: Option<fix_common::RepeatingValues<RelativeValue>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RelativeValue {
	/// Required if NoRelativeValues(2529) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2530")]
	pub relative_value_type: Option<RelativeValueType>,
	/// Required if NoRelativeValues(2529) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2531")]
	pub relative_value: Option<f64>,
	/// RelativeValueSide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2532")]
	pub relative_value_side: Option<RelativeValueSide>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RelativeValueType {
	/// Asset Swap Spread
	#[serde(rename = "1")]
	AssetSwapSpread,
	/// Overnight Indexed Swap Spread
	#[serde(rename = "2")]
	OvernightIndexedSwapSpread,
	/// Zero Volatility Spread
	#[serde(rename = "3")]
	ZeroVolatilitySpread,
	/// Discount Margin
	#[serde(rename = "4")]
	DiscountMargin,
	/// Interpolated Spread
	#[serde(rename = "5")]
	InterpolatedSpread,
	/// Option Adjusted Spread
	#[serde(rename = "6")]
	OptionAdjustedSpread,
	/// G-Spread
	#[serde(rename = "7")]
	GSpread,
	/// CDS Basis
	#[serde(rename = "8")]
	CdsBasis,
	/// CDS Interpolated Basis
	#[serde(rename = "9")]
	CdsInterpolatedBasis,
}

impl Default for RelativeValueType {
	fn default() -> Self {
		RelativeValueType::AssetSwapSpread
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RelativeValueSide {
	/// Bid
	#[serde(rename = "1")]
	Bid,
	/// Mid
	#[serde(rename = "2")]
	Mid,
	/// Offer
	#[serde(rename = "3")]
	Offer,
}

impl Default for RelativeValueSide {
	fn default() -> Self {
		RelativeValueSide::Bid
	}
}
