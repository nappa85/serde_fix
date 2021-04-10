
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExecCollGrp {
	/// Executions for which collateral is required.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "124")]
	pub execs: Option<fix_common::RepeatingValues<Exec>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Exec {
	/// Required if <a href="tag_124_NoExecs.html" target="bottom">NoExecs&nbsp;(124)</a> &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "17")]
	pub exec_id: Option<String>,
}
