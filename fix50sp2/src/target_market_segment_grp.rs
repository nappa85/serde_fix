
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TargetMarketSegmentGrp {
	/// NoTargetMarketSegments
	#[serde(rename = "1789")]
	pub target_market_segments: fix_common::RepeatingValues<TargetMarketSegment>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TargetMarketSegment {
	/// Required when NoTargetMarketSegments(1789) &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1790")]
	pub target_market_segment_id: Option<String>,
}
