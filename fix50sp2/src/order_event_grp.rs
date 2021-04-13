
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OrderEventGrp {
	/// NoOrderEvents
	#[serde(rename = "1795")]
	pub order_events: fix_common::RepeatingValues<OrderEvent>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OrderEvent {
	/// Required when NoOrderEvents (1795) &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1796")]
	pub order_event_type: Option<OrderEventType>,
	/// OrderEventExecID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1797")]
	pub order_event_exec_id: Option<String>,
	/// OrderEventReason
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1798")]
	pub order_event_reason: Option<OrderEventReason>,
	/// OrderEventPx
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1799")]
	pub order_event_px: Option<f64>,
	/// OrderEventQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1800")]
	pub order_event_qty: Option<f64>,
	/// OrderEventLiquidityIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1801")]
	pub order_event_liquidity_indicator: Option<OrderEventLiquidityIndicator>,
	/// OrderEventText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1802")]
	pub order_event_text: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OrderEventType {
	/// Added (0=New)
	#[serde(rename = "1")]
	Added,
	/// Modified (5=Replaced)
	#[serde(rename = "2")]
	Modified,
	/// Deleted (4=Cancelled)
	#[serde(rename = "3")]
	Deleted,
	/// Partially Filled (F=Trade)
	#[serde(rename = "4")]
	PartiallyFilled,
	/// Filled (F=Trade)
	#[serde(rename = "5")]
	Filled,
	/// Suspended (9=Suspended)
	#[serde(rename = "6")]
	Suspended,
	/// Released (N=Released)
	#[serde(rename = "7")]
	Released,
	/// Restated (D=Restated)
	#[serde(rename = "8")]
	Restated,
	/// Locked (M=Locked)
	#[serde(rename = "9")]
	Locked,
	/// Triggered (L=Triggered)
	#[serde(rename = "10")]
	Triggered,
	/// Activated (L=Triggered or Activated by System)
	#[serde(rename = "11")]
	Activated,
}

impl Default for OrderEventType {
	fn default() -> Self {
		OrderEventType::Added
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OrderEventReason {
	/// Add order request
	#[serde(rename = "1")]
	AddOrderRequest,
	/// Modify order request
	#[serde(rename = "2")]
	ModifyOrderRequest,
	/// Delete order request
	#[serde(rename = "3")]
	DeleteOrderRequest,
	/// Order entered out-of-band
	#[serde(rename = "4")]
	OrderEnteredOutOfBand,
	/// Order modified out-of-band
	#[serde(rename = "5")]
	OrderModifiedOutOfBand,
	/// Order deleted out-of-band
	#[serde(rename = "6")]
	OrderDeletedOutOfBand,
	/// Order activated or triggered
	#[serde(rename = "7")]
	OrderActivatedOrTriggered,
	/// Order expired
	#[serde(rename = "8")]
	OrderExpired,
	/// Reserve order refreshed
	#[serde(rename = "9")]
	ReserveOrderRefreshed,
	/// Away market better
	#[serde(rename = "10")]
	AwayMarketBetter,
	/// Corporate action
	#[serde(rename = "11")]
	CorporateAction,
	/// Start of day
	#[serde(rename = "12")]
	StartOfDay,
	/// End of day
	#[serde(rename = "13")]
	EndOfDay,
}

impl Default for OrderEventReason {
	fn default() -> Self {
		OrderEventReason::AddOrderRequest
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OrderEventLiquidityIndicator {
	/// Added Liquidity
	#[serde(rename = "1")]
	AddedLiquidity,
	/// Removed Liquidity
	#[serde(rename = "2")]
	RemovedLiquidity,
	/// Liquidity Routed Out
	#[serde(rename = "3")]
	LiquidityRoutedOut,
	/// Auction
	#[serde(rename = "4")]
	Auction,
}

impl Default for OrderEventLiquidityIndicator {
	fn default() -> Self {
		OrderEventLiquidityIndicator::AddedLiquidity
	}
}
