
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FundingSourceGrp {
	/// NoFundingSources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2849")]
	pub funding_sources: Option<fix_common::RepeatingValues<FundingSource>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FundingSource {
	/// Required if NoFundingSources(2849) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2846")]
	pub funding_source_item: Option<FundingSourceItem>,
	/// FundingSourceMarketValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2848")]
	pub funding_source_market_value: Option<f64>,
	/// FundingSourceCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2847")]
	pub funding_source_currency: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FundingSourceItem {
	/// Repurchase agreement
	#[serde(rename = "0")]
	RepurchaseAgreement,
	/// Cash
	#[serde(rename = "1")]
	Cash,
	/// Free credits
	#[serde(rename = "2")]
	FreeCredits,
	/// Customer short sales
	#[serde(rename = "3")]
	CustomerShortSales,
	/// Broker short sales
	#[serde(rename = "4")]
	BrokerShortSales,
	/// Unsecured borrowing
	#[serde(rename = "5")]
	UnsecuredBorrowing,
	/// Other
	#[serde(rename = "99")]
	Other,
}

impl Default for FundingSourceItem {
	fn default() -> Self {
		FundingSourceItem::RepurchaseAgreement
	}
}
