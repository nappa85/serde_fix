
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ThrottleParamsGrp {
	/// Indicates number of throttles to follow
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1610")]
	pub throttles: Option<crate::entities::RepeatingValues<Throttle>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Throttle {
	/// Required when <a href="tag_1610_NoThrottles.html" target="bottom">NoThrottles&nbsp;(1610)</a> &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1611")]
	pub throttle_action: Option<ThrottleAction>,
	/// Required when <a href="tag_1610_NoThrottles.html" target="bottom">NoThrottles&nbsp;(1610)</a> &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1612")]
	pub throttle_type: Option<ThrottleType>,
	/// Number of messages per time interval, or number of outstanding requests. Required when <a href="tag_1610_NoThrottles.html" target="bottom">NoThrottles&nbsp;(1610)</a> &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1613")]
	pub throttle_no_msgs: Option<i32>,
	/// Can be used only when <a href="tag_1612_ThrottleType.html" target="bottom">ThrottleType&nbsp;(1612)</a> = Inbound Rate. Indicates, along with <a href="tag_1615_ThrottleTimeUnit.html" target="bottom">ThrottleTimeUnit&nbsp;(1615)</a> , the interval of time in which <a href="tag_1613_ThrottleNoMsgs.html" target="bottom">ThrottleNoMsgs&nbsp;(1613)</a> may be sent. Default is 1
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1614")]
	pub throttle_time_interval: Option<i32>,
	/// Can be used only when <a href="tag_1612_ThrottleType.html" target="bottom">ThrottleType&nbsp;(1612)</a> = Inbound Rate. Indicates, along with <a href="tag_1615_ThrottleTimeUnit.html" target="bottom">ThrottleTimeUnit&nbsp;(1615)</a> , the interval of time in which <a href="tag_1613_ThrottleNoMsgs.html" target="bottom">ThrottleNoMsgs&nbsp;(1613)</a> may be sent. Default is 1
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1615")]
	pub throttle_time_unit: Option<ThrottleTimeUnit>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ThrottleAction {
	/// Queue Inbound
	#[serde(rename = "0")]
	QueueInbound,
	/// Queue Outbound
	#[serde(rename = "1")]
	QueueOutbound,
	/// Reject
	#[serde(rename = "2")]
	Reject,
	/// Disconnect
	#[serde(rename = "3")]
	Disconnect,
	/// Warning
	#[serde(rename = "4")]
	Warning,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ThrottleType {
	/// Inbound Rate
	#[serde(rename = "0")]
	InboundRate,
	/// Outstanding Requests
	#[serde(rename = "1")]
	OutstandingRequests,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ThrottleTimeUnit {
	/// Seconds (default if not specified)
	#[serde(rename = "0")]
	Seconds,
	/// Tenths of a second
	#[serde(rename = "1")]
	TenthsOfASecond,
	/// Hundredths of a second
	#[serde(rename = "2")]
	HundredthsOfASecond,
	/// Milliseconds
	#[serde(rename = "3")]
	Milliseconds,
	/// Microseconds
	#[serde(rename = "4")]
	Microseconds,
	/// Nanoseconds
	#[serde(rename = "5")]
	Nanoseconds,
	/// Minutes
	#[serde(rename = "10")]
	Minutes,
	/// Hours
	#[serde(rename = "11")]
	Hours,
	/// Days
	#[serde(rename = "12")]
	Days,
	/// Weeks
	#[serde(rename = "13")]
	Weeks,
	/// Months
	#[serde(rename = "14")]
	Months,
	/// Years
	#[serde(rename = "15")]
	Years,
}
