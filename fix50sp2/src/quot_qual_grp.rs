
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct QuotQualGrp {
	/// NoQuoteQualifiers
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "735")]
	pub quote_qualifiers: Option<fix_common::RepeatingValues<QuoteQualifier>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct QuoteQualifier {
	/// Required if NoQuoteQualifiers &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "695")]
	pub quote_qualifier_item: Option<QuoteQualifierItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum QuoteQualifierItem {
	/// All or none
	#[serde(rename = "A")]
	AllOrNone,
	/// Market On Close (MOC) (held to close)
	#[serde(rename = "B")]
	MarketOnClose,
	/// At the close (around/not held to close)
	#[serde(rename = "C")]
	AtTheClose,
	/// VWAP (Volume Weighted Avg Price)
	#[serde(rename = "D")]
	Vwap,
	/// Axe (indicates that a quote is an Axe, without specifying a side preference. Mutually exclusive with F(Axe on bid) and G (Axe
	/// on offer))
	#[serde(rename = "E")]
	AxeAndG,
	/// Axe on bid (indicates that a quote is an Axe, with a preference to execute on the bid side. Mutually exclusive with E(Axe)
	/// and G (Axe on offer))
	#[serde(rename = "F")]
	AxeOnBidAndG,
	/// Axe on offer (indicates that a quote is an Axe, with a preference to execute on the offer side. Mutually exclusive with E(Axe)
	/// and F (Axe on bid))
	#[serde(rename = "G")]
	AxeOnOfferAndF,
	/// In touch with
	#[serde(rename = "I")]
	InTouchWith,
	/// Limit
	#[serde(rename = "L")]
	Limit,
	/// More behind
	#[serde(rename = "M")]
	MoreBehind,
	/// At the open
	#[serde(rename = "O")]
	AtTheOpen,
	/// Taking a position
	#[serde(rename = "P")]
	TakingAPosition,
	/// At the Market (previously called Current Quote)
	#[serde(rename = "Q")]
	AtTheMarket,
	/// Ready to trade
	#[serde(rename = "R")]
	ReadyToTrade,
	/// Portfolio shown
	#[serde(rename = "S")]
	PortfolioShown,
	/// Through the day
	#[serde(rename = "T")]
	ThroughTheDay,
	/// Versus
	#[serde(rename = "V")]
	Versus,
	/// Indication - Working away
	#[serde(rename = "W")]
	IndicationWorkingAway,
	/// Crossing opportunity
	#[serde(rename = "X")]
	CrossingOpportunity,
	/// At the Midpoint
	#[serde(rename = "Y")]
	AtTheMidpoint,
	/// Pre-open
	#[serde(rename = "Z")]
	PreOpen,
	/// Outside spread
	#[serde(rename = "c")]
	OutsideSpread,
	/// Client natural block
	#[serde(rename = "N")]
	ClientNaturalBlock,
	/// Client natural working
	#[serde(rename = "H")]
	ClientNaturalWorking,
	/// Unwind
	#[serde(rename = "U")]
	Unwind,
	/// Position wanted
	#[serde(rename = "J")]
	PositionWanted,
	/// Market making
	#[serde(rename = "K")]
	MarketMaking,
	/// Quantity is negotiable
	#[serde(rename = "1")]
	QuantityIsNegotiable,
	/// Allow late bids
	#[serde(rename = "2")]
	AllowLateBids,
	/// Immediate or counter
	#[serde(rename = "3")]
	ImmediateOrCounter,
	/// Auto trade
	#[serde(rename = "4")]
	AutoTrade,
	/// Automatic spot
	#[serde(rename = "a")]
	AutomaticSpot,
	/// Platform calculated spot
	#[serde(rename = "b")]
	PlatformCalculatedSpot,
	/// Deferred spot
	#[serde(rename = "d")]
	DeferredSpot,
	/// Negotiated spot
	#[serde(rename = "n")]
	NegotiatedSpot,
}

impl Default for QuoteQualifierItem {
	fn default() -> Self {
		QuoteQualifierItem::AllOrNone
	}
}
