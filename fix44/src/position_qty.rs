
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PositionQty {
	/// NoPositions
	#[serde(rename = "702")]
	pub positions: fix_common::RepeatingValues<Position>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Position {
	/// Required if <a href="tag_702_NoPositions.html" target="bottom">NoPositions&nbsp;(702)</a> &gt;0
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
	/// PosQtyStatus
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "706")]
	pub pos_qty_status: Option<PosQtyStatus>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PosType {
	/// Transaction Quantity
	#[serde(rename = "TQ")]
	TransactionQuantity,
	/// Intra-Spread Qty
	#[serde(rename = "IAS")]
	IntraSpreadQty,
	/// Inter-Spread Qty
	#[serde(rename = "IES")]
	InterSpreadQty,
	/// End-of-Day Qty
	#[serde(rename = "FIN")]
	EndOfDayQty,
	/// Start-of-Day Qty
	#[serde(rename = "SOD")]
	StartOfDayQty,
	/// Option Exercise Qty
	#[serde(rename = "EX")]
	OptionExerciseQty,
	/// Option Assignment
	#[serde(rename = "AS")]
	OptionAssignment,
	/// Transaction from Exercise
	#[serde(rename = "TX")]
	TransactionFromExercise,
	/// Transaction from Assignment
	#[serde(rename = "TA")]
	TransactionFromAssignment,
	/// Pit Trade Qty
	#[serde(rename = "PIT")]
	PitTradeQty,
	/// Transfer Trade Qty
	#[serde(rename = "TRF")]
	TransferTradeQty,
	/// Electronic Trade Qty
	#[serde(rename = "ETR")]
	ElectronicTradeQty,
	/// Allocation Trade Qty
	#[serde(rename = "ALC")]
	AllocationTradeQty,
	/// Adjustment Qty
	#[serde(rename = "PA")]
	AdjustmentQty,
	/// As-of Trade Qty
	#[serde(rename = "ASF")]
	AsOfTradeQty,
	/// Delivery Qty
	#[serde(rename = "DLV")]
	DeliveryQty,
	/// Total Transaction Qty
	#[serde(rename = "TOT")]
	TotalTransactionQty,
	/// Cross Margin Qty
	#[serde(rename = "XM")]
	CrossMarginQty,
	/// Integral Split
	#[serde(rename = "SPL")]
	IntegralSplit,
}

impl Default for PosType {
	fn default() -> Self {
		PosType::TransactionQuantity
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
