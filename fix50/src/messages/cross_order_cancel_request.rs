
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CrossOrderCancelRequest {
	/// MsgType = u
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'u', ' '>,
	/// Unique identifier of most recent order as assigned by sell-side (broker, exchange, ECN).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "37")]
	pub order_id: Option<String>,
	/// CrossID for the replacement order
	#[serde(rename = "548")]
	pub cross_id: String,
	/// Must match the <a href="tag_548_CrossID.html" target="bottom">CrossID&nbsp;(548)</a> of previous cross order. Same order chaining mechanism as ClOrdID/OrigClOrdID with single order Cancel/Replace.
	#[serde(rename = "551")]
	pub orig_cross_id: String,
	/// Host assigned entity ID that can be used to reference all components of a cross; sides + strategy + legs.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "961")]
	pub host_cross_id: Option<String>,
	/// CrossType
	#[serde(rename = "549")]
	pub cross_type: CrossType,
	/// CrossPrioritization
	#[serde(rename = "550")]
	pub cross_prioritization: CrossPrioritization,
	/// Insert here the set of "Root Parties" fields defined in "common components of application messages" Used for acting parties
	/// that applies to the whole message, not individual sides.
	#[serde(flatten)]
	pub root_parties: Option<super::super::root_parties::RootParties>,
	/// Must be 1 or 2
	#[serde(rename = "552")]
	pub no_sides: NoSides,
	/// Insert here the set of "Instrument" fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub instrument: super::super::instrument::Instrument,
	/// Number of underlyings
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "711")]
	pub underlyings: Option<fix_common::RepeatingValues<super::super::underlying_instrument::UnderlyingInstrument>>,
	/// Number of legs Identifies a Multi-leg Execution if present and non-zero.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "555")]
	pub legs: Option<fix_common::RepeatingValues<super::super::instrument_leg::InstrumentLeg>>,
	/// Time this order request was initiated/released by the trader, trading system, or intermediary.
	#[serde(rename = "60")]
	pub transact_time: fix_common::UTCTimestamp,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NoSide {
	/// Side
	#[serde(rename = "54")]
	pub side: Side,
	/// OrigClOrdID
	#[serde(rename = "41")]
	pub orig_cl_ord_id: String,
	/// Unique identifier of the order as assigned by institution or by the intermediary with closest association with the investor.
	#[serde(rename = "11")]
	pub cl_ord_id: String,
	/// SecondaryClOrdID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "526")]
	pub secondary_cl_ord_id: Option<String>,
	/// ClOrdLinkID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "583")]
	pub cl_ord_link_id: Option<String>,
	/// OrigOrdModTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "586")]
	pub orig_ord_mod_time: Option<fix_common::UTCTimestamp>,
	/// TradeOriginationDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "229")]
	pub trade_origination_date: Option<fix_common::LocalMktDate>,
	/// TradeDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "75")]
	pub trade_date: Option<fix_common::LocalMktDate>,
	/// ComplianceID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "376")]
	pub compliance_id: Option<String>,
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
}





#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum CrossType {
	/// Cross AON - cross tade which is executed complete or not. Both sides are treated in the same manner. This is equivalent to
	/// an "All or None".
	#[serde(rename = "1")]
	CrossAonCrossTadeWhichIsExecutedCompleteOrNotBothSidesAreTreatedInTheSameMannerThisIsEquivalentToAnAllOrNone,
	/// Cross IOC - cross trade which is executed partially and the rest is cancelled. One side is fully executed, the other side
	/// is partially executed with the remainder being cancelled. This is equivalent to an IOC on the other side. Note: <a href="tag_550_CrossPrioritization.html" target="bottom">CrossPrioritization&nbsp;(550)</a> field may be used to indicate which side should fully execute in this scenario.
	#[serde(rename = "2")]
	CrossIocCrossTradeWhichIsExecutedPartiallyAndTheRestIsCancelledOneSideIsFullyExecutedTheOtherSideIsPartiallyExecutedWithTheRemainderBeingCancelledThisIsEquivalentToAnIocOnTheOtherSideNoteCrossPrioritizationFieldMayBeUsedToIndicateWhichSideShouldFullyExecuteInThisScenario,
	/// Cross One Side - cross trade which is partially executed with the unfilled portions remaining active. One side of the corss
	/// is fully executed (as denoted by the <a href="tag_550_CrossPrioritization.html" target="bottom">CrossPrioritization&nbsp;(550)</a> field), but the unfilled portion remains active.
	#[serde(rename = "3")]
	CrossOneSideCrossTradeWhichIsPartiallyExecutedWithTheUnfilledPortionsRemainingActiveOneSideOfTheCorssIsFullyExecutedAFieldButTheUnfilledPortionRemainsActive,
	/// Cross Same Price - cross trade is executed with existing orders with the same price. In this case other orders exist with
	/// the same price, the quantity of the Cross is executed against the existing orders and quotes, the remainder of the corss is
	/// executed against the other side of the cross. The two sides potentially have different quantities.
	#[serde(rename = "4")]
	CrossSamePriceCrossTradeIsExecutedWithExistingOrdersWithTheSamePriceInThisCaseOtherOrdersExistWithTheSamePriceTheQuantityOfTheCrossIsExecutedAgainstTheExistingOrdersAndQuotesTheRemainderOfTheCorssIsExecutedAgainstTheOtherSideOfTheCrossTheTwoSidesPotentiallyHaveDifferentQuantities,
}

impl Default for CrossType {
	fn default() -> Self {
		CrossType::CrossAonCrossTadeWhichIsExecutedCompleteOrNotBothSidesAreTreatedInTheSameMannerThisIsEquivalentToAnAllOrNone
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum CrossPrioritization {
	/// None
	#[serde(rename = "0")]
	None,
	/// Buy side is prioritized
	#[serde(rename = "1")]
	BuySideIsPrioritized,
	/// Sell side is prioritized
	#[serde(rename = "2")]
	SellSideIsPrioritized,
}

impl Default for CrossPrioritization {
	fn default() -> Self {
		CrossPrioritization::None
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum NoSides {
	/// One Side
	#[serde(rename = "1")]
	OneSide,
	/// Both Sides
	#[serde(rename = "2")]
	BothSides,
}

impl Default for NoSides {
	fn default() -> Self {
		NoSides::OneSide
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
