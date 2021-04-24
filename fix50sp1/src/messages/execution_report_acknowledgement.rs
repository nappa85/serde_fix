
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExecutionReportAcknowledgement {
	/// MsgType = BN
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'B', 'N'>,
	/// OrderID
	#[serde(rename = "37")]
	pub order_id: String,
	/// SecondaryOrderID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "198")]
	pub secondary_order_id: Option<String>,
	/// Conditionally required if the Execution Report message contains a ClOrdID.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "11")]
	pub cl_ord_id: Option<String>,
	/// Indicates the status of the execution acknowledgement. The "received, not yet processed" is an optional intermediary status
	/// that can be used to notify the counterparty that the Execution Report has been received.
	#[serde(rename = "1036")]
	pub exec_ack_status: ExecAckStatus,
	/// The <a href="tag_17_ExecID.html" target="bottom">ExecID&nbsp;(17)</a> of the Execution Report being acknowledged.
	#[serde(rename = "17")]
	pub exec_id: String,
	/// Conditionally required when <a href="tag_1036_ExecAckStatus.html" target="bottom">ExecAckStatus&nbsp;(1036)</a> = 2 (Don't know / Rejected).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "127")]
	pub dk_reason: Option<DKReason>,
	/// Instrument
	#[serde(flatten)]
	pub instrument: super::super::instrument::Instrument,
	/// Number of underlyings
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "711")]
	pub underlyings: Option<fix_common::RepeatingValues<super::super::underlying_instrument::UnderlyingInstrument>>,
	/// Number of legs. Identifies a Multi-leg Execution if present and non-zero.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "555")]
	pub legs: Option<fix_common::RepeatingValues<super::super::instrument_leg::InstrumentLeg>>,
	/// Side
	#[serde(rename = "54")]
	pub side: Side,
	/// OrderQtyData
	#[serde(flatten)]
	pub order_qty_data: super::super::order_qty_data::OrderQtyData,
	/// Conditionally required if specified on the Execution Report
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "32")]
	pub last_qty: Option<f64>,
	/// Conditionally Required if specified on the Execution Report
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "31")]
	pub last_px: Option<f64>,
	/// Conditionally required if specified on the Execution Report
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "423")]
	pub price_type: Option<PriceType>,
	/// Conditionally required if specified on the Execution Report
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "669")]
	pub last_par_px: Option<f64>,
	/// Conditionally required if specified on the Execution Report
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "14")]
	pub cum_qty: Option<f64>,
	/// Conditionally required if specified on the Execution Report
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "6")]
	pub avg_px: Option<f64>,
	/// Conditionally required if <a href="tag_127_DKReason.html" target="bottom">DKReason&nbsp;(127)</a> = "other".
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// EncodedTextLen
	#[serde(rename = "354")]
	/// EncodedText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}





#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ExecAckStatus {
	/// Received, not yet processed
	#[serde(rename = "0")]
	ReceivedNotYetProcessed,
	/// Accepted
	#[serde(rename = "1")]
	Accepted,
	/// Don't know / Rejected
	#[serde(rename = "2")]
	DonTKnowRejected,
}

impl Default for ExecAckStatus {
	fn default() -> Self {
		ExecAckStatus::ReceivedNotYetProcessed
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum DKReason {
	/// Unknown Symbol
	#[serde(rename = "A")]
	UnknownSymbol,
	/// Wrong Side
	#[serde(rename = "B")]
	WrongSide,
	/// Quantity Exceeds Order
	#[serde(rename = "C")]
	QuantityExceedsOrder,
	/// No Matching Order
	#[serde(rename = "D")]
	NoMatchingOrder,
	/// Price Exceeds Limit
	#[serde(rename = "E")]
	PriceExceedsLimit,
	/// Calculation Difference
	#[serde(rename = "F")]
	CalculationDifference,
	/// Other
	#[serde(rename = "Z")]
	Other,
}

impl Default for DKReason {
	fn default() -> Self {
		DKReason::UnknownSymbol
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Side {
	/// Buy
	#[serde(rename = "1")]
	Buy,
	/// Sell
	#[serde(rename = "2")]
	Sell,
	/// Buy minus
	#[serde(rename = "3")]
	BuyMinus,
	/// Sell plus
	#[serde(rename = "4")]
	SellPlus,
	/// Sell short
	#[serde(rename = "5")]
	SellShort,
	/// Sell short exempt
	#[serde(rename = "6")]
	SellShortExempt,
	/// Undisclosed (valid for IOI and List Order messages only)
	#[serde(rename = "7")]
	Undisclosed,
	/// Cross (orders where counterparty is an exchange, valid for all messages except IOIs)
	#[serde(rename = "8")]
	Cross,
	/// Cross short
	#[serde(rename = "9")]
	CrossShort,
	/// Cross short exempt
	#[serde(rename = "A")]
	CrossShortExempt,
	/// "As Defined" (for use with multileg instruments)
	#[serde(rename = "B")]
	AsDefined,
	/// "Opposite" (for use with multileg instruments)
	#[serde(rename = "C")]
	Opposite,
	/// Subscribe (e.g. CIV)
	#[serde(rename = "D")]
	Subscribe,
	/// Redeem (e.g. CIV)
	#[serde(rename = "E")]
	Redeem,
	/// Lend (FINANCING - identifies direction of collateral)
	#[serde(rename = "F")]
	Lend,
	/// Borrow (FINANCING - identifies direction of collateral)
	#[serde(rename = "G")]
	Borrow,
}

impl Default for Side {
	fn default() -> Self {
		Side::Buy
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PriceType {
	/// Percentage (e.g. percent of par) (often called "dollar price" for fixed income)
	#[serde(rename = "1")]
	Percentage,
	/// Per unit (i.e. per share or contract)
	#[serde(rename = "2")]
	PerUnit,
	/// Fixed Amount (absolute value)
	#[serde(rename = "3")]
	FixedAmount,
	/// Discount - percentage points below par
	#[serde(rename = "4")]
	DiscountPercentagePointsBelowPar,
	/// Premium - percentage points over par
	#[serde(rename = "5")]
	PremiumPercentagePointsOverPar,
	/// Spread
	#[serde(rename = "6")]
	Spread,
	/// TED price
	#[serde(rename = "7")]
	TedPrice,
	/// TED yield
	#[serde(rename = "8")]
	TedYield,
	/// Yield
	#[serde(rename = "9")]
	Yield,
	/// Fixed cabinet trade price (primarily for listed futures and options)
	#[serde(rename = "10")]
	FixedCabinetTradePrice,
	/// Variable cabinet trade price (primarily for listed futures and options)
	#[serde(rename = "11")]
	VariableCabinetTradePrice,
	/// Product ticks in halfs
	#[serde(rename = "13")]
	ProductTicksInHalfs,
	/// Product ticks in fourths
	#[serde(rename = "14")]
	ProductTicksInFourths,
	/// Product ticks in eights
	#[serde(rename = "15")]
	ProductTicksInEights,
	/// Product ticks in sixteenths
	#[serde(rename = "16")]
	ProductTicksInSixteenths,
	/// Product ticks in thirty-seconds
	#[serde(rename = "17")]
	ProductTicksInThirtySeconds,
	/// Product ticks in sixty-forths
	#[serde(rename = "18")]
	ProductTicksInSixtyForths,
	/// Product ticks in one-twenty-eights
	#[serde(rename = "19")]
	ProductTicksInOneTwentyEights,
}

impl Default for PriceType {
	fn default() -> Self {
		PriceType::Percentage
	}
}
