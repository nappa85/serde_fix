
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MDStatisticReqGrp {
	/// NoMDStatistics
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2474")]
	pub md_statistics: Option<fix_common::RepeatingValues<MDStatistic>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MDStatistic {
	/// Unique statistics identifier used as a placeholder for a set of parameters. If an ID is not applicable use "[N/A]". Required
	/// if <a href="tag_2474_NoMDStatistics.html" target="bottom">NoMDStatistics (2474)&nbsp;(2474)</a> &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2475")]
	pub md_statistic_id: Option<String>,
}
