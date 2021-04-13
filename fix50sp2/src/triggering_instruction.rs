
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggeringInstruction {
	/// Required if any other Triggering tags are specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1100")]
	pub trigger_type: Option<TriggerType>,
	/// TriggerAction
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1101")]
	pub trigger_action: Option<TriggerAction>,
	/// Only relevant and required for <a href="tag_1101_TriggerAction.html" target="bottom">TriggerAction&nbsp;(1101)</a> = 1
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1102")]
	pub trigger_price: Option<f64>,
	/// Only relevant and required for <a href="tag_1101_TriggerAction.html" target="bottom">TriggerAction&nbsp;(1101)</a> = 1
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1103")]
	pub trigger_symbol: Option<String>,
	/// Requires <a href="tag_1105_TriggerSecurityIDSource.html" target="bottom">TriggerSecurityIDSource&nbsp;(1105)</a> if specified. Only relevant and required for <a href="tag_1101_TriggerAction.html" target="bottom">TriggerAction&nbsp;(1101)</a> = 1
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1104")]
	pub trigger_security_id: Option<String>,
	/// Requires <a href="tag_1104_TriggerSecurityID.html" target="bottom">TriggerSecurityIDSource&nbsp;(1104)</a> if specified. Only relevant and required for <a href="tag_1101_TriggerAction.html" target="bottom">TriggerAction&nbsp;(1101)</a> = 1
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1105")]
	pub trigger_security_id_source: Option<TriggerSecurityIDSource>,
	/// TriggerSecurityDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1106")]
	pub trigger_security_desc: Option<String>,
	/// Only relevant for <a href="tag_1101_TriggerAction.html" target="bottom">TriggerAction&nbsp;(1101)</a> = 1
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1107")]
	pub trigger_price_type: Option<TriggerPriceType>,
	/// Only relevant for <a href="tag_1101_TriggerAction.html" target="bottom">TriggerAction&nbsp;(1101)</a> = 1
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1108")]
	pub trigger_price_type_scope: Option<TriggerPriceTypeScope>,
	/// Only relevant for <a href="tag_1101_TriggerAction.html" target="bottom">TriggerAction&nbsp;(1101)</a> = 1
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1109")]
	pub trigger_price_direction: Option<TriggerPriceDirection>,
	/// Should be specified if the order changes Price.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1110")]
	pub trigger_new_price: Option<f64>,
	/// Should be specified if the order changes type.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1111")]
	pub trigger_order_type: Option<TriggerOrderType>,
	/// Required if the order should change quantity
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1112")]
	pub trigger_new_qty: Option<f64>,
	/// Only relevant and required for <a href="tag_1100_TriggerType.html" target="bottom">TriggerType&nbsp;(1100)</a> = 2.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1113")]
	pub trigger_trading_session_id: Option<String>,
	/// Requires <a href="tag_1113_TriggerTradingSessionID.html" target="bottom">TriggerTradingSessionID&nbsp;(1113)</a> if specified. Relevant for <a href="tag_1100_TriggerType.html" target="bottom">TriggerType&nbsp;(1100)</a> = 2 only.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1114")]
	pub trigger_trading_session_sub_id: Option<String>,
	/// TriggerScope
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1628")]
	pub trigger_scope: Option<TriggerScope>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TriggerType {
	/// Partial Execution
	#[serde(rename = "1")]
	PartialExecution,
	/// Specified Trading Session
	#[serde(rename = "2")]
	SpecifiedTradingSession,
	/// Next Auction
	#[serde(rename = "3")]
	NextAuction,
	/// Price Movement
	#[serde(rename = "4")]
	PriceMovement,
	/// On Order Entry or order modification entry
	#[serde(rename = "5")]
	OnOrderEntryOrOrderModificationEntry,
}

impl Default for TriggerType {
	fn default() -> Self {
		TriggerType::PartialExecution
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TriggerAction {
	/// Activate
	#[serde(rename = "1")]
	Activate,
	/// Modify
	#[serde(rename = "2")]
	Modify,
	/// Cancel
	#[serde(rename = "3")]
	Cancel,
}

impl Default for TriggerAction {
	fn default() -> Self {
		TriggerAction::Activate
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TriggerSecurityIDSource {
	/// CUSIP
	#[serde(rename = "1")]
	Cusip,
	/// SEDOL
	#[serde(rename = "2")]
	Sedol,
	/// QUIK
	#[serde(rename = "3")]
	Quik,
	/// ISIN
	#[serde(rename = "4")]
	Isin,
	/// RIC
	#[serde(rename = "5")]
	Ric,
	/// ISO Currency Code
	#[serde(rename = "6")]
	IsoCurrencyCode,
	/// ISO Country Code
	#[serde(rename = "7")]
	IsoCountryCode,
	/// Exchange Symbol
	#[serde(rename = "8")]
	ExchangeSymbol,
	/// Consolidated Tape Association (CTA) Symbol (SIAC CTS/CQS line format)
	#[serde(rename = "9")]
	ConsolidatedTapeAssociationSymbol,
	/// Bloomberg Symbol
	#[serde(rename = "A")]
	BloombergSymbol,
	/// Wertpapier
	#[serde(rename = "B")]
	Wertpapier,
	/// Dutch
	#[serde(rename = "C")]
	Dutch,
	/// Valoren
	#[serde(rename = "D")]
	Valoren,
	/// Sicovam
	#[serde(rename = "E")]
	Sicovam,
	/// Belgian
	#[serde(rename = "F")]
	Belgian,
	/// "Common" (Clearstream and Euroclear)
	#[serde(rename = "G")]
	Common,
	/// Clearing House / Clearing Organization
	#[serde(rename = "H")]
	ClearingHouseClearingOrganization,
	/// ISDA/FpML Product Specification
	#[serde(rename = "I")]
	IsdaFpMlProductSpecification,
	/// Option Price Reporting Authority
	#[serde(rename = "J")]
	OptionPriceReportingAuthority,
	/// ISDA/FpML Product URL (URL in SecurityID)
	#[serde(rename = "K")]
	IsdaFpMlProductUrl,
	/// Letter of Credit
	#[serde(rename = "L")]
	LetterOfCredit,
	/// Marketplace-assigned Identifier
	#[serde(rename = "M")]
	MarketplaceAssignedIdentifier,
	/// Markit RED entity CLIP
	#[serde(rename = "N")]
	MarkitRedEntityClip,
	/// Markit RED pair CLIP
	#[serde(rename = "P")]
	MarkitRedPairClip,
	/// CFTC commodity code
	#[serde(rename = "Q")]
	CftcCommodityCode,
	/// ISDA Commodity Reference Price
	#[serde(rename = "R")]
	IsdaCommodityReferencePrice,
	/// Financial Instrument Global Identifier
	#[serde(rename = "S")]
	FinancialInstrumentGlobalIdentifier,
	/// Legal Entity Identifier
	#[serde(rename = "T")]
	LegalEntityIdentifier,
	/// Synthetic
	#[serde(rename = "U")]
	Synthetic,
	/// Fidessa Instrument Mnemonic (FIM)
	#[serde(rename = "V")]
	FidessaInstrumentMnemonic,
	/// Index name
	#[serde(rename = "W")]
	IndexName,
	/// Uniform Symbol (UMTF Symbol)
	#[serde(rename = "X")]
	UniformSymbol,
}

impl Default for TriggerSecurityIDSource {
	fn default() -> Self {
		TriggerSecurityIDSource::Cusip
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TriggerPriceType {
	/// Best Offer
	#[serde(rename = "1")]
	BestOffer,
	/// Last Trade
	#[serde(rename = "2")]
	LastTrade,
	/// Best Bid
	#[serde(rename = "3")]
	BestBid,
	/// Best Bid or Last Trade
	#[serde(rename = "4")]
	BestBidOrLastTrade,
	/// Best Offer or Last Trade
	#[serde(rename = "5")]
	BestOfferOrLastTrade,
	/// Best Mid
	#[serde(rename = "6")]
	BestMid,
}

impl Default for TriggerPriceType {
	fn default() -> Self {
		TriggerPriceType::BestOffer
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TriggerPriceTypeScope {
	/// None
	#[serde(rename = "0")]
	None,
	/// Local (Exchange, ECN, ATS)
	#[serde(rename = "1")]
	Local,
	/// National (Across all national markets)
	#[serde(rename = "2")]
	National,
	/// Global (Across all markets)
	#[serde(rename = "3")]
	Global,
}

impl Default for TriggerPriceTypeScope {
	fn default() -> Self {
		TriggerPriceTypeScope::None
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TriggerPriceDirection {
	/// Trigger if the price of the specified type goes UP to or through the specified Trigger Price.
	#[serde(rename = "U")]
	TriggerIfThePriceOfTheSpecifiedTypeGoesUpToOrThroughTheSpecifiedTriggerPrice,
	/// Trigger if the price of the specified type goes DOWN to or through the specified Trigger Price.
	#[serde(rename = "D")]
	TriggerIfThePriceOfTheSpecifiedTypeGoesDownToOrThroughTheSpecifiedTriggerPrice,
}

impl Default for TriggerPriceDirection {
	fn default() -> Self {
		TriggerPriceDirection::TriggerIfThePriceOfTheSpecifiedTypeGoesUpToOrThroughTheSpecifiedTriggerPrice
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TriggerOrderType {
	/// Market
	#[serde(rename = "1")]
	Market,
	/// Limit
	#[serde(rename = "2")]
	Limit,
}

impl Default for TriggerOrderType {
	fn default() -> Self {
		TriggerOrderType::Market
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TriggerScope {
	/// This order (default)
	#[serde(rename = "0")]
	ThisOrder,
	/// Other order (use RefId)
	#[serde(rename = "1")]
	OtherOrder,
	/// All other orders for the given security
	#[serde(rename = "2")]
	AllOtherOrdersForTheGivenSecurity,
	/// All other orders for the given security and price
	#[serde(rename = "3")]
	AllOtherOrdersForTheGivenSecurityAndPrice,
	/// All other orders for the given security and side
	#[serde(rename = "4")]
	AllOtherOrdersForTheGivenSecurityAndSide,
	/// All other orders for the given security, price and side
	#[serde(rename = "5")]
	AllOtherOrdersForTheGivenSecurityPriceAndSide,
}

impl Default for TriggerScope {
	fn default() -> Self {
		TriggerScope::ThisOrder
	}
}
