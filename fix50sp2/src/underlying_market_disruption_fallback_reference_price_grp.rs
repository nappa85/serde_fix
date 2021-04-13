
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingMarketDisruptionFallbackReferencePriceGrp {
	/// NoUnderlyingMarketDisruptionFallbackReferencePrices
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41868")]
	pub underlying_market_disruption_fallback_reference_prices: Option<fix_common::RepeatingValues<UnderlyingMarketDisruptionFallbackReferencePrice>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingMarketDisruptionFallbackReferencePrice {
	/// Required if NoUnderlyingMarketDisruptionFallbackReferencePrices (41868) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41869")]
	pub underlying_market_disruption_fallback_underlier_type: Option<UnderlyingMarketDisruptionFallbackUnderlierType>,
	/// Conditionally required whem UnderlyingMarketDisruptionFallbackUnderlierSecurityIDSource(41871) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41870")]
	pub underlying_market_disruption_fallback_underlier_security_id: Option<String>,
	/// Conditionally required whem UnderlyingMarketDisruptionFallbackUnderlierSecurityID(41870) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41871")]
	pub underlying_market_disruption_fallback_underlier_security_id_source: Option<UnderlyingMarketDisruptionFallbackUnderlierSecurityIDSource>,
	/// UnderlyingMarketDisruptionFallbackUnderlierSecurityDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41872")]
	pub underlying_market_disruption_fallback_underlier_security_desc: Option<String>,
	/// Must be set if EncodedUnderlyingMarketDisruptionFallbackUnderlierSecurityDesc(41874) field is specified and must immediately
	/// precede it.
	#[serde(rename = "41873")]
	/// Encoded (non-ASCII characters) representation of the UnderlyingMarketDisruptionFallbackUnderlierSecurityDesc(41872) field
	/// in the encoded format specified via the MessageEncoding(347) field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "41874")]
	pub encoded_underlying_market_disruption_fallback_underlier_security_desc: Option<fix_common::EncodedText<41874>>,
	/// UnderlyingMarketDisruptionFallbackOpenUnits
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41875")]
	pub underlying_market_disruption_fallback_open_units: Option<f64>,
	/// UnderlyingMarketDisruptionFallbackBasketCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41876")]
	pub underlying_market_disruption_fallback_basket_currency: Option<String>,
	/// UnderlyingMarketDisruptionFallbackBasketDivisor
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41877")]
	pub underlying_market_disruption_fallback_basket_divisor: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingMarketDisruptionFallbackUnderlierType {
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

impl Default for UnderlyingMarketDisruptionFallbackUnderlierType {
	fn default() -> Self {
		UnderlyingMarketDisruptionFallbackUnderlierType::Basket
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingMarketDisruptionFallbackUnderlierSecurityIDSource {
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

impl Default for UnderlyingMarketDisruptionFallbackUnderlierSecurityIDSource {
	fn default() -> Self {
		UnderlyingMarketDisruptionFallbackUnderlierSecurityIDSource::Cusip
	}
}
