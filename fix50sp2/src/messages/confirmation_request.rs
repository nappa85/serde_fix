
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Confirmation {
	/// MsgType = BH
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// Unique identifier for this message
	#[serde(rename = "859")]
	pub confirm_req_id: String,
	/// Denotes whether this message is being used to request a confirmation or a trade status message
	#[serde(rename = "773")]
	pub confirm_type: ConfirmType,
	/// Indicates number of orders to be combined for allocation. If order(s) were manually delivered set to 1 (one).Required when <a href="tag_857_AllocNoOrdersType.html" target="bottom">AllocNoOrdersType&nbsp;(857)</a> = 1
	#[serde(flatten)]
	pub ord_alloc_grp: Option<super::super::ord_alloc_grp::OrdAllocGrp>,
	/// Used to refer to an earlier Allocation Instruction.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "70")]
	pub alloc_id: Option<String>,
	/// Used to refer to an earlier <a href="message_AllocationInstruction_J.html" target="main">Allocation Instruction&nbsp;(J)</a> via its secondary identifier
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "793")]
	pub secondary_alloc_id: Option<String>,
	/// Used to refer to an allocation account within an earlier Allocation Instruction.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "467")]
	pub individual_alloc_id: Option<String>,
	/// Represents the time this message was generated
	#[serde(rename = "60")]
	pub transact_time: crate::entities::UTCTimestamp,
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
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "354")]
	pub encoded_text_len: Option<usize>,
	/// EncodedText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "355")]
	pub encoded_text: Option<String>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
	/// Special Segregated Account ID
	#[serde(rename = "6")]
	SpecialSegregatedAccountId,
	/// Other (custom or proprietary)
	#[serde(rename = "99")]
	Other,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
