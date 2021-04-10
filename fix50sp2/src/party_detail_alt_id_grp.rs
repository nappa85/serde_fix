
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PartyDetailAltIDGrp {
	/// NoPartyDetailAltIDs
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1516")]
	pub party_detail_alt_i_ds: Option<crate::entities::RepeatingValues<PartyDetailAltID>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PartyDetailAltID {
	/// <p>Required when NoPartyDetailAltID &gt; 0</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1517")]
	pub party_detail_alt_id: Option<String>,
	/// <p>Required when NoPartyDetailAltID &gt; 0</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1518")]
	pub party_detail_alt_id_source: Option<PartyDetailAltIDSource>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PartyDetailAltIDSource {
	/// BIC (Bank Identification Code-Swift managed) code (ISO 9362)
	#[serde(rename = "B")]
	BicCode,
	/// Generally accepted market participant identifier (e.g. NASD mnemonic)
	#[serde(rename = "C")]
	GenerallyAcceptedMarketParticipantIdentifier,
	/// Proprietary/Custom code
	#[serde(rename = "D")]
	ProprietaryCustomCode,
	/// ISO Country Code
	#[serde(rename = "E")]
	IsoCountryCode,
	/// Settlement Entity Location (note if Local Market Settlement use "E = ISO Country Code")
	#[serde(rename = "F")]
	SettlementEntityLocation,
	/// Market Identifier Code (ISO 10383) MIC
	#[serde(rename = "G")]
	MarketIdentifierCodeMic,
	/// CSD participant/member code (e.g. Euroclear, DTC, CREST or Kassenverein number)
	#[serde(rename = "H")]
	CsdParticipantMemberCode,
	/// Korean Investor ID
	#[serde(rename = "1")]
	KoreanInvestorId,
	/// Taiwanese Qualified Foreign Investor ID QFII / FID
	#[serde(rename = "2")]
	TaiwaneseQualifiedForeignInvestorIdQfiiFid,
	/// Taiwanese Trading Account
	#[serde(rename = "3")]
	TaiwaneseTradingAccount,
	/// Malaysian Central Depository (MCD) number
	#[serde(rename = "4")]
	MalaysianCentralDepositoryNumber,
	/// Chinese B Share (Shezhen and Shanghai)
	#[serde(rename = "5")]
	ChineseBShare,
	/// UK National Insurance or Pension Number
	#[serde(rename = "6")]
	UkNationalInsuranceOrPensionNumber,
	/// US Social Security Number
	#[serde(rename = "7")]
	UsSocialSecurityNumber,
	/// US Employer Identification Number
	#[serde(rename = "8")]
	UsEmployerIdentificationNumber,
	/// Australian Business Number
	#[serde(rename = "9")]
	AustralianBusinessNumber,
	/// Australian Tax File Number
	#[serde(rename = "A")]
	AustralianTaxFileNumber,
	/// Directed broker three character acronym as defined in ISITC 'ETC Best Practice' guidelines document
	#[serde(rename = "I")]
	DirectedBrokerThreeCharacterAcronymAsDefinedInIsitcEtcBestPracticeGuidelinesDocument,
	/// Tax ID
	#[serde(rename = "J")]
	TaxId,
	/// Australian Company Number
	#[serde(rename = "K")]
	AustralianCompanyNumber,
	/// Australian Registered Body Number
	#[serde(rename = "L")]
	AustralianRegisteredBodyNumber,
	/// CFTC reporting firm identifier
	#[serde(rename = "M")]
	CftcReportingFirmIdentifier,
	/// Legal Entity Identifier (ISO 17442) LEI
	#[serde(rename = "N")]
	LegalEntityIdentifierLei,
	/// Interim identifier (An interim entity identifier assigned by a regulatory agency prior to an LEI (ISO 17442) being assigned.
	#[serde(rename = "O")]
	InterimIdentifierBeingAssigned,
	/// Short code identifier
	#[serde(rename = "P")]
	ShortCodeIdentifier,
	/// National ID of natural person
	#[serde(rename = "Q")]
	NationalIdOfNaturalPerson,
	/// India Permanent Account Number (Also referred to as PAN ID. An identifier issued by the Income Tax Department of India)
	#[serde(rename = "R")]
	IndiaPermanentAccountNumber,
	/// Firm designated identifier
	#[serde(rename = "S")]
	FirmDesignatedIdentifier,
	/// Special Segregated Account ID
	#[serde(rename = "T")]
	SpecialSegregatedAccountId,
	/// Master Special Segregated Account ID
	#[serde(rename = "U")]
	MasterSpecialSegregatedAccountId,
}
