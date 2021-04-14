
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CrossOrderCancelRequest {
	/// MsgType = u
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
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
	/// Insert here the set of "Root Parties" fields defined in "common components of application messages". Used for acting parties
	/// that applies to the whole message, not individual sides.
	#[serde(flatten)]
	pub root_parties: Option<super::super::root_parties::RootParties>,
	/// Must be 1 or 2
	#[serde(flatten)]
	pub side_cross_ord_cxl_grp: super::super::side_cross_ord_cxl_grp::SideCrossOrdCxlGrp,
	/// Insert here the set of "Instrument" fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub instrument: super::super::instrument::Instrument,
	/// Number of underlyings.
	#[serde(flatten)]
	pub und_instrmt_grp: Option<super::super::und_instrmt_grp::UndInstrmtGrp>,
	/// Number of Leg.
	#[serde(flatten)]
	pub instrmt_leg_grp: Option<super::super::instrmt_leg_grp::InstrmtLegGrp>,
	/// Time this order request was initiated/released by the trader, trading system, or intermediary.
	#[serde(rename = "60")]
	pub transact_time: fix_common::UTCTimestamp,
	/// MarketSegmentID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1300")]
	pub market_segment_id: Option<String>,
	/// Required if provided on the order being cancelled. Echo back the value provided by the requester.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2422")]
	pub order_request_id: Option<i32>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CrossType {
	/// Cross AON - cross tade which is executed complete or not. Both sides are treated in the same manner. This is equivalent to
	/// an "All or None"."
	#[serde(rename = "1")]
	CrossAonCrossTadeWhichIsExecutedCompleteOrNotBothSidesAreTreatedInTheSameMannerThisIsEquivalentToAnAllOrNone,
	/// Cross IOC - cross trade which is executed partially and the rest is cancelled. One side is fully executed, the other side
	/// is partially executed with the remainder being cancelled. This is equivalent to an IOC on the other side. Note: <a href="tag_550_CrossPrioritization.html" target="bottom">CrossPrioritization&nbsp;(550)</a> field may be used to indicate which side should fully execute in this scenario.
	#[serde(rename = "2")]
	CrossIocCrossTradeWhichIsExecutedPartiallyAndTheRestIsCancelledOneSideIsFullyExecutedTheOtherSideIsPartiallyExecutedWithTheRemainderBeingCancelledThisIsEquivalentToAnIocOnTheOtherSideNoteAHrefTag550CrossPrioritizationHtmlTargetBottomCrossPrioritizationNbspAFieldMayBeUsedToIndicateWhichSideShouldFullyExecuteInThisScenario,
	/// Cross One Side - cross trade which is partially executed with the unfilled portions remaining active. One side of the corss
	/// is fully executed (as denoted by the <a href="tag_550_CrossPrioritization.html" target="bottom">CrossPrioritization&nbsp;(550)</a> field), but the unfilled portion remains active.
	#[serde(rename = "3")]
	CrossOneSideCrossTradeWhichIsPartiallyExecutedWithTheUnfilledPortionsRemainingActiveOneSideOfTheCorssIsFullyExecutedAFieldButTheUnfilledPortionRemainsActive,
	/// Cross Same Price - cross trade is executed with existing orders with the same price. In this case other orders exist with
	/// the same price, the quantity of the Cross is executed against the existing orders and quotes, the remainder of the corss is
	/// executed against the other side of the cross. The two sides potentially have different quantities.
	#[serde(rename = "4")]
	CrossSamePriceCrossTradeIsExecutedWithExistingOrdersWithTheSamePriceInThisCaseOtherOrdersExistWithTheSamePriceTheQuantityOfTheCrossIsExecutedAgainstTheExistingOrdersAndQuotesTheRemainderOfTheCorssIsExecutedAgainstTheOtherSideOfTheCrossTheTwoSidesPotentiallyHaveDifferentQuantities,
	/// Basis Cross (A trade where a basket of securities or an index participation unit is transacted at prices achieved through
	/// the execution of related exchange-traded derivative instruments in an amount that will correspond to an equivalent market
	/// exposure)
	#[serde(rename = "5")]
	BasisCross,
	/// Contingent Cross (A cross resulting from a paired order placed by a Participant to execute an order on a security that is
	/// contingent on the execution of a second order for an offsetting volume of a related security)
	#[serde(rename = "6")]
	ContingentCross,
	/// VWAP Cross (A cross for the purpose of executing a trade at a volume-weighted average price of a security traded for a continuous
	/// period on or during a trading day)
	#[serde(rename = "7")]
	VwapCross,
	/// STS Cross (A closing price cross resulting from an order placed by a Participant for execution in a Special Trading Session
	/// at the last sale price)
	#[serde(rename = "8")]
	StsCross,
	/// Customer to customer cross
	#[serde(rename = "9")]
	CustomerToCustomerCross,
}

impl Default for CrossType {
	fn default() -> Self {
		CrossType::CrossAonCrossTadeWhichIsExecutedCompleteOrNotBothSidesAreTreatedInTheSameMannerThisIsEquivalentToAnAllOrNone
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
