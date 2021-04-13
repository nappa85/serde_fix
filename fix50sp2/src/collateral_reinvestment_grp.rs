
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CollateralReinvestmentGrp {
	/// NoCollateralReinvestments
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2845")]
	pub collateral_reinvestments: Option<fix_common::RepeatingValues<CollateralReinvestment>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CollateralReinvestment {
	/// Required if NoCollateralReinvestments(2845) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2844")]
	pub collateral_reinvestment_type: Option<CollateralReinvestmentType>,
	/// CollateralReinvestmentAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2842")]
	pub collateral_reinvestment_amount: Option<f64>,
	/// CollateralReinvestmentCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2843")]
	pub collateral_reinvestment_currency: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CollateralReinvestmentType {
	/// Money market fund
	#[serde(rename = "0")]
	MoneyMarketFund,
	/// Other comingled pool
	#[serde(rename = "1")]
	OtherComingledPool,
	/// Repo market
	#[serde(rename = "2")]
	RepoMarket,
	/// Direct purchase of securities
	#[serde(rename = "3")]
	DirectPurchaseOfSecurities,
	/// Other investments
	#[serde(rename = "4")]
	OtherInvestments,
}

impl Default for CollateralReinvestmentType {
	fn default() -> Self {
		CollateralReinvestmentType::MoneyMarketFund
	}
}
