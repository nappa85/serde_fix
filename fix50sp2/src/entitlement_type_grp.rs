
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EntitlementTypeGrp {
	/// Number of Entitlement Types.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2345")]
	pub entitlement_types: Option<fix_common::RepeatingValues<EntitlementType>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EntitlementType {
	/// Required if <a href="tag_2345_NoEntitlementTypes.html" target="bottom">NoEntitlementTypes&nbsp;(2345)</a> &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1775")]
	pub entitlement_type_item: Option<EntitlementTypeItem>,
	/// EntitlementSubType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2402")]
	pub entitlement_sub_type: Option<EntitlementSubType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EntitlementTypeItem {
	/// Trade
	#[serde(rename = "0")]
	Trade,
	/// Make markets
	#[serde(rename = "1")]
	MakeMarkets,
	/// Hold positions
	#[serde(rename = "2")]
	HoldPositions,
	/// Perform give-ups
	#[serde(rename = "3")]
	PerformGiveUps,
	/// Submit Indications of Interest (IOIs)
	#[serde(rename = "4")]
	SubmitIndicationsOfInterest,
	/// Subscribe to market data
	#[serde(rename = "5")]
	SubscribeToMarketData,
	/// Short with pre-borrow
	#[serde(rename = "6")]
	ShortWithPreBorrow,
	/// Submit quote requests
	#[serde(rename = "7")]
	SubmitQuoteRequests,
	/// Respond to quote requests
	#[serde(rename = "8")]
	RespondToQuoteRequests,
}

impl Default for EntitlementTypeItem {
	fn default() -> Self {
		EntitlementTypeItem::Trade
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EntitlementSubType {
	/// Order Entry(Entitle to enter new orders)
	#[serde(rename = "1")]
	OrderEntry,
	/// Hit/Lift (Entitle to Hit/Lift)
	#[serde(rename = "2")]
	HitLift,
	/// View indicative prices (Entitle to subscribe to indicative prices)
	#[serde(rename = "3")]
	ViewIndicativePrices,
	/// View executable prices (Entitle to subscribe to executable prices)
	#[serde(rename = "4")]
	ViewExecutablePrices,
	/// Single quote (Entitle to submit quote request for a single quote)
	#[serde(rename = "5")]
	SingleQuote,
	/// Streaming quotes (Entitle to submit quote request for streaming quotes)
	#[serde(rename = "6")]
	StreamingQuotes,
	/// Single broker (Entitle to submit quote request for a single broker)
	#[serde(rename = "7")]
	SingleBroker,
	/// Multi brokers (Entitle to submit quote request for multiple brokers)
	#[serde(rename = "8")]
	MultiBrokers,
}

impl Default for EntitlementSubType {
	fn default() -> Self {
		EntitlementSubType::OrderEntry
	}
}
