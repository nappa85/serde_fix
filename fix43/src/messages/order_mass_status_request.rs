
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OrderMassStatusRequest {
	/// MsgType = AF
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'A', 'F'>,
	/// Unique ID of <a href="message_Order_Mass_Status_Request_AF.html" target="main">Order Mass Status Request&nbsp;(AF)</a> as assigned by the institution.
	#[serde(rename = "584")]
	pub mass_status_req_id: String,
	/// Specifies the scope of the <a href="message_Order_Mass_Status_Request_AF.html" target="main">Order Mass Status Request&nbsp;(AF)</a>
	#[serde(rename = "585")]
	pub mass_status_req_type: MassStatusReqType,
	/// Parties
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// Account
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1")]
	pub account: Option<String>,
	/// Trading Session
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "336")]
	pub trading_session_id: Option<String>,
	/// TradingSessionSubID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "625")]
	pub trading_session_sub_id: Option<String>,
	/// Instrument
	#[serde(flatten)]
	pub instrument: Option<super::super::instrument::Instrument>,
	/// Underlying Instrument
	#[serde(flatten)]
	pub underlying_instrument: Option<super::super::underlying_instrument::UnderlyingInstrument>,
	/// Optional qualifier used to indicate the side of the market for which orders will be returned.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "54")]
	pub side: Option<Side>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MassStatusReqType {
	/// Status for orders for a security
	#[serde(rename = "1")]
	StatusForOrdersForASecurity,
	/// Status for orders for an Underlying security
	#[serde(rename = "2")]
	StatusForOrdersForAnUnderlyingSecurity,
	/// Status for orders for a <a href="tag_460_Product.html" target="bottom">Product&nbsp;(460)</a>
	#[serde(rename = "3")]
	StatusForOrdersForAProduct,
	/// Status for orders for a <a href="tag_461_CFICode.html" target="bottom">CFICode&nbsp;(461)</a>
	#[serde(rename = "4")]
	StatusForOrdersForACfiCode,
	/// Status for orders for a <a href="tag_167_SecurityType.html" target="bottom">SecurityType&nbsp;(167)</a>
	#[serde(rename = "5")]
	StatusForOrdersForASecurityType,
	/// Status for orders for a trading session
	#[serde(rename = "6")]
	StatusForOrdersForATradingSession,
	/// Status for all orders
	#[serde(rename = "7")]
	StatusForAllOrders,
	/// Status for orders for a <a href="tag_448_PartyID.html" target="bottom">PartyID&nbsp;(448)</a>
	#[serde(rename = "8")]
	StatusForOrdersForAPartyId,
}

impl Default for MassStatusReqType {
	fn default() -> Self {
		MassStatusReqType::StatusForOrdersForASecurity
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
}

impl Default for Side {
	fn default() -> Self {
		Side::Buy
	}
}
