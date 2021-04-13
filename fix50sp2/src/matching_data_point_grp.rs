
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MatchingDataPointGrp {
	/// NoMatchingDataPoints
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2781")]
	pub matching_data_points: Option<fix_common::RepeatingValues<MatchingDataPoint>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MatchingDataPoint {
	/// Required if NoMatchingDataPoints(2781) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2782")]
	pub matching_data_point_indicator: Option<MatchingDataPointIndicator>,
	/// Required if NoMatchingDataPoints(2781) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2783")]
	pub matching_data_point_value: Option<String>,
	/// Required if NoMatchingDataPoints(2781) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2784")]
	pub matching_data_point_type: Option<MatchingDataPointType>,
	/// MatchingDataPointName
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2785")]
	pub matching_data_point_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MatchingDataPointIndicator {
	/// Mandatory
	#[serde(rename = "1")]
	Mandatory,
	/// Optional
	#[serde(rename = "2")]
	Optional,
}

impl Default for MatchingDataPointIndicator {
	fn default() -> Self {
		MatchingDataPointIndicator::Mandatory
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MatchingDataPointType {
	/// Accrued interest
	#[serde(rename = "1")]
	AccruedInterest,
	/// Deal price
	#[serde(rename = "2")]
	DealPrice,
	/// Trade date (Tolerance not applicable)
	#[serde(rename = "3")]
	TradeDate,
	/// Settlement date (Tolerance not applicable)
	#[serde(rename = "4")]
	SettlementDate,
	/// Side indicator (Tolerance not applicable)
	#[serde(rename = "5")]
	SideIndicator,
	/// Traded currency (Tolerance not applicable)
	#[serde(rename = "6")]
	TradedCurrency,
	/// Account ID (Tolerance not applicable)
	#[serde(rename = "7")]
	AccountId,
	/// Executing broker ID (Tolerance not applicable)
	#[serde(rename = "8")]
	ExecutingBrokerId,
	/// Settlement currency and amount
	#[serde(rename = "9")]
	SettlementCurrencyAndAmount,
	/// Investment manager ID (Tolerance not applicable)
	#[serde(rename = "10")]
	InvestmentManagerId,
	/// Net amount
	#[serde(rename = "11")]
	NetAmount,
	/// Place of settlement (Tolerance not applicable)
	#[serde(rename = "12")]
	PlaceOfSettlement,
	/// Commissions
	#[serde(rename = "13")]
	Commissions,
	/// Security identifier (Tolerance not applicable)
	#[serde(rename = "14")]
	SecurityIdentifier,
	/// Quantity allocated
	#[serde(rename = "15")]
	QuantityAllocated,
	/// Principal
	#[serde(rename = "16")]
	Principal,
	/// Fees
	#[serde(rename = "17")]
	Fees,
	/// Tax
	#[serde(rename = "18")]
	Tax,
}

impl Default for MatchingDataPointType {
	fn default() -> Self {
		MatchingDataPointType::AccruedInterest
	}
}
