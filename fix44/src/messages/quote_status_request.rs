
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct QuoteStatusRequest {
	/// MsgType = a
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'a', ' '>,
	/// QuoteStatusReqID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "649")]
	pub quote_status_req_id: Option<String>,
	/// QuoteID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "117")]
	pub quote_id: Option<String>,
	/// Instrument
	#[serde(flatten)]
	pub instrument: super::super::instrument::Instrument,
	/// Number of underlyings
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "711")]
	pub underlyings: Option<fix_common::RepeatingValues<super::super::underlying_instrument::UnderlyingInstrument>>,
	/// Required for multileg quotes. For Swaps one leg is Buy and other leg is Sell
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "555")]
	pub legs: Option<fix_common::RepeatingValues<super::super::instrument_leg::InstrumentLeg>>,
	/// Parties
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// Account
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1")]
	pub account: Option<String>,
	/// AcctIDSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "660")]
	pub acct_id_source: Option<AcctIDSource>,
	/// Type of account associated with the order (Origin)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "581")]
	pub account_type: Option<AccountType>,
	/// TradingSessionID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "336")]
	pub trading_session_id: Option<String>,
	/// TradingSessionSubID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "625")]
	pub trading_session_sub_id: Option<String>,
	/// Used to subscribe for <a href="message_Quote_Status_Report_AI.html" target="main">Quote Status Report&nbsp;(AI)</a> messages
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "263")]
	pub subscription_request_type: Option<SubscriptionRequestType>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SubscriptionRequestType {
	/// Snapshot
	#[serde(rename = "0")]
	Snapshot,
	/// Snapshot + Updates (Subscribe)
	#[serde(rename = "1")]
	SnapshotUpdates,
	/// Disable previous Snapshot + Update Request (Unsubscribe)
	#[serde(rename = "2")]
	DisablePreviousSnapshotUpdateRequest,
}

impl Default for SubscriptionRequestType {
	fn default() -> Self {
		SubscriptionRequestType::Snapshot
	}
}
