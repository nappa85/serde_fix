
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PegInstructions {
	/// Amount (signed) added to the peg for a pegged order in the context of the PegOffsetType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "211")]
	pub peg_offset_value: Option<f64>,
	/// Defines the type of peg.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1094")]
	pub peg_price_type: Option<PegPriceType>,
	/// Describes whether peg is static/fixed or floats
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "835")]
	pub peg_move_type: Option<PegMoveType>,
	/// Type of Peg Offset (e.g. price offset, tick offset etc)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "836")]
	pub peg_offset_type: Option<PegOffsetType>,
	/// Specifies nature of resulting pegged price (e.g. or better limit, strict limit etc)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "837")]
	pub peg_limit_type: Option<PegLimitType>,
	/// If the calculated peg price is not a valid tick price, specifies how to round the price (e.g. be more or less aggressive)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "838")]
	pub peg_round_direction: Option<PegRoundDirection>,
	/// The scope of the "related to" price of the peg (e.g. local, global etc)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "840")]
	pub peg_scope: Option<PegScope>,
	/// Required if <a href="tag_1097_PegSecurityID.html" target="bottom">PegSecurityID&nbsp;(1097)</a> is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1096")]
	pub peg_security_id_source: Option<PegSecurityIDSource>,
	/// Requires <a href="tag_1096_PegSecurityIDSource.html" target="bottom">PegSecurityIDSource&nbsp;(1096)</a> if specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1097")]
	pub peg_security_id: Option<String>,
	/// PegSymbol
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1098")]
	pub peg_symbol: Option<String>,
	/// PegSecurityDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1099")]
	pub peg_security_desc: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PegPriceType {
	/// Last peg (last sale)
	#[serde(rename = "1")]
	LastPeg,
	/// Mid-price peg (midprice of inside quote)
	#[serde(rename = "2")]
	MidPricePeg,
	/// Opening peg
	#[serde(rename = "3")]
	OpeningPeg,
	/// Market peg
	#[serde(rename = "4")]
	MarketPeg,
	/// Primary peg (primary market - buy at bid or sell at offer)
	#[serde(rename = "5")]
	PrimaryPeg,
	/// Peg to VWAP
	#[serde(rename = "7")]
	PegToVwap,
	/// Trailing Stop Peg
	#[serde(rename = "8")]
	TrailingStopPeg,
	/// Peg to Limit Price
	#[serde(rename = "9")]
	PegToLimitPrice,
	/// Short sale minimum price peg
	#[serde(rename = "10")]
	ShortSaleMinimumPricePeg,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PegMoveType {
	/// Floating (default)
	#[serde(rename = "0")]
	Floating,
	/// Fixed
	#[serde(rename = "1")]
	Fixed,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PegOffsetType {
	/// Price (default)
	#[serde(rename = "0")]
	Price,
	/// Basis Points
	#[serde(rename = "1")]
	BasisPoints,
	/// Ticks
	#[serde(rename = "2")]
	Ticks,
	/// Price Tier / Level
	#[serde(rename = "3")]
	PriceTierLevel,
	/// Percentage
	#[serde(rename = "4")]
	Percentage,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PegLimitType {
	/// Or better (default) - price improvement allowed
	#[serde(rename = "0")]
	OrBetterPriceImprovementAllowed,
	/// Strict - limit is a strict limit
	#[serde(rename = "1")]
	StrictLimitIsAStrictLimit,
	/// Or worse - for a buy the peg limit is a minimum and for a sell the peg limit is a maximum (for use for orders which have a
	/// price range)
	#[serde(rename = "2")]
	OrWorseForABuyThePegLimitIsAMinimumAndForASellThePegLimitIsAMaximum,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PegRoundDirection {
	/// More aggressive - on a buy order round the price up to the nearest tick; on a sell order round down to the nearest tick
	#[serde(rename = "1")]
	MoreAggressiveOnABuyOrderRoundThePriceUpToTheNearestTickOnASellOrderRoundDownToTheNearestTick,
	/// More passive - on a buy order round down to the nearest tick; on a sell order round up to the nearest tick
	#[serde(rename = "2")]
	MorePassiveOnABuyOrderRoundDownToTheNearestTickOnASellOrderRoundUpToTheNearestTick,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PegScope {
	/// Local (Exchange, ECN, ATS)
	#[serde(rename = "1")]
	Local,
	/// National
	#[serde(rename = "2")]
	National,
	/// Global
	#[serde(rename = "3")]
	Global,
	/// National excluding local
	#[serde(rename = "4")]
	NationalExcludingLocal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PegSecurityIDSource {
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
