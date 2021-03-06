
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MarginReqmtInqQualGrp {
	/// Number of qualifier entries
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1636")]
	pub margin_reqmt_inq_qualifier: Option<fix_common::RepeatingValues<MarginReqmtInqQualifie>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MarginReqmtInqQualifie {
	/// MarginReqmtInqQualifier
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1637")]
	pub margin_reqmt_inq_qualifier: Option<MarginReqmtInqQualifier>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MarginReqmtInqQualifier {
	/// Summary
	#[serde(rename = "0")]
	Summary,
	/// Detail
	#[serde(rename = "1")]
	Detail,
	/// Excess/Deficit
	#[serde(rename = "2")]
	ExcessDeficit,
	/// Net Position
	#[serde(rename = "3")]
	NetPosition,
}

impl Default for MarginReqmtInqQualifier {
	fn default() -> Self {
		MarginReqmtInqQualifier::Summary
	}
}
