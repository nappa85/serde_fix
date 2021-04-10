
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Dont {
	/// MsgType = Q
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// Broker Order ID as identified on problem execution
	#[serde(rename = "37")]
	pub order_id: String,
	/// SecondaryOrderID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "198")]
	pub secondary_order_id: Option<String>,
	/// Execution ID of problem execution
	#[serde(rename = "17")]
	pub exec_id: String,
	/// DKReason
	#[serde(rename = "127")]
	pub dk_reason: DKReason,
	/// Insert here the set of "Instrument" fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub instrument: super::super::instrument::Instrument,
	/// Number of underlyings
	#[serde(flatten)]
	pub und_instrmt_grp: Option<super::super::und_instrmt_grp::UndInstrmtGrp>,
	/// Number of Legs
	#[serde(flatten)]
	pub instrmt_leg_grp: Option<super::super::instrmt_leg_grp::InstrmtLegGrp>,
	/// Side
	#[serde(rename = "54")]
	pub side: Side,
	/// Insert here the set of "OrderQtyData" fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub order_qty_data: super::super::order_qty_data::OrderQtyData,
	/// Required if specified on the ExecutionRpt
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "32")]
	pub last_qty: Option<f64>,
	/// Required if specified on the ExecutionRpt
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "31")]
	pub last_px: Option<f64>,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if <a href="tag_355_EncodedText.html" target="bottom">EncodedText&nbsp;(355)</a> field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "354")]
	pub encoded_text_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the <a href="tag_58_Text.html" target="bottom">Text&nbsp;(58)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "355")]
	pub encoded_text: Option<String>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
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
	/// No matching execution report
	#[serde(rename = "G")]
	NoMatchingExecutionReport,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
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
	/// Sell undisclosed
	#[serde(rename = "H")]
	SellUndisclosed,
}
