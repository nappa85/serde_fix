
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClearingPriceParametersGrp {
	/// NoClearingPriceParameters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2580")]
	pub clearing_price_parameters: Option<fix_common::RepeatingValues<ClearingPriceParameter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClearingPriceParameter {
	/// Required if NoClearingPriceParameters (2580) &gt; 0. Use to identify the relative business day to which the parameters apply.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2581")]
	pub business_day_type: Option<i32>,
	/// ClearingPriceOffset
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2582")]
	pub clearing_price_offset: Option<f64>,
	/// VegaMultiplier
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2583")]
	pub vega_multiplier: Option<f64>,
	/// AnnualTradingBusinessDays
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2584")]
	pub annual_trading_business_days: Option<i32>,
	/// TotalTradingBusinessDays
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2585")]
	pub total_trading_business_days: Option<i32>,
	/// TradingBusinessDays
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2586")]
	pub trading_business_days: Option<i32>,
	/// StandardVariance
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2588")]
	pub standard_variance: Option<f64>,
	/// RealizedVariance
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2587")]
	pub realized_variance: Option<f64>,
	/// RelatedClosePrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2589")]
	pub related_close_price: Option<f64>,
	/// Interest rate until the instrument expires and used to calculate DiscountFactor(1592).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1190")]
	pub risk_free_rate: Option<f64>,
	/// Used to calculate AccumulatedReturnModifiedVariationMargin(2591).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2590")]
	pub overnight_interest_rate: Option<f64>,
	/// AccumulatedReturnModifiedVariationMargin
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2591")]
	pub accumulated_return_modified_variation_margin: Option<f64>,
	/// DiscountFactor
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1592")]
	pub discount_factor: Option<f64>,
	/// Volatility
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1188")]
	pub volatility: Option<f64>,
	/// SettlPrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "730")]
	pub settl_price: Option<f64>,
	/// CalculationMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2592")]
	pub calculation_method: Option<CalculationMethod>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum CalculationMethod {
	/// Automatic (default)
	#[serde(rename = "0")]
	Automatic,
	/// Manual
	#[serde(rename = "1")]
	Manual,
}

impl Default for CalculationMethod {
	fn default() -> Self {
		CalculationMethod::Automatic
	}
}
