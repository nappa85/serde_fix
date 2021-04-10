
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SideCollateralReinvestmentGrp {
	/// NoSideCollateralReinvestments
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2864")]
	pub side_collateral_reinvestments: Option<fix_common::RepeatingValues<SideCollateralReinvestment>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SideCollateralReinvestment {
	/// Required if NoSideCollateralReinvestments(2864) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2867")]
	pub side_collateral_reinvestment_type: Option<i32>,
	/// SideCollateralReinvestmentAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2865")]
	pub side_collateral_reinvestment_amount: Option<f64>,
	/// SideCollateralReinvestmentCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2866")]
	pub side_collateral_reinvestment_currency: Option<String>,
}
