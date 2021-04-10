
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MarketSegmentGrp {
	/// Number of Market Segments on which a security may trade..
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1310")]
	pub market_segments: Option<crate::entities::RepeatingValues<MarketSegment>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MarketSegment {
	/// Identifies the market which lists and trades the instrument.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1301")]
	pub market_id: Option<String>,
	/// Identifies the segment of the market to which the specify trading rules and listing rules apply.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1300")]
	pub market_segment_id: Option<String>,
}
