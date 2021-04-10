
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AuctionTypeRuleGrp {
	/// NoAuctionTypeRules
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2548")]
	pub auction_type_rules: Option<fix_common::RepeatingValues<AuctionTypeRule>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AuctionTypeRule {
	/// Required if NoAuctionTypeRules(2548) &gt; 0. AuctionType(1803) = 0 (None) can be used to invalidate all auction types on the
	/// instrument level that are defined on a market segment level.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1803")]
	pub auction_type: Option<AuctionType>,
	/// Can be used to limit auction order type to specific product suite. Use multiple entries with the same AuctionType(1803) if
	/// multiple but not all product suites are supported.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2549")]
	pub auction_type_product_complex: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AuctionType {
	/// None
	#[serde(rename = "0")]
	None,
	/// Block order auction
	#[serde(rename = "1")]
	BlockOrderAuction,
	/// Directed order auction
	#[serde(rename = "2")]
	DirectedOrderAuction,
	/// Exposure order auction
	#[serde(rename = "3")]
	ExposureOrderAuction,
	/// Flash order auction
	#[serde(rename = "4")]
	FlashOrderAuction,
	/// Facilitation order auction
	#[serde(rename = "5")]
	FacilitationOrderAuction,
	/// Solicitation order auction
	#[serde(rename = "6")]
	SolicitationOrderAuction,
	/// Price improvement mechanism
	#[serde(rename = "7")]
	PriceImprovementMechanism,
	/// Directed order price improvement mechanism
	#[serde(rename = "8")]
	DirectedOrderPriceImprovementMechanism,
}
