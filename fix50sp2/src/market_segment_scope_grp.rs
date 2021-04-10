
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MarketSegmentScopeGrp {
	/// NoMarketSegments
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1310")]
	pub market_segments: Option<fix_common::RepeatingValues<MarketSegment>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MarketSegment {
	/// Required if NoMarketSegments(1310) &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1301")]
	pub market_id: Option<String>,
	/// MarketSegmentID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1300")]
	pub market_segment_id: Option<String>,
}
