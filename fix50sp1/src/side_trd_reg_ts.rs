
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SideTrdRegTS {
	/// NoSideTrdRegTS
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1016")]
	pub side_trd_reg_ts: Option<fix_common::RepeatingValues<SideTrdRegT>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SideTrdRegT {
	/// SideTrdRegTimestamp
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1012")]
	pub side_trd_reg_timestamp: Option<fix_common::UTCTimestamp>,
	/// SideTrdRegTimestampType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1013")]
	pub side_trd_reg_timestamp_type: Option<SideTrdRegTimestampType>,
	/// SideTrdRegTimestampSrc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1014")]
	pub side_trd_reg_timestamp_src: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SideTrdRegTimestampType {
	/// Execution Time
	#[serde(rename = "1")]
	ExecutionTime,
	/// Time In
	#[serde(rename = "2")]
	TimeIn,
	/// Time Out
	#[serde(rename = "3")]
	TimeOut,
	/// Broker Receipt
	#[serde(rename = "4")]
	BrokerReceipt,
	/// Broker Execution
	#[serde(rename = "5")]
	BrokerExecution,
	/// Desk Receipt
	#[serde(rename = "6")]
	DeskReceipt,
}

impl Default for SideTrdRegTimestampType {
	fn default() -> Self {
		SideTrdRegTimestampType::ExecutionTime
	}
}
