
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MarketDisruptionFallbackReferencePriceGrp {
	/// NoMarketDisruptionFallbackReferencePrices
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41096")]
	pub market_disruption_fallback_reference_prices: Option<fix_common::RepeatingValues<MarketDisruptionFallbackReferencePrice>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MarketDisruptionFallbackReferencePrice {
	/// Required if NoMarketDisruptionFallbackReferencePrices(41096) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41097")]
	pub market_disruption_fallback_underlier_type: Option<MarketDisruptionFallbackUnderlierType>,
	/// Conditionally required when MarketDisruptionFallbackUnderlyerSecurityIDSource(41099) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41098")]
	pub market_disruption_fallback_underlier_security_id: Option<String>,
	/// Conditionally required when MarketDisruptionFallbackUnderlierSecurityID(41098) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41099")]
	pub market_disruption_fallback_underlier_security_id_source: Option<MarketDisruptionFallbackUnderlierSecurityIDSource>,
	/// MarketDisruptionFallbackUnderlierSecurityDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41100")]
	pub market_disruption_fallback_underlier_security_desc: Option<String>,
	/// Must be set if EncodedMarketDisruptionFallbackUnderlierSecurityDesc(41102) field is specified and must immediately precede
	/// it
	#[serde(rename = "41101")]
	/// Encoded (non-ASCII characters) representation of the MarketDisruptionFallbackUnderlierSecurityDesc(41100) field in the encoded
	/// format specified via the MessageEncoding(347) field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "41102")]
	pub encoded_market_disruption_fallback_underlier_security_desc: Option<fix_common::EncodedText<41102>>,
	/// MarketDisruptionFallbackOpenUnits
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41103")]
	pub market_disruption_fallback_open_units: Option<f64>,
	/// MarketDisruptionFallbackBasketCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41104")]
	pub market_disruption_fallback_basket_currency: Option<String>,
	/// MarketDisruptionFallbackBasketDivisor
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41105")]
	pub market_disruption_fallback_basket_divisor: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MarketDisruptionFallbackUnderlierType {
	/// Basket
	#[serde(rename = "0")]
	Basket,
	/// Bond
	#[serde(rename = "1")]
	Bond,
	/// Cash
	#[serde(rename = "2")]
	Cash,
	/// Commodity
	#[serde(rename = "3")]
	Commodity,
	/// Convertible bond
	#[serde(rename = "4")]
	ConvertibleBond,
	/// Equity
	#[serde(rename = "5")]
	Equity,
	/// Exchange traded fund
	#[serde(rename = "6")]
	ExchangeTradedFund,
	/// Future
	#[serde(rename = "7")]
	Future,
	/// Index
	#[serde(rename = "8")]
	Index,
	/// Loan
	#[serde(rename = "9")]
	Loan,
	/// Mortgage
	#[serde(rename = "10")]
	Mortgage,
	/// Mutual fund
	#[serde(rename = "11")]
	MutualFund,
}

impl Default for MarketDisruptionFallbackUnderlierType {
	fn default() -> Self {
		MarketDisruptionFallbackUnderlierType::Basket
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MarketDisruptionFallbackUnderlierSecurityIDSource {
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

impl Default for MarketDisruptionFallbackUnderlierSecurityIDSource {
	fn default() -> Self {
		MarketDisruptionFallbackUnderlierSecurityIDSource::Cusip
	}
}
