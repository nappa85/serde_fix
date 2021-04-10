
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AffectedMarketSegmentGrp {
	/// NoAffectedMarketSegments
	#[serde(rename = "1791")]
	pub affected_market_segments: crate::entities::RepeatingValues<AffectedMarketSegment>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AffectedMarketSegment {
	/// Required when NoAffectedMarketSegments(1791) &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1792")]
	pub affected_market_segment_id: Option<String>,
}
