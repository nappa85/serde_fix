
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrdRegTimestamps {
	/// NoTrdRegTimestamps
	#[serde(rename = "768")]
	pub trd_reg_timestamps: fix_common::RepeatingValues<TrdRegTimestamp>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrdRegTimestamp {
	/// Required if <a href="tag_768_NoTrdRegTimestamps.html" target="bottom">NoTrdRegTimestamps&nbsp;(768)</a> &gt;0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "769")]
	pub trd_reg_timestamp: Option<fix_common::UTCTimestamp>,
	/// Required if <a href="tag_768_NoTrdRegTimestamps.html" target="bottom">NoTrdRegTimestamps&nbsp;(768)</a> &gt;0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "770")]
	pub trd_reg_timestamp_type: Option<TrdRegTimestampType>,
	/// Optional
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "771")]
	pub trd_reg_timestamp_origin: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TrdRegTimestampType {
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
}

impl Default for TrdRegTimestampType {
	fn default() -> Self {
		TrdRegTimestampType::ExecutionTime
	}
}
