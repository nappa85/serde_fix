
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RelatedMarketSegmentGrp {
	/// NoRelatedMarketSegments
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2545")]
	pub related_market_segments: Option<fix_common::RepeatingValues<RelatedMarketSegment>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RelatedMarketSegment {
	/// Required if NoRelatedMarketSegments (2545) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2546")]
	pub related_market_segment_id: Option<String>,
	/// MarketSegmentRelationship
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2547")]
	pub market_segment_relationship: Option<MarketSegmentRelationship>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MarketSegmentRelationship {
	/// Market segment pool member
	#[serde(rename = "1")]
	MarketSegmentPoolMember,
	/// Retail segment
	#[serde(rename = "2")]
	RetailSegment,
	/// Wholesale segment
	#[serde(rename = "3")]
	WholesaleSegment,
}

impl Default for MarketSegmentRelationship {
	fn default() -> Self {
		MarketSegmentRelationship::MarketSegmentPoolMember
	}
}
