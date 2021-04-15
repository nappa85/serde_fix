
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ThrottleResponse {
	/// ThrottleInst
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1685")]
	pub throttle_inst: Option<ThrottleInst>,
	/// ThrottleStatus
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1609")]
	pub throttle_status: Option<ThrottleStatus>,
	/// ThrottleCountIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1686")]
	pub throttle_count_indicator: Option<ThrottleCountIndicator>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ThrottleInst {
	/// Reject if throttle limit exceeded
	#[serde(rename = "0")]
	RejectIfThrottleLimitExceeded,
	/// Queue if throttle limit exceeded
	#[serde(rename = "1")]
	QueueIfThrottleLimitExceeded,
}

impl Default for ThrottleInst {
	fn default() -> Self {
		ThrottleInst::RejectIfThrottleLimitExceeded
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ThrottleStatus {
	/// Throttle limit not exceeded, not queued
	#[serde(rename = "0")]
	ThrottleLimitNotExceededNotQueued,
	/// Queued due to throttle limit exceeded
	#[serde(rename = "1")]
	QueuedDueToThrottleLimitExceeded,
}

impl Default for ThrottleStatus {
	fn default() -> Self {
		ThrottleStatus::ThrottleLimitNotExceededNotQueued
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ThrottleCountIndicator {
	/// Outstanding requests unchanged
	#[serde(rename = "0")]
	OutstandingRequestsUnchanged,
	/// Outstanding requests decreased
	#[serde(rename = "1")]
	OutstandingRequestsDecreased,
}

impl Default for ThrottleCountIndicator {
	fn default() -> Self {
		ThrottleCountIndicator::OutstandingRequestsUnchanged
	}
}
