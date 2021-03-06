
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfirmationRequest {
	/// MsgType = BH
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'B', 'H'>,
	/// Unique identifier for this message
	#[serde(rename = "859")]
	pub confirm_req_id: String,
	/// Denotes whether this message is being used to request a confirmation or a trade status message
	#[serde(rename = "773")]
	pub confirm_type: ConfirmType,
	/// Indicates number of orders to be combined for allocation. If order(s) were manually delivered set to 1 (one).Required when <a href="tag_857_AllocNoOrdersType.html" target="bottom">AllocNoOrdersType&nbsp;(857)</a> = 1
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "73")]
	pub orders: Option<fix_common::RepeatingValues<Order>>,
	/// Used to refer to an earlier Allocation Instruction.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "70")]
	pub alloc_id: Option<String>,
	/// Used to refer to an earlier <a href="message_Allocation_Instruction_J.html" target="main">Allocation Instruction&nbsp;(J)</a> via its secondary identifier
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "793")]
	pub secondary_alloc_id: Option<String>,
	/// Used to refer to an allocation account within an earlier Allocation Instruction.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "467")]
	pub individual_alloc_id: Option<String>,
	/// Represents the time this message was generated
	#[serde(rename = "60")]
	pub transact_time: fix_common::UTCTimestamp,
	/// Account number for the trade being confirmed by this message
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "79")]
	pub alloc_account: Option<String>,
	/// AllocAcctIDSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "661")]
	pub alloc_acct_id_source: Option<AllocAcctIDSource>,
	/// AllocAccountType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "798")]
	pub alloc_account_type: Option<AllocAccountType>,
	/// Text
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

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Order {
	/// Order ID assigned by client if order(s) were electronically delivered and executed. If order(s) were manually delivered this
	/// field should contain string "MANUAL".Note where an order has undergone one or more cancel/replaces, this should be the <a href="tag_11_ClOrdID.html" target="bottom">ClOrdID&nbsp;(11)</a> of the most recent version of the order. Required when <a href="tag_73_NoOrders.html" target="bottom">NoOrders&nbsp;(73)</a> &gt; 0 and must be the first repeating field in the group.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "11")]
	pub cl_ord_id: Option<String>,
	/// OrderID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "37")]
	pub order_id: Option<String>,
	/// Can be used to provide order id used by exchange or executing system.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "198")]
	pub secondary_order_id: Option<String>,
	/// SecondaryClOrdID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "526")]
	pub secondary_cl_ord_id: Option<String>,
	/// Required for List Orders.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "66")]
	pub list_id: Option<String>,
	/// OrderQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "38")]
	pub order_qty: Option<f64>,
	/// Average price for this order
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "799")]
	pub order_avg_px: Option<f64>,
	/// Quantity of this order that is being booked out by this message (will be equal to or less than this order's OrderQty) Note
	/// that the sum of the <a href="tag_800_OrderBookingQty.html" target="bottom">OrderBookingQty&nbsp;(800)</a> values in this repeating group must equal the total quantity being allocated (in <a href="tag_53_Quantity.html" target="bottom">Quantity&nbsp;(53)</a> (53) field)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "800")]
	pub order_booking_qty: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ConfirmType {
	/// Status
	#[serde(rename = "1")]
	Status,
	/// Confirmation
	#[serde(rename = "2")]
	Confirmation,
	/// Confirmation Request Rejected (reason can be stated in <a href="tag_58_Text.html" target="bottom">Text&nbsp;(58)</a> field)
	#[serde(rename = "3")]
	ConfirmationRequestRejectedAField,
}

impl Default for ConfirmType {
	fn default() -> Self {
		ConfirmType::Status
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum AllocAcctIDSource {
	/// BIC
	#[serde(rename = "1")]
	Bic,
	/// SID code
	#[serde(rename = "2")]
	SidCode,
	/// TFM (GSPTA)
	#[serde(rename = "3")]
	Tfm,
	/// OMGEO (AlertID)
	#[serde(rename = "4")]
	Omgeo,
	/// DTCC code
	#[serde(rename = "5")]
	DtccCode,
	/// Other (custom or proprietary)
	#[serde(rename = "99")]
	Other,
}

impl Default for AllocAcctIDSource {
	fn default() -> Self {
		AllocAcctIDSource::Bic
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum AllocAccountType {
	/// Account is carried on customer Side of Books
	#[serde(rename = "1")]
	AccountIsCarriedOnCustomerSideOfBooks,
	/// Account is carried on non-Customer Side of books
	#[serde(rename = "2")]
	AccountIsCarriedOnNonCustomerSideOfBooks,
	/// House Trader
	#[serde(rename = "3")]
	HouseTrader,
	/// Floor Trader
	#[serde(rename = "4")]
	FloorTrader,
	/// Account is carried on non-customer side of books and is cross margined
	#[serde(rename = "6")]
	AccountIsCarriedOnNonCustomerSideOfBooksAndIsCrossMargined,
	/// Account is house trader and is cross margined
	#[serde(rename = "7")]
	AccountIsHouseTraderAndIsCrossMargined,
	/// Joint Backoffice Account (JBO)
	#[serde(rename = "8")]
	JointBackofficeAccount,
}

impl Default for AllocAccountType {
	fn default() -> Self {
		AllocAccountType::AccountIsCarriedOnCustomerSideOfBooks
	}
}
