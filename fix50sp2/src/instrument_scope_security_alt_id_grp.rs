
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InstrumentScopeSecurityAltIDGrp {
	/// NoInstrumentScopeSecurityAltIDs
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1540")]
	pub instrument_scope_security_alt_i_ds: Option<fix_common::RepeatingValues<InstrumentScopeSecurityAltID>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InstrumentScopeSecurityAltID {
	/// <p>Required when <a href="tag_1540_NoInstrumentScopeSecurityAltIDs.html" target="bottom">NoInstrumentScopeSecurityAltIDs&nbsp;(1540)</a> &gt; 0
	/// </p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1541")]
	pub instrument_scope_security_alt_id: Option<String>,
	/// <p>Required when <a href="tag_1540_NoInstrumentScopeSecurityAltIDs.html" target="bottom">NoInstrumentScopeSecurityAltIDs&nbsp;(1540)</a> &gt; 0
	/// </p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1542")]
	pub instrument_scope_security_alt_id_source: Option<InstrumentScopeSecurityAltIDSource>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum InstrumentScopeSecurityAltIDSource {
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

impl Default for InstrumentScopeSecurityAltIDSource {
	fn default() -> Self {
		InstrumentScopeSecurityAltIDSource::Cusip
	}
}
