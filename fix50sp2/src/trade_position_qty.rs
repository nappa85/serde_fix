
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TradePositionQty {
	/// NoPositions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "702")]
	pub positions: Option<fix_common::RepeatingValues<Position>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Position {
	/// PosType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "703")]
	pub pos_type: Option<PosType>,
	/// LongQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "704")]
	pub long_qty: Option<f64>,
	/// ShortQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "705")]
	pub short_qty: Option<f64>,
	/// CoveredQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1654")]
	pub covered_qty: Option<f64>,
	/// PosQtyStatus
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "706")]
	pub pos_qty_status: Option<PosQtyStatus>,
	/// QuantityDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "976")]
	pub quantity_date: Option<fix_common::LocalMktDate>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PosType {
	/// Allocation Trade Qty
	#[serde(rename = "ALC")]
	AllocationTradeQty,
	/// Option Assignment
	#[serde(rename = "AS")]
	OptionAssignment,
	/// As-of Trade Qty
	#[serde(rename = "ASF")]
	AsOfTradeQty,
	/// Delivery Qty
	#[serde(rename = "DLV")]
	DeliveryQty,
	/// Electronic Trade Qty
	#[serde(rename = "ETR")]
	ElectronicTradeQty,
	/// Option Exercise Qty
	#[serde(rename = "EX")]
	OptionExerciseQty,
	/// End-of-Day Qty
	#[serde(rename = "FIN")]
	EndOfDayQty,
	/// Intra-spread Qty
	#[serde(rename = "IAS")]
	IntraSpreadQty,
	/// Inter-spread Qty
	#[serde(rename = "IES")]
	InterSpreadQty,
	/// Adjustment Qty
	#[serde(rename = "PA")]
	AdjustmentQty,
	/// Pit Trade Qty
	#[serde(rename = "PIT")]
	PitTradeQty,
	/// Privately negotiated trade qty (non-regulated)
	#[serde(rename = "PNTN")]
	PrivatelyNegotiatedTradeQty,
	/// Start-of-Day Qty
	#[serde(rename = "SOD")]
	StartOfDayQty,
	/// Integral Split
	#[serde(rename = "SPL")]
	IntegralSplit,
	/// Transaction from Assignment
	#[serde(rename = "TA")]
	TransactionFromAssignment,
	/// Total Transaction Qty
	#[serde(rename = "TOT")]
	TotalTransactionQty,
	/// Transaction Quantity
	#[serde(rename = "TQ")]
	TransactionQuantity,
	/// Transfer Trade Qty
	#[serde(rename = "TRF")]
	TransferTradeQty,
	/// Transaction from Exercise
	#[serde(rename = "TX")]
	TransactionFromExercise,
	/// Cross Margin Qty
	#[serde(rename = "XM")]
	CrossMarginQty,
	/// Receive Quantity
	#[serde(rename = "RCV")]
	ReceiveQuantity,
	/// Corporate Action Adjustment
	#[serde(rename = "CAA")]
	CorporateActionAdjustment,
	/// Delivery Notice Qty
	#[serde(rename = "DN")]
	DeliveryNoticeQty,
	/// Exchange for Physical Qty
	#[serde(rename = "EP")]
	ExchangeForPhysicalQty,
	/// Net Delta Qty
	#[serde(rename = "DLT ")]
	NetDeltaQty,
	/// Credit Event Adjustment
	#[serde(rename = "CEA ")]
	CreditEventAdjustment,
	/// Succession Event Adjustment
	#[serde(rename = "SEA ")]
	SuccessionEventAdjustment,
	/// Net Qty
	#[serde(rename = "NET")]
	NetQty,
	/// Gross Qty
	#[serde(rename = "GRS")]
	GrossQty,
	/// Intraday Qty
	#[serde(rename = "ITD")]
	IntradayQty,
	/// Gross long non-delta-adjusted swaption position
	#[serde(rename = "NDAS")]
	GrossLongNonDeltaAdjustedSwaptionPosition,
	/// Long delta-adjusted paired swaption position
	#[serde(rename = "DAS")]
	LongDeltaAdjustedPairedSwaptionPosition,
	/// Expiring quantity (Elaboration: The position quantity on expiration day after the application of trade and post trade activity,
	/// but prior to the application of exercises and assignments)
	#[serde(rename = "EXP")]
	ExpiringQuantity,
	/// Quantity not exercised (Elaboration: The exercise quantity requested that was not allowed, e.g., the exercise quantity requested
	/// that exceeded the final long position
	#[serde(rename = "UNEX")]
	QuantityNotExercisedElaborationTheExerciseQuantityRequestedThatWasNotAllowedEGTheExerciseQuantityRequestedThatExceededTheFinalLongPosition,
	/// Requested exercise quantity (Elaboration: The exercise quantity requested. It may differ from the exercise quantity if it
	/// exceeds the final long position)
	#[serde(rename = "REQ")]
	RequestedExerciseQuantity,
	/// Cash futures equivalent quantity
	#[serde(rename = "CFE")]
	CashFuturesEquivalentQuantity,
	/// Loan or borrowed quantity
	#[serde(rename = "SECLN")]
	LoanOrBorrowedQuantity,
}

impl Default for PosType {
	fn default() -> Self {
		PosType::AllocationTradeQty
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PosQtyStatus {
	/// Submitted
	#[serde(rename = "0")]
	Submitted,
	/// Accepted
	#[serde(rename = "1")]
	Accepted,
	/// Rejected
	#[serde(rename = "2")]
	Rejected,
}

impl Default for PosQtyStatus {
	fn default() -> Self {
		PosQtyStatus::Submitted
	}
}
