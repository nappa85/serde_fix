
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct QuoteAttributeGrp {
	/// NoQuoteAttributes
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2706")]
	pub quote_attributes: Option<fix_common::RepeatingValues<QuoteAttribute>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct QuoteAttribute {
	/// Required if NoQuoteAttributes(2706) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2707")]
	pub quote_attribute_type: Option<QuoteAttributeType>,
	/// Required if NoQuoteAttributes(2706) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2708")]
	pub quote_attribute_value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum QuoteAttributeType {
	/// Quote is above standard market size
	#[serde(rename = "0")]
	QuoteIsAboveStandardMarketSize,
	/// Quote is above size specific to the instrument
	#[serde(rename = "1")]
	QuoteIsAboveSizeSpecificToTheInstrument,
	/// Quote applicable for liquidity provision activity
	#[serde(rename = "2")]
	QuoteApplicableForLiquidityProvisionActivity,
	/// Quote issuer status
	#[serde(rename = "3")]
	QuoteIssuerStatus,
	/// Bid or ask request
	#[serde(rename = "4")]
	BidOrAskRequest,
}
