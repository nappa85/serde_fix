
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SideCollateralAmountGrp {
	/// NoSideCollateralAmounts
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2691")]
	pub side_collateral_amounts: Option<crate::entities::RepeatingValues<SideCollateralAmount>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SideCollateralAmount {
	/// Required if NoSideCollateralAmounts(2691) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2702")]
	pub side_current_collateral_amount: Option<f64>,
	/// Can be used to specify the currency of SideCollateralAmount(2702) if Currency(15) is not specified or is not the same.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2695")]
	pub side_collateral_currency: Option<String>,
	/// SideCollateralAmountType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2694")]
	pub side_collateral_amount_type: Option<i32>,
	/// SideCollateralFXRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2696")]
	pub side_collateral_fx_rate: Option<f64>,
	/// SideCollateralFXRateCalc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2697")]
	pub side_collateral_fx_rate_calc: Option<char>,
	/// SideCollateralType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2701")]
	pub side_collateral_type: Option<String>,
	/// SideCollateralAmountMarketSegmentID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2693")]
	pub side_collateral_amount_market_segment_id: Option<String>,
	/// SideCollateralAmountMarketID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2692")]
	pub side_collateral_amount_market_id: Option<String>,
	/// SideHaircutIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2703")]
	pub side_haircut_indicator: Option<crate::entities::Boolean>,
	/// SideCollateralPortfolioID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2700")]
	pub side_collateral_portfolio_id: Option<String>,
	/// SideCollateralPercentOverage
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2699")]
	pub side_collateral_percent_overage: Option<f32>,
	/// SideCollateralMarketPrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2698")]
	pub side_collateral_market_price: Option<f64>,
	/// When multiple instances of the SideCollateralReinvestmentGrp component are present this field specifies the average reinvestment
	/// rate.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2862")]
	pub side_collateral_reinvestment_rate: Option<f32>,
	/// May be used to indicate that this entry applies to the underlying collateral instrument being referenced by the value in UnderlyingID(2874).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2863")]
	pub side_underlying_ref_id: Option<String>,
}
