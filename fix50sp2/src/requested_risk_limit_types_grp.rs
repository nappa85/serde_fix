
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RequestedRiskLimitTypesGrp {
	/// NoRequestedRiskLimitType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1668")]
	pub requested_risk_limit_type: Option<fix_common::RepeatingValues<RequestedRiskLimitTyp>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RequestedRiskLimitTyp {
	/// <p>Required when <a href="tag_1668_NoRequestedRiskLimitType.html" target="bottom">NoRequestedRiskLimitType (1668)&nbsp;(1668)</a> &gt; 0
	/// </p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1530")]
	pub risk_limit_type: Option<RiskLimitType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RiskLimitType {
	/// Gross Limit
	#[serde(rename = "1")]
	GrossLimit,
	/// Net Limit
	#[serde(rename = "2")]
	NetLimit,
	/// Exposure
	#[serde(rename = "3")]
	Exposure,
	/// Long Limit
	#[serde(rename = "4")]
	LongLimit,
	/// Short Limit
	#[serde(rename = "5")]
	ShortLimit,
	/// CashMargin
	#[serde(rename = "6")]
	CashMargin,
	/// AdditionalMargin
	#[serde(rename = "7")]
	AdditionalMargin,
	/// TotalMargin
	#[serde(rename = "8")]
	TotalMargin,
	/// Credit limit
	#[serde(rename = "0")]
	CreditLimit,
	/// Limit consumed
	#[serde(rename = "9")]
	LimitConsumed,
	/// Clip size/notional limit per time period
	#[serde(rename = "10")]
	ClipSizeNotionalLimitPerTimePeriod,
	/// Maximum notional order size
	#[serde(rename = "11")]
	MaximumNotionalOrderSize,
	/// DV01/PV01 limit
	#[serde(rename = "12")]
	Dv01Pv01Limit,
	/// CS01 limit
	#[serde(rename = "13")]
	Cs01Limit,
	/// Volume limit per time period
	#[serde(rename = "14")]
	VolumeLimitPerTimePeriod,
	/// Volume filled as percent of ordered volume per time period
	#[serde(rename = "15")]
	VolumeFilledAsPercentOfOrderedVolumePerTimePeriod,
	/// Notional filled as percent of notional per time period
	#[serde(rename = "16")]
	NotionalFilledAsPercentOfNotionalPerTimePeriod,
	/// Transaction/execution limit per time period
	#[serde(rename = "17")]
	TransactionExecutionLimitPerTimePeriod,
}
