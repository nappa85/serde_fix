
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NotAffectedMarketSegmentGrp {
	/// NoNotAffectedMarketSegments
	#[serde(rename = "1793")]
	pub not_affected_market_segments: crate::entities::RepeatingValues<NotAffectedMarketSegment>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NotAffectedMarketSegment {
	/// Required when NoNotAffectedMarketSegments(1793) &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1794")]
	pub not_affected_market_segment_id: Option<String>,
}
