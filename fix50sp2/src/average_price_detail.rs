
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AveragePriceDetail {
	/// AveragePriceType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2763")]
	pub average_price_type: Option<AveragePriceType>,
	/// Required if AveragePriceType(2763)=2 (Percent of volume average price) or 0 (Time weighted average price).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2764")]
	pub average_price_start_time: Option<fix_common::UTCTimestamp>,
	/// Required if AveragePriceType(2763)=2 (Percent of volume average price) or 0 (Time weighted average price).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2765")]
	pub average_price_end_time: Option<fix_common::UTCTimestamp>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AveragePriceType {
	/// Time weighted average price
	#[serde(rename = "0")]
	TimeWeightedAveragePrice,
	/// Volume weighted average price
	#[serde(rename = "1")]
	VolumeWeightedAveragePrice,
	/// Percent of volume average price
	#[serde(rename = "2")]
	PercentOfVolumeAveragePrice,
	/// Limit order average price
	#[serde(rename = "3")]
	LimitOrderAveragePrice,
}
