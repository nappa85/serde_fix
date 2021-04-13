
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Order {
	/// MsgType = 9
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// If CxlRejReason="Unknown order", specify "NONE".
	#[serde(rename = "37")]
	pub order_id: String,
	/// Can be used to provide order id used by exchange or executing system.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "198")]
	pub secondary_order_id: Option<String>,
	/// SecondaryClOrdID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "526")]
	pub secondary_cl_ord_id: Option<String>,
	/// Unique order id assigned by institution or by the intermediary with closest association with the investor. to the cancel request
	/// or to the replacement order.
	#[serde(rename = "11")]
	pub cl_ord_id: String,
	/// ClOrdLinkID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "583")]
	pub cl_ord_link_id: Option<String>,
	/// ClOrdID which could not be canceled/replaced. <a href="tag_11_ClOrdID.html" target="bottom">ClOrdID&nbsp;(11)</a> of the previous accepted order (NOT the initial order of the day) when canceling or replacing an order.
	#[serde(rename = "41")]
	pub orig_cl_ord_id: String,
	/// OrdStatus value after this cancel reject is applied. " If <a href="tag_102_CxlRejReason.html" target="bottom">CxlRejReason&nbsp;(102)</a> = "Unknown Order", specify Rejected."
	#[serde(rename = "39")]
	pub ord_status: OrdStatus,
	/// For optional use with <a href="tag_39_OrdStatus.html" target="bottom">OrdStatus&nbsp;(39)</a> = 0 (New)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "636")]
	pub working_indicator: Option<WorkingIndicator>,
	/// OrigOrdModTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "586")]
	pub orig_ord_mod_time: Option<fix_common::UTCTimestamp>,
	/// Required for rejects against orders which were submitted as part of a list.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "66")]
	pub list_id: Option<String>,
	/// Account
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1")]
	pub account: Option<String>,
	/// AcctIDSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "660")]
	pub acct_id_source: Option<AcctIDSource>,
	/// AccountType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "581")]
	pub account_type: Option<AccountType>,
	/// TradeOriginationDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "229")]
	pub trade_origination_date: Option<fix_common::LocalMktDate>,
	/// TradeDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "75")]
	pub trade_date: Option<fix_common::LocalMktDate>,
	/// TransactTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<fix_common::UTCTimestamp>,
	/// CxlRejResponseTo
	#[serde(rename = "434")]
	pub cxl_rej_response_to: CxlRejResponseTo,
	/// CxlRejReason
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "102")]
	pub cxl_rej_reason: Option<CxlRejReason>,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if <a href="tag_355_EncodedText.html" target="bottom">EncodedText&nbsp;(355)</a> field is specified and must immediately precede it.
	#[serde(rename = "354")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_58_Text.html" target="bottom">Text&nbsp;(58)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AcctIDSource {
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

impl Default for AcctIDSource {
	fn default() -> Self {
		AcctIDSource::Bic
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AccountType {
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

impl Default for AccountType {
	fn default() -> Self {
		AccountType::AccountIsCarriedOnCustomerSideOfBooks
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CxlRejResponseTo {
	/// Order cancel request
	#[serde(rename = "1")]
	OrderCancelRequest,
	/// Order cancel/replace request
	#[serde(rename = "2")]
	OrderCancelReplaceRequest,
}

impl Default for CxlRejResponseTo {
	fn default() -> Self {
		CxlRejResponseTo::OrderCancelRequest
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CxlRejReason {
	/// Too late to cancel
	#[serde(rename = "0")]
	TooLateToCancel,
	/// Unknown order
	#[serde(rename = "1")]
	UnknownOrder,
	/// Broker / Exchange Option
	#[serde(rename = "2")]
	BrokerExchangeOption,
	/// Order already in Pending Cancel or Pending Replace status
	#[serde(rename = "3")]
	OrderAlreadyInPendingCancelOrPendingReplaceStatus,
	/// Unable to process Order Mass Cancel Request
	#[serde(rename = "4")]
	UnableToProcessOrderMassCancelRequest,
	/// <a href="tag_586_OrigOrdModTime.html" target="bottom">OrigOrdModTime&nbsp;(586)</a> did not match last <a href="tag_60_TransactTime.html" target="bottom">TransactTime&nbsp;(60)</a> of order
	#[serde(rename = "5")]
	AHrefTag586OrigOrdModTimeHtmlTargetBottomOrigOrdModTimeNbspADidNotMatchLastAHrefTag60TransactTimeHtmlTargetBottomTransactTimeNbspAOfOrder,
	/// Duplicate <a href="tag_11_ClOrdID.html" target="bottom">ClOrdID&nbsp;(11)</a> received
	#[serde(rename = "6")]
	DuplicateAHrefTag11ClOrdIdHtmlTargetBottomClOrdIdNbspAReceived,
	/// Invalid price increment
	#[serde(rename = "18")]
	InvalidPriceIncrement,
	/// Other
	#[serde(rename = "99")]
	Other,
}

impl Default for CxlRejReason {
	fn default() -> Self {
		CxlRejReason::TooLateToCancel
	}
}
