
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StatsIndGrp {
	/// Number of statistics indicators
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1175")]
	pub stats_indicators: Option<fix_common::RepeatingValues<StatsIndicator>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StatsIndicator {
	/// Indicates that the MD Entry is eligible for inclusion in the type of statistic specified by the StatsType. Must be provided
	/// if NoStatsIndicators greater than 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1176")]
	pub stats_type: Option<StatsType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StatsType {
	/// Exchange Last
	#[serde(rename = "1")]
	ExchangeLast,
	/// High / Low Price
	#[serde(rename = "2")]
	HighLowPrice,
	/// Average Price (VWAP, TWAP ... )
	#[serde(rename = "3")]
	AveragePrice,
	/// Turnover (Price * Qty)
	#[serde(rename = "4")]
	Turnover,
}

impl Default for StatsType {
	fn default() -> Self {
		StatsType::ExchangeLast
	}
}
