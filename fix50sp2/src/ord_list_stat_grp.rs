
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OrdListStatGrp {
	/// Number of orders statused in this message, i.e. number of repeating groups to follow.
	#[serde(rename = "73")]
	pub orders: fix_common::RepeatingValues<Order>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Order {
	/// Required when referring to orders that were electronically submitted over FIX or otherwise assigned a ClOrdID.
	#[serde(rename = "11")]
	pub cl_ord_id: String,
	/// OrderID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "37")]
	pub order_id: Option<String>,
	/// SecondaryClOrdID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "526")]
	pub secondary_cl_ord_id: Option<String>,
	/// CumQty
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "14")]
	pub cum_qty: f64,
	/// OrdStatus
	#[serde(rename = "39")]
	pub ord_status: OrdStatus,
	/// For optional use with OrdStatus = 0 (New)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "636")]
	pub working_indicator: Option<WorkingIndicator>,
	/// Quantity open for further execution. LeavesQty = OrderQty - CumQty.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "151")]
	pub leaves_qty: Option<f64>,
	/// CxlQty
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "84")]
	pub cxl_qty: f64,
	/// AvgPx
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "6")]
	pub avg_px: f64,
	/// Used if the order is rejected
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "103")]
	pub ord_rej_reason: Option<OrdRejReason>,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if EncodedText field is specified and must immediately precede it.
	#[serde(rename = "354")]
	/// Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum OrdStatus {
	/// New
	#[serde(rename = "0")]
	New,
	/// Partially filled
	#[serde(rename = "1")]
	PartiallyFilled,
	/// Filled
	#[serde(rename = "2")]
	Filled,
	/// Done for day
	#[serde(rename = "3")]
	DoneForDay,
	/// Canceled
	#[serde(rename = "4")]
	Canceled,
	/// Replaced (No longer used)
	#[serde(rename = "5")]
	Replaced,
	/// Pending Cancel (i.e. result of Order Cancel Request)
	#[serde(rename = "6")]
	PendingCancel,
	/// Stopped
	#[serde(rename = "7")]
	Stopped,
	/// Rejected
	#[serde(rename = "8")]
	Rejected,
	/// Suspended
	#[serde(rename = "9")]
	Suspended,
	/// Pending New
	#[serde(rename = "A")]
	PendingNew,
	/// Calculated
	#[serde(rename = "B")]
	Calculated,
	/// Expired
	#[serde(rename = "C")]
	Expired,
	/// Accepted for Bidding
	#[serde(rename = "D")]
	AcceptedForBidding,
	/// Pending Replace (i.e. result of Order Cancel/Replace Request)
	#[serde(rename = "E")]
	PendingReplace,
}

impl Default for OrdStatus {
	fn default() -> Self {
		OrdStatus::New
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum WorkingIndicator {
	/// Order has been accepted but not yet in a working state
	#[serde(rename = "N")]
	OrderHasBeenAcceptedButNotYetInAWorkingState,
	/// Order is currently being worked
	#[serde(rename = "Y")]
	OrderIsCurrentlyBeingWorked,
}

impl Default for WorkingIndicator {
	fn default() -> Self {
		WorkingIndicator::OrderHasBeenAcceptedButNotYetInAWorkingState
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum OrdRejReason {
	/// Broker / Exchange option
	#[serde(rename = "0")]
	BrokerExchangeOption,
	/// Unknown symbol
	#[serde(rename = "1")]
	UnknownSymbol,
	/// Exchange closed
	#[serde(rename = "2")]
	ExchangeClosed,
	/// Order exceeds limit
	#[serde(rename = "3")]
	OrderExceedsLimit,
	/// Too late to enter
	#[serde(rename = "4")]
	TooLateToEnter,
	/// Unknown order
	#[serde(rename = "5")]
	UnknownOrder,
	/// Duplicate Order (e.g. dupe ClOrdID)
	#[serde(rename = "6")]
	DuplicateOrder,
	/// Duplicate of a verbally communicated order
	#[serde(rename = "7")]
	DuplicateOfAVerballyCommunicatedOrder,
	/// Stale order
	#[serde(rename = "8")]
	StaleOrder,
	/// Trade along required
	#[serde(rename = "9")]
	TradeAlongRequired,
	/// Invalid Investor ID
	#[serde(rename = "10")]
	InvalidInvestorId,
	/// Unsupported order characteristic
	#[serde(rename = "11")]
	UnsupportedOrderCharacteristic,
	/// Surveillance option
	#[serde(rename = "12")]
	SurveillanceOption,
	/// Incorrect quantity
	#[serde(rename = "13")]
	IncorrectQuantity,
	/// Incorrect allocated quantity
	#[serde(rename = "14")]
	IncorrectAllocatedQuantity,
	/// Unknown account(s)
	#[serde(rename = "15")]
	UnknownAccount,
	/// Price exceeds current price band
	#[serde(rename = "16")]
	PriceExceedsCurrentPriceBand,
	/// Invalid price increment
	#[serde(rename = "18")]
	InvalidPriceIncrement,
	/// Other
	#[serde(rename = "99")]
	Other,
	/// Reference price not available
	#[serde(rename = "19")]
	ReferencePriceNotAvailable,
	/// Notional value exceeds threshold
	#[serde(rename = "20")]
	NotionalValueExceedsThreshold,
	/// Algorithm risk threshold breached (Elaboration: Elaboration: A sell-side broker algorithm has detected that a risk limit has
	/// been breached which requires further communication with the client. Used in conjunction with Text(58) to convey the details
	/// of the specific event.
	#[serde(rename = "21")]
	AlgorithmRiskThresholdBreachedToConveyTheDetailsOfTheSpecificEvent,
	/// Short sell not permitted
	#[serde(rename = "22")]
	ShortSellNotPermitted,
	/// Short sell rejected due to security pre-borrow restriction
	#[serde(rename = "23")]
	ShortSellRejectedDueToSecurityPreBorrowRestriction,
	/// Short sell rejected due to account pre-borrow restriction
	#[serde(rename = "24")]
	ShortSellRejectedDueToAccountPreBorrowRestriction,
	/// Insufficient credit limit
	#[serde(rename = "25")]
	InsufficientCreditLimit,
	/// Exceeded clip size limit
	#[serde(rename = "26")]
	ExceededClipSizeLimit,
	/// Exceeded maximum notional order amount
	#[serde(rename = "27")]
	ExceededMaximumNotionalOrderAmount,
	/// Exceeded DV01/PV01 limit
	#[serde(rename = "28")]
	ExceededDv01Pv01Limit,
	/// Exceeded CS01 limit
	#[serde(rename = "29")]
	ExceededCs01Limit,
}

impl Default for OrdRejReason {
	fn default() -> Self {
		OrdRejReason::BrokerExchangeOption
	}
}
