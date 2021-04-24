
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InstrumentLeg {
	/// LegSymbol
	#[serde(rename = "600")]
	pub leg_symbol: String,
	/// LegSymbolSfx
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "601")]
	pub leg_symbol_sfx: Option<String>,
	/// LegSecurityID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "602")]
	pub leg_security_id: Option<String>,
	/// LegSecurityIDSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "603")]
	pub leg_security_id_source: Option<LegSecurityIDSource>,
	/// NoLegSecurityAltID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "604")]
	pub leg_security_alt_id: Option<fix_common::RepeatingValues<LegSecurityAltI>>,
	/// LegProduct
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "607")]
	pub leg_product: Option<LegProduct>,
	/// LegCFICode
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "608")]
	pub leg_cfi_code: Option<String>,
	/// LegSecurityType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "609")]
	pub leg_security_type: Option<LegSecurityType>,
	/// LegMaturityMonthYear
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "610")]
	pub leg_maturity_month_year: Option<fix_common::MonthYear>,
	/// LegMaturityDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "611")]
	pub leg_maturity_date: Option<fix_common::LocalMktDate>,
	/// LegCouponPaymentDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "248")]
	pub leg_coupon_payment_date: Option<fix_common::UTCDateOnly>,
	/// LegIssueDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "249")]
	pub leg_issue_date: Option<fix_common::UTCDateOnly>,
	/// LegRepoCollateralSecurityType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "250")]
	pub leg_repo_collateral_security_type: Option<LegRepoCollateralSecurityType>,
	/// LegRepurchaseTerm
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "251")]
	pub leg_repurchase_term: Option<i32>,
	/// LegRepurchaseRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "252")]
	pub leg_repurchase_rate: Option<f32>,
	/// LegFactor
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "253")]
	pub leg_factor: Option<f64>,
	/// LegCreditRating
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "257")]
	pub leg_credit_rating: Option<String>,
	/// LegInstrRegistry
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "599")]
	pub leg_instr_registry: Option<String>,
	/// LegCountryOfIssue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "596")]
	pub leg_country_of_issue: Option<LegCountryOfIssue>,
	/// LegStateOrProvinceOfIssue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "597")]
	pub leg_state_or_province_of_issue: Option<String>,
	/// LegLocaleOfIssue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "598")]
	pub leg_locale_of_issue: Option<String>,
	/// LegRedemptionDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "254")]
	pub leg_redemption_date: Option<fix_common::UTCDateOnly>,
	/// LegStrikePrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "612")]
	pub leg_strike_price: Option<f64>,
	/// LegOptAttribute
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "613")]
	pub leg_opt_attribute: Option<char>,
	/// LegContractMultiplier
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "614")]
	pub leg_contract_multiplier: Option<f64>,
	/// LegCouponRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "615")]
	pub leg_coupon_rate: Option<f32>,
	/// LegSecurityExchange
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "616")]
	pub leg_security_exchange: Option<String>,
	/// LegIssuer
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "617")]
	pub leg_issuer: Option<String>,
	/// EncodedLegIssuerLen
	#[serde(rename = "618")]
	/// EncodedLegIssuer
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "619")]
	pub encoded_leg_issuer: Option<fix_common::EncodedText<619>>,
	/// LegSecurityDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "620")]
	pub leg_security_desc: Option<String>,
	/// EncodedLegSecurityDescLen
	#[serde(rename = "621")]
	/// EncodedLegSecurityDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "622")]
	pub encoded_leg_security_desc: Option<fix_common::EncodedText<622>>,
	/// Specific to the <a href="block_Instrument_Leg.html" target="main">InstrumentLeg</a> (not in <a href="block_Instrument.html" target="main">Instrument</a> )
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "623")]
	pub leg_ratio_qty: Option<f64>,
	/// Specific to the <a href="block_Instrument_Leg.html" target="main">InstrumentLeg</a> (not in <a href="block_Instrument.html" target="main">Instrument</a> )
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "624")]
	pub leg_side: Option<LegSide>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegSecurityAltI {
	/// LegSecurityAltID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "605")]
	pub leg_security_alt_id: Option<String>,
	/// LegSecurityAltIDSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "606")]
	pub leg_security_alt_id_source: Option<LegSecurityAltIDSource>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegSecurityIDSource {
	/// CUSIP
	#[serde(rename = "1")]
	Cusip,
	/// SEDOL
	#[serde(rename = "2")]
	Sedol,
	/// QUIK
	#[serde(rename = "3")]
	Quik,
	/// ISIN number
	#[serde(rename = "4")]
	IsinNumber,
	/// RIC code
	#[serde(rename = "5")]
	RicCode,
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
}

impl Default for LegSecurityIDSource {
	fn default() -> Self {
		LegSecurityIDSource::Cusip
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegProduct {
	/// AGENCY
	#[serde(rename = "1")]
	Agency,
	/// COMMODITY
	#[serde(rename = "2")]
	Commodity,
	/// CORPORATE
	#[serde(rename = "3")]
	Corporate,
	/// CURRENCY
	#[serde(rename = "4")]
	Currency,
	/// EQUITY
	#[serde(rename = "5")]
	Equity,
	/// GOVERNMENT
	#[serde(rename = "6")]
	Government,
	/// INDEX
	#[serde(rename = "7")]
	Index,
	/// LOAN
	#[serde(rename = "8")]
	Loan,
	/// MONEYMARKET
	#[serde(rename = "9")]
	Moneymarket,
	/// MORTGAGE
	#[serde(rename = "10")]
	Mortgage,
	/// MUNICIPAL
	#[serde(rename = "11")]
	Municipal,
	/// OTHER
	#[serde(rename = "12")]
	Other,
}

impl Default for LegProduct {
	fn default() -> Self {
		LegProduct::Agency
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegSecurityType {
	/// Federal Agency Coupon
	#[serde(rename = "FAC")]
	FederalAgencyCoupon,
	/// Federal Agency Discount Note
	#[serde(rename = "FADN")]
	FederalAgencyDiscountNote,
	/// Private Export Funding Identify the Issuer in the <a href="tag_106_Issuer.html" target="bottom">Issuer&nbsp;(106)</a> field
	#[serde(rename = "PEF")]
	PrivateExportFundingIdentifyTheIssuerInTheAHrefTag106IssuerHtmlTargetBottomIssuerNbspAField,
	/// Corporate Bond
	#[serde(rename = "CORP")]
	CorporateBond,
	/// Corporate Private Placement
	#[serde(rename = "CPP")]
	CorporatePrivatePlacement,
	/// Convertible Bond
	#[serde(rename = "CB")]
	ConvertibleBond,
	/// Dual Currency
	#[serde(rename = "DUAL")]
	DualCurrency,
	/// Indexed Linked
	#[serde(rename = "XLINKD")]
	IndexedLinked,
	/// Structured Notes
	#[serde(rename = "STRUCT")]
	StructuredNotes,
	/// Yankee Corporate Bond
	#[serde(rename = "YANK")]
	YankeeCorporateBond,
	/// Foreign Exchange Contract
	#[serde(rename = "FOR")]
	ForeignExchangeContract,
	/// Common Stock
	#[serde(rename = "CS")]
	CommonStock,
	/// Preferred Stock
	#[serde(rename = "PS")]
	PreferredStock,
	/// Brady Bond
	#[serde(rename = "BRANDY")]
	BradyBond,
	/// US Treasury Bond
	#[serde(rename = "TBOND")]
	UsTreasuryBond,
	/// Interest strip from any bond or note
	#[serde(rename = "TINT")]
	InterestStripFromAnyBondOrNote,
	/// Treasury Inflation Protected Securities
	#[serde(rename = "TIPS")]
	TreasuryInflationProtectedSecurities,
	/// Principal strip of a callable bond or note
	#[serde(rename = "TCAL")]
	PrincipalStripOfACallableBondOrNote,
	/// Principal strip from a non-callable bond or note
	#[serde(rename = "TPRN")]
	PrincipalStripFromANonCallableBondOrNote,
	/// US Treasury Note/Bond
	#[serde(rename = "UST")]
	UsTreasuryNoteBond,
	/// US Treasury Bill
	#[serde(rename = "USTB")]
	UsTreasuryBill,
	/// Term Loan
	#[serde(rename = "TERM")]
	TermLoan,
	/// Revolver Loan
	#[serde(rename = "RVLV")]
	RevolverLoan,
	/// Revolver/Term Loan
	#[serde(rename = "RVLVTRM")]
	RevolverTermLoan,
	/// Bridge Loan
	#[serde(rename = "BRIDGE")]
	BridgeLoan,
	/// Letter of Credit
	#[serde(rename = "LOFC")]
	LetterOfCredit,
	/// Swing Line Facility
	#[serde(rename = "SWING")]
	SwingLineFacility,
	/// Debtor in Possession
	#[serde(rename = "DINP")]
	DebtorInPossession,
	/// Defaulted
	#[serde(rename = "DEFLTED")]
	Defaulted,
	/// Withdrawn
	#[serde(rename = "WITHDRN")]
	Withdrawn,
	/// Replaced
	#[serde(rename = "REPLACD")]
	Replaced,
	/// Matured
	#[serde(rename = "MATURED")]
	Matured,
	/// Amended &amp; Restated
	#[serde(rename = "AMENDED")]
	AmendedAmpRestated,
	/// Retired
	#[serde(rename = "RETIRED")]
	Retired,
	/// Bankers Acceptance
	#[serde(rename = "BA")]
	BankersAcceptance,
	/// Bank Notes
	#[serde(rename = "BN")]
	BankNotes,
	/// Bill of Exchanges
	#[serde(rename = "BOX")]
	BillOfExchanges,
	/// Certificate of Deposit
	#[serde(rename = "CD")]
	CertificateOfDeposit,
	/// Call Loans
	#[serde(rename = "CL")]
	CallLoans,
	/// Commercial Paper
	#[serde(rename = "CP")]
	CommercialPaper,
	/// Deposit Notes
	#[serde(rename = "DN")]
	DepositNotes,
	/// Liquidity Note
	#[serde(rename = "LQN")]
	LiquidityNote,
	/// Medium Term Notes
	#[serde(rename = "MTN")]
	MediumTermNotes,
	/// Overnight
	#[serde(rename = "ONITE")]
	Overnight,
	/// Promissory Note
	#[serde(rename = "PN")]
	PromissoryNote,
	/// Plazos Fijos
	#[serde(rename = "PZFJ")]
	PlazosFijos,
	/// Repurchase Agreement
	#[serde(rename = "RP")]
	RepurchaseAgreement,
	/// Reverse Repurchase Agreement
	#[serde(rename = "RVRP")]
	ReverseRepurchaseAgreement,
	/// Short Term Loan Note
	#[serde(rename = "STN")]
	ShortTermLoanNote,
	/// Time Deposit
	#[serde(rename = "TD")]
	TimeDeposit,
	/// Extended Comm Note
	#[serde(rename = "XCN")]
	ExtendedCommNote,
	/// Agency Pools
	#[serde(rename = "POOL")]
	AgencyPools,
	/// Asset-backed Securities
	#[serde(rename = "ABS")]
	AssetBackedSecurities,
	/// Corp. Mortgage-backed Securities
	#[serde(rename = "CMBS")]
	CorpMortgageBackedSecurities,
	/// Collateralized Mortgage Obligation
	#[serde(rename = "CMO")]
	CollateralizedMortgageObligation,
	/// IOETTE Mortgage
	#[serde(rename = "IET")]
	IoetteMortgage,
	/// Mortgage-backed Securities
	#[serde(rename = "MBS")]
	MortgageBackedSecurities,
	/// Mortgage Interest Only
	#[serde(rename = "MIO")]
	MortgageInterestOnly,
	/// Mortgage Principal Only
	#[serde(rename = "MPO")]
	MortgagePrincipalOnly,
	/// Mortgage Private Placement
	#[serde(rename = "MPP")]
	MortgagePrivatePlacement,
	/// Miscellaneous Pass-through
	#[serde(rename = "MPT")]
	MiscellaneousPassThrough,
	/// To be Announced
	#[serde(rename = "TBA")]
	ToBeAnnounced,
	/// Other Anticipation Notes BAN, GAN, etc.
	#[serde(rename = "AN")]
	OtherAnticipationNotesBanGanEtc,
	/// Certificate of Obligation
	#[serde(rename = "COFO")]
	CertificateOfObligation,
	/// Certificate of Participation
	#[serde(rename = "COFP")]
	CertificateOfParticipation,
	/// General Obligation Bonds
	#[serde(rename = "GO")]
	GeneralObligationBonds,
	/// Mandatory Tender
	#[serde(rename = "MT")]
	MandatoryTender,
	/// Revenue Anticipation Note
	#[serde(rename = "RAN")]
	RevenueAnticipationNote,
	/// Revenue Bonds
	#[serde(rename = "REV")]
	RevenueBonds,
	/// Special Assessment
	#[serde(rename = "SPCLA")]
	SpecialAssessment,
	/// Special Obligation
	#[serde(rename = "SPCLO")]
	SpecialObligation,
	/// Special Tax
	#[serde(rename = "SPCLT")]
	SpecialTax,
	/// Tax Anticipation Note
	#[serde(rename = "TAN")]
	TaxAnticipationNote,
	/// Tax Allocation
	#[serde(rename = "TAXA")]
	TaxAllocation,
	/// Tax Exempt Commercial Paper
	#[serde(rename = "TECP")]
	TaxExemptCommercialPaper,
	/// Tax &amp; Revenue Anticipation Note
	#[serde(rename = "TRAN")]
	TaxAmpRevenueAnticipationNote,
	/// Variable Rate Demand Note
	#[serde(rename = "VRDN")]
	VariableRateDemandNote,
	/// Warrant
	#[serde(rename = "WAR")]
	Warrant,
	/// Mutual Fund (i.e. any kind of open-ended "Collective Investment Vehicle")
	#[serde(rename = "MF")]
	MutualFund,
	/// Multi-leg instrument (e.g. options strategy or futures spread. <a href="tag_461_CFICode.html" target="bottom">CFICode&nbsp;(461)</a> can be used to identify if options-based, futures-based, etc.)
	#[serde(rename = "MLEG")]
	MultiLegInstrumentACanBeUsedToIdentifyIfOptionsBasedFuturesBasedEtc,
	/// No Security Type
	#[serde(rename = "NONE")]
	NoSecurityType,
	/// "Wildcard" entry (used on <a href="message_Security_Definition_Request_c.html" target="main">Security Definition Request&nbsp;(c)</a> message)
	#[serde(rename = "?")]
	WildcardEntryAMessage,
}

impl Default for LegSecurityType {
	fn default() -> Self {
		LegSecurityType::FederalAgencyCoupon
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegRepoCollateralSecurityType {
	/// Federal Agency Coupon
	#[serde(rename = "FAC")]
	FederalAgencyCoupon,
	/// Federal Agency Discount Note
	#[serde(rename = "FADN")]
	FederalAgencyDiscountNote,
	/// Private Export Funding Identify the Issuer in the <a href="tag_106_Issuer.html" target="bottom">Issuer&nbsp;(106)</a> field
	#[serde(rename = "PEF")]
	PrivateExportFundingIdentifyTheIssuerInTheAHrefTag106IssuerHtmlTargetBottomIssuerNbspAField,
	/// Corporate Bond
	#[serde(rename = "CORP")]
	CorporateBond,
	/// Corporate Private Placement
	#[serde(rename = "CPP")]
	CorporatePrivatePlacement,
	/// Convertible Bond
	#[serde(rename = "CB")]
	ConvertibleBond,
	/// Dual Currency
	#[serde(rename = "DUAL")]
	DualCurrency,
	/// Indexed Linked
	#[serde(rename = "XLINKD")]
	IndexedLinked,
	/// Structured Notes
	#[serde(rename = "STRUCT")]
	StructuredNotes,
	/// Yankee Corporate Bond
	#[serde(rename = "YANK")]
	YankeeCorporateBond,
	/// Foreign Exchange Contract
	#[serde(rename = "FOR")]
	ForeignExchangeContract,
	/// Common Stock
	#[serde(rename = "CS")]
	CommonStock,
	/// Preferred Stock
	#[serde(rename = "PS")]
	PreferredStock,
	/// Brady Bond
	#[serde(rename = "BRANDY")]
	BradyBond,
	/// US Treasury Bond
	#[serde(rename = "TBOND")]
	UsTreasuryBond,
	/// Interest strip from any bond or note
	#[serde(rename = "TINT")]
	InterestStripFromAnyBondOrNote,
	/// Treasury Inflation Protected Securities
	#[serde(rename = "TIPS")]
	TreasuryInflationProtectedSecurities,
	/// Principal strip of a callable bond or note
	#[serde(rename = "TCAL")]
	PrincipalStripOfACallableBondOrNote,
	/// Principal strip from a non-callable bond or note
	#[serde(rename = "TPRN")]
	PrincipalStripFromANonCallableBondOrNote,
	/// US Treasury Note/Bond
	#[serde(rename = "UST")]
	UsTreasuryNoteBond,
	/// US Treasury Bill
	#[serde(rename = "USTB")]
	UsTreasuryBill,
	/// Term Loan
	#[serde(rename = "TERM")]
	TermLoan,
	/// Revolver Loan
	#[serde(rename = "RVLV")]
	RevolverLoan,
	/// Revolver/Term Loan
	#[serde(rename = "RVLVTRM")]
	RevolverTermLoan,
	/// Bridge Loan
	#[serde(rename = "BRIDGE")]
	BridgeLoan,
	/// Letter of Credit
	#[serde(rename = "LOFC")]
	LetterOfCredit,
	/// Swing Line Facility
	#[serde(rename = "SWING")]
	SwingLineFacility,
	/// Debtor in Possession
	#[serde(rename = "DINP")]
	DebtorInPossession,
	/// Defaulted
	#[serde(rename = "DEFLTED")]
	Defaulted,
	/// Withdrawn
	#[serde(rename = "WITHDRN")]
	Withdrawn,
	/// Replaced
	#[serde(rename = "REPLACD")]
	Replaced,
	/// Matured
	#[serde(rename = "MATURED")]
	Matured,
	/// Amended &amp; Restated
	#[serde(rename = "AMENDED")]
	AmendedAmpRestated,
	/// Retired
	#[serde(rename = "RETIRED")]
	Retired,
	/// Bankers Acceptance
	#[serde(rename = "BA")]
	BankersAcceptance,
	/// Bank Notes
	#[serde(rename = "BN")]
	BankNotes,
	/// Bill of Exchanges
	#[serde(rename = "BOX")]
	BillOfExchanges,
	/// Certificate of Deposit
	#[serde(rename = "CD")]
	CertificateOfDeposit,
	/// Call Loans
	#[serde(rename = "CL")]
	CallLoans,
	/// Commercial Paper
	#[serde(rename = "CP")]
	CommercialPaper,
	/// Deposit Notes
	#[serde(rename = "DN")]
	DepositNotes,
	/// Liquidity Note
	#[serde(rename = "LQN")]
	LiquidityNote,
	/// Medium Term Notes
	#[serde(rename = "MTN")]
	MediumTermNotes,
	/// Overnight
	#[serde(rename = "ONITE")]
	Overnight,
	/// Promissory Note
	#[serde(rename = "PN")]
	PromissoryNote,
	/// Plazos Fijos
	#[serde(rename = "PZFJ")]
	PlazosFijos,
	/// Repurchase Agreement
	#[serde(rename = "RP")]
	RepurchaseAgreement,
	/// Reverse Repurchase Agreement
	#[serde(rename = "RVRP")]
	ReverseRepurchaseAgreement,
	/// Short Term Loan Note
	#[serde(rename = "STN")]
	ShortTermLoanNote,
	/// Time Deposit
	#[serde(rename = "TD")]
	TimeDeposit,
	/// Extended Comm Note
	#[serde(rename = "XCN")]
	ExtendedCommNote,
	/// Agency Pools
	#[serde(rename = "POOL")]
	AgencyPools,
	/// Asset-backed Securities
	#[serde(rename = "ABS")]
	AssetBackedSecurities,
	/// Corp. Mortgage-backed Securities
	#[serde(rename = "CMBS")]
	CorpMortgageBackedSecurities,
	/// Collateralized Mortgage Obligation
	#[serde(rename = "CMO")]
	CollateralizedMortgageObligation,
	/// IOETTE Mortgage
	#[serde(rename = "IET")]
	IoetteMortgage,
	/// Mortgage-backed Securities
	#[serde(rename = "MBS")]
	MortgageBackedSecurities,
	/// Mortgage Interest Only
	#[serde(rename = "MIO")]
	MortgageInterestOnly,
	/// Mortgage Principal Only
	#[serde(rename = "MPO")]
	MortgagePrincipalOnly,
	/// Mortgage Private Placement
	#[serde(rename = "MPP")]
	MortgagePrivatePlacement,
	/// Miscellaneous Pass-through
	#[serde(rename = "MPT")]
	MiscellaneousPassThrough,
	/// To be Announced
	#[serde(rename = "TBA")]
	ToBeAnnounced,
	/// Other Anticipation Notes BAN, GAN, etc.
	#[serde(rename = "AN")]
	OtherAnticipationNotesBanGanEtc,
	/// Certificate of Obligation
	#[serde(rename = "COFO")]
	CertificateOfObligation,
	/// Certificate of Participation
	#[serde(rename = "COFP")]
	CertificateOfParticipation,
	/// General Obligation Bonds
	#[serde(rename = "GO")]
	GeneralObligationBonds,
	/// Mandatory Tender
	#[serde(rename = "MT")]
	MandatoryTender,
	/// Revenue Anticipation Note
	#[serde(rename = "RAN")]
	RevenueAnticipationNote,
	/// Revenue Bonds
	#[serde(rename = "REV")]
	RevenueBonds,
	/// Special Assessment
	#[serde(rename = "SPCLA")]
	SpecialAssessment,
	/// Special Obligation
	#[serde(rename = "SPCLO")]
	SpecialObligation,
	/// Special Tax
	#[serde(rename = "SPCLT")]
	SpecialTax,
	/// Tax Anticipation Note
	#[serde(rename = "TAN")]
	TaxAnticipationNote,
	/// Tax Allocation
	#[serde(rename = "TAXA")]
	TaxAllocation,
	/// Tax Exempt Commercial Paper
	#[serde(rename = "TECP")]
	TaxExemptCommercialPaper,
	/// Tax &amp; Revenue Anticipation Note
	#[serde(rename = "TRAN")]
	TaxAmpRevenueAnticipationNote,
	/// Variable Rate Demand Note
	#[serde(rename = "VRDN")]
	VariableRateDemandNote,
	/// Warrant
	#[serde(rename = "WAR")]
	Warrant,
	/// Mutual Fund (i.e. any kind of open-ended "Collective Investment Vehicle")
	#[serde(rename = "MF")]
	MutualFund,
	/// Multi-leg instrument (e.g. options strategy or futures spread. <a href="tag_461_CFICode.html" target="bottom">CFICode&nbsp;(461)</a> can be used to identify if options-based, futures-based, etc.)
	#[serde(rename = "MLEG")]
	MultiLegInstrumentACanBeUsedToIdentifyIfOptionsBasedFuturesBasedEtc,
	/// No Security Type
	#[serde(rename = "NONE")]
	NoSecurityType,
	/// "Wildcard" entry (used on <a href="message_Security_Definition_Request_c.html" target="main">Security Definition Request&nbsp;(c)</a> message)
	#[serde(rename = "?")]
	WildcardEntryAMessage,
}

impl Default for LegRepoCollateralSecurityType {
	fn default() -> Self {
		LegRepoCollateralSecurityType::FederalAgencyCoupon
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegCountryOfIssue {
	/// AFGHANISTAN
	#[serde(rename = "AF")]
	Afghanistan,
	/// ALBANIA
	#[serde(rename = "AL")]
	Albania,
	/// ALGERIA
	#[serde(rename = "DZ")]
	Algeria,
	/// AMERICAN SAMOA
	#[serde(rename = "AS")]
	AmericanSamoa,
	/// ANDORRA
	#[serde(rename = "AD")]
	Andorra,
	/// ANGOLA
	#[serde(rename = "AO")]
	Angola,
	/// ANGUILLA
	#[serde(rename = "AI")]
	Anguilla,
	/// ANTARCTICA
	#[serde(rename = "AQ")]
	Antarctica,
	/// ANTIGUA AND BARBUDA
	#[serde(rename = "AG")]
	AntiguaAndBarbuda,
	/// ARGENTINA
	#[serde(rename = "AR")]
	Argentina,
	/// ARMENIA
	#[serde(rename = "AM")]
	Armenia,
	/// ARUBA
	#[serde(rename = "AW")]
	Aruba,
	/// AUSTRALIA
	#[serde(rename = "AU")]
	Australia,
	/// AUSTRIA
	#[serde(rename = "AT")]
	Austria,
	/// AZERBAIJAN
	#[serde(rename = "AZ")]
	Azerbaijan,
	/// BAHAMAS
	#[serde(rename = "BS")]
	Bahamas,
	/// BAHRAIN
	#[serde(rename = "BH")]
	Bahrain,
	/// BANGLADESH
	#[serde(rename = "BD")]
	Bangladesh,
	/// BARBADOS
	#[serde(rename = "BB")]
	Barbados,
	/// BELARUS
	#[serde(rename = "BY")]
	Belarus,
	/// BELGIUM
	#[serde(rename = "BE")]
	Belgium,
	/// BELIZE
	#[serde(rename = "BZ")]
	Belize,
	/// BENIN
	#[serde(rename = "BJ")]
	Benin,
	/// BERMUDA
	#[serde(rename = "BM")]
	Bermuda,
	/// BHUTAN
	#[serde(rename = "BT")]
	Bhutan,
	/// BOLIVIA
	#[serde(rename = "BO")]
	Bolivia,
	/// BOSNIA AND HERZEGOVINA
	#[serde(rename = "BA")]
	BosniaAndHerzegovina,
	/// BOTSWANA
	#[serde(rename = "BW")]
	Botswana,
	/// BOUVET ISLAND
	#[serde(rename = "BV")]
	BouvetIsland,
	/// BRAZIL
	#[serde(rename = "BR")]
	Brazil,
	/// BRITISH INDIAN OCEAN TERRITORY
	#[serde(rename = "IO")]
	BritishIndianOceanTerritory,
	/// BRUNEI DARUSSALAM
	#[serde(rename = "BN")]
	BruneiDarussalam,
	/// BULGARIA
	#[serde(rename = "BG")]
	Bulgaria,
	/// BURKINA FASO
	#[serde(rename = "BF")]
	BurkinaFaso,
	/// BURUNDI
	#[serde(rename = "BI")]
	Burundi,
	/// CAMBODIA
	#[serde(rename = "KH")]
	Cambodia,
	/// CAMEROON
	#[serde(rename = "CM")]
	Cameroon,
	/// CANADA
	#[serde(rename = "CA")]
	Canada,
	/// CAPE VERDE
	#[serde(rename = "CV")]
	CapeVerde,
	/// CAYMAN ISLANDS
	#[serde(rename = "KY")]
	CaymanIslands,
	/// CENTRAL AFRICAN REPUBLIC
	#[serde(rename = "CF")]
	CentralAfricanRepublic,
	/// CHAD
	#[serde(rename = "TD")]
	Chad,
	/// CHILE
	#[serde(rename = "CL")]
	Chile,
	/// CHINA
	#[serde(rename = "CN")]
	China,
	/// CHRISTMAS ISLAND
	#[serde(rename = "CX")]
	ChristmasIsland,
	/// COCOS (KEELING) ISLANDS
	#[serde(rename = "CC")]
	CocosIslands,
	/// COLOMBIA
	#[serde(rename = "CO")]
	Colombia,
	/// COMOROS
	#[serde(rename = "KM")]
	Comoros,
	/// CONGO
	#[serde(rename = "CG")]
	Congo,
	/// CONGO, THE DEMOCRATIC REPUBLIC OF THE
	#[serde(rename = "CD")]
	CongoTheDemocraticRepublicOfThe,
	/// COOK ISLANDS
	#[serde(rename = "CK")]
	CookIslands,
	/// COSTA RICA
	#[serde(rename = "CR")]
	CostaRica,
	/// COTE D'IVOIRE
	#[serde(rename = "CI")]
	CoteDIvoire,
	/// CROATIA
	#[serde(rename = "HR")]
	Croatia,
	/// CUBA
	#[serde(rename = "CU")]
	Cuba,
	/// CYPRUS
	#[serde(rename = "CY")]
	Cyprus,
	/// CZECH REPUBLIC
	#[serde(rename = "CZ")]
	CzechRepublic,
	/// DENMARK
	#[serde(rename = "DK")]
	Denmark,
	/// DJIBOUTI
	#[serde(rename = "DJ")]
	Djibouti,
	/// DOMINICA
	#[serde(rename = "DM")]
	Dominica,
	/// DOMINICAN REPUBLIC
	#[serde(rename = "DO")]
	DominicanRepublic,
	/// ECUADOR
	#[serde(rename = "EC")]
	Ecuador,
	/// EGYPT
	#[serde(rename = "EG")]
	Egypt,
	/// EL SALVADOR
	#[serde(rename = "SV")]
	ElSalvador,
	/// EQUATORIAL GUINEA
	#[serde(rename = "GQ")]
	EquatorialGuinea,
	/// ERITREA
	#[serde(rename = "ER")]
	Eritrea,
	/// ESTONIA
	#[serde(rename = "EE")]
	Estonia,
	/// ETHIOPIA
	#[serde(rename = "ET")]
	Ethiopia,
	/// FALKLAND ISLANDS (MALVINAS)
	#[serde(rename = "FK")]
	FalklandIslands,
	/// FAROE ISLANDS
	#[serde(rename = "FO")]
	FaroeIslands,
	/// FIJI
	#[serde(rename = "FJ")]
	Fiji,
	/// FINLAND
	#[serde(rename = "FI")]
	Finland,
	/// FRANCE
	#[serde(rename = "FR")]
	France,
	/// FRENCH GUIANA
	#[serde(rename = "GF")]
	FrenchGuiana,
	/// FRENCH POLYNESIA
	#[serde(rename = "PF")]
	FrenchPolynesia,
	/// FRENCH SOUTHERN TERRITORIES
	#[serde(rename = "TF")]
	FrenchSouthernTerritories,
	/// GABON
	#[serde(rename = "GA")]
	Gabon,
	/// GAMBIA
	#[serde(rename = "GM")]
	Gambia,
	/// GEORGIA
	#[serde(rename = "GE")]
	Georgia,
	/// GERMANY
	#[serde(rename = "DE")]
	Germany,
	/// GHANA
	#[serde(rename = "GH")]
	Ghana,
	/// GIBRALTAR
	#[serde(rename = "GI")]
	Gibraltar,
	/// GREECE
	#[serde(rename = "GR")]
	Greece,
	/// GREENLAND
	#[serde(rename = "GL")]
	Greenland,
	/// GRENADA
	#[serde(rename = "GD")]
	Grenada,
	/// GUADELOUPE
	#[serde(rename = "GP")]
	Guadeloupe,
	/// GUAM
	#[serde(rename = "GU")]
	Guam,
	/// GUATEMALA
	#[serde(rename = "GT")]
	Guatemala,
	/// GUINEA
	#[serde(rename = "GN")]
	Guinea,
	/// GUINEA-BISSAU
	#[serde(rename = "GW")]
	GuineaBissau,
	/// GUYANA
	#[serde(rename = "GY")]
	Guyana,
	/// HAITI
	#[serde(rename = "HT")]
	Haiti,
	/// HEARD ISLAND AND MCDONALD ISLANDS
	#[serde(rename = "HM")]
	HeardIslandAndMcdonaldIslands,
	/// HOLY SEE (VATICAN CITY STATE)
	#[serde(rename = "VA")]
	HolySee,
	/// HONDURAS
	#[serde(rename = "HN")]
	Honduras,
	/// HONG KONG
	#[serde(rename = "HK")]
	HongKong,
	/// HUNGARY
	#[serde(rename = "HU")]
	Hungary,
	/// ICELAND
	#[serde(rename = "IS")]
	Iceland,
	/// INDIA
	#[serde(rename = "IN")]
	India,
	/// INDONESIA
	#[serde(rename = "ID")]
	Indonesia,
	/// IRAN, ISLAMIC REPUBLIC OF
	#[serde(rename = "IR")]
	IranIslamicRepublicOf,
	/// IRAQ
	#[serde(rename = "IQ")]
	Iraq,
	/// IRELAND
	#[serde(rename = "IE")]
	Ireland,
	/// ISRAEL
	#[serde(rename = "IL")]
	Israel,
	/// ITALY
	#[serde(rename = "IT")]
	Italy,
	/// JAMAICA
	#[serde(rename = "JM")]
	Jamaica,
	/// JAPAN
	#[serde(rename = "JP")]
	Japan,
	/// JORDAN
	#[serde(rename = "JO")]
	Jordan,
	/// KAZAKHSTAN
	#[serde(rename = "KZ")]
	Kazakhstan,
	/// KENYA
	#[serde(rename = "KE")]
	Kenya,
	/// KIRIBATI
	#[serde(rename = "KI")]
	Kiribati,
	/// KOREA, DEMOCRATIC PEOPLE'S REPUBLIC OF
	#[serde(rename = "KP")]
	KoreaDemocraticPeopleSRepublicOf,
	/// KOREA, REPUBLIC OF
	#[serde(rename = "KR")]
	KoreaRepublicOf,
	/// KUWAIT
	#[serde(rename = "KW")]
	Kuwait,
	/// KYRGYZSTAN
	#[serde(rename = "KG")]
	Kyrgyzstan,
	/// LAO PEOPLE'S DEMOCRATIC REPUBLIC
	#[serde(rename = "LA")]
	LaoPeopleSDemocraticRepublic,
	/// LATVIA
	#[serde(rename = "LV")]
	Latvia,
	/// LEBANON
	#[serde(rename = "LB")]
	Lebanon,
	/// LESOTHO
	#[serde(rename = "LS")]
	Lesotho,
	/// LIBERIA
	#[serde(rename = "LR")]
	Liberia,
	/// LIBYAN ARAB JAMAHIRIYA
	#[serde(rename = "LY")]
	LibyanArabJamahiriya,
	/// LIECHTENSTEIN
	#[serde(rename = "LI")]
	Liechtenstein,
	/// LITHUANIA
	#[serde(rename = "LT")]
	Lithuania,
	/// LUXEMBOURG
	#[serde(rename = "LU")]
	Luxembourg,
	/// MACAO
	#[serde(rename = "MO")]
	Macao,
	/// MACEDONIA, THE FORMER YUGOSLAV REPUBLIC OF
	#[serde(rename = "MK")]
	MacedoniaTheFormerYugoslavRepublicOf,
	/// MADAGASCAR
	#[serde(rename = "MG")]
	Madagascar,
	/// MALAWI
	#[serde(rename = "MW")]
	Malawi,
	/// MALAYSIA
	#[serde(rename = "MY")]
	Malaysia,
	/// MALDIVES
	#[serde(rename = "MV")]
	Maldives,
	/// MALI
	#[serde(rename = "ML")]
	Mali,
	/// MALTA
	#[serde(rename = "MT")]
	Malta,
	/// MARSHALL ISLANDS
	#[serde(rename = "MH")]
	MarshallIslands,
	/// MARTINIQUE
	#[serde(rename = "MQ")]
	Martinique,
	/// MAURITANIA
	#[serde(rename = "MR")]
	Mauritania,
	/// MAURITIUS
	#[serde(rename = "MU")]
	Mauritius,
	/// MAYOTTE
	#[serde(rename = "YT")]
	Mayotte,
	/// MEXICO
	#[serde(rename = "MX")]
	Mexico,
	/// MICRONESIA, FEDERATED STATES OF
	#[serde(rename = "FM")]
	MicronesiaFederatedStatesOf,
	/// MOLDOVA, REPUBLIC OF
	#[serde(rename = "MD")]
	MoldovaRepublicOf,
	/// MONACO
	#[serde(rename = "MC")]
	Monaco,
	/// MONGOLIA
	#[serde(rename = "MN")]
	Mongolia,
	/// MONTSERRAT
	#[serde(rename = "MS")]
	Montserrat,
	/// MOROCCO
	#[serde(rename = "MA")]
	Morocco,
	/// MOZAMBIQUE
	#[serde(rename = "MZ")]
	Mozambique,
	/// MYANMAR
	#[serde(rename = "MM")]
	Myanmar,
	/// NAMIBIA
	#[serde(rename = "NA")]
	Namibia,
	/// NAURU
	#[serde(rename = "NR")]
	Nauru,
	/// NEPAL
	#[serde(rename = "NP")]
	Nepal,
	/// NETHERLANDS
	#[serde(rename = "NL")]
	Netherlands,
	/// NETHERLANDS ANTILLES
	#[serde(rename = "AN")]
	NetherlandsAntilles,
	/// NEW CALEDONIA
	#[serde(rename = "NC")]
	NewCaledonia,
	/// NEW ZEALAND
	#[serde(rename = "NZ")]
	NewZealand,
	/// NICARAGUA
	#[serde(rename = "NI")]
	Nicaragua,
	/// NIGER
	#[serde(rename = "NE")]
	Niger,
	/// NIGERIA
	#[serde(rename = "NG")]
	Nigeria,
	/// NIUE
	#[serde(rename = "NU")]
	Niue,
	/// NORFOLK ISLAND
	#[serde(rename = "NF")]
	NorfolkIsland,
	/// NORTHERN MARIANA ISLANDS
	#[serde(rename = "MP")]
	NorthernMarianaIslands,
	/// NORWAY
	#[serde(rename = "NO")]
	Norway,
	/// OMAN
	#[serde(rename = "OM")]
	Oman,
	/// PAKISTAN
	#[serde(rename = "PK")]
	Pakistan,
	/// PALAU
	#[serde(rename = "PW")]
	Palau,
	/// PALESTINIAN TERRITORY, OCCUPIED
	#[serde(rename = "PS")]
	PalestinianTerritoryOccupied,
	/// PANAMA
	#[serde(rename = "PA")]
	Panama,
	/// PAPUA NEW GUINEA
	#[serde(rename = "PG")]
	PapuaNewGuinea,
	/// PARAGUAY
	#[serde(rename = "PY")]
	Paraguay,
	/// PERU
	#[serde(rename = "PE")]
	Peru,
	/// PHILIPPINES
	#[serde(rename = "PH")]
	Philippines,
	/// PITCAIRN
	#[serde(rename = "PN")]
	Pitcairn,
	/// POLAND
	#[serde(rename = "PL")]
	Poland,
	/// PORTUGAL
	#[serde(rename = "PT")]
	Portugal,
	/// PUERTO RICO
	#[serde(rename = "PR")]
	PuertoRico,
	/// QATAR
	#[serde(rename = "QA")]
	Qatar,
	/// REUNION
	#[serde(rename = "RE")]
	Reunion,
	/// ROMANIA
	#[serde(rename = "RO")]
	Romania,
	/// RUSSIAN FEDERATION
	#[serde(rename = "RU")]
	RussianFederation,
	/// RWANDA
	#[serde(rename = "RW")]
	Rwanda,
	/// SAINT HELENA
	#[serde(rename = "SH")]
	SaintHelena,
	/// SAINT KITTS AND NEVIS
	#[serde(rename = "KN")]
	SaintKittsAndNevis,
	/// SAINT LUCIA
	#[serde(rename = "LC")]
	SaintLucia,
	/// SAINT PIERRE AND MIQUELON
	#[serde(rename = "PM")]
	SaintPierreAndMiquelon,
	/// SAINT VINCENT AND THE GRENADINES
	#[serde(rename = "VC")]
	SaintVincentAndTheGrenadines,
	/// SAMOA
	#[serde(rename = "WS")]
	Samoa,
	/// SAN MARINO
	#[serde(rename = "SM")]
	SanMarino,
	/// SAO TOME AND PRINCIPE
	#[serde(rename = "ST")]
	SaoTomeAndPrincipe,
	/// SAUDI ARABIA
	#[serde(rename = "SA")]
	SaudiArabia,
	/// SENEGAL
	#[serde(rename = "SN")]
	Senegal,
	/// SEYCHELLES
	#[serde(rename = "SC")]
	Seychelles,
	/// SIERRA LEONE
	#[serde(rename = "SL")]
	SierraLeone,
	/// SINGAPORE
	#[serde(rename = "SG")]
	Singapore,
	/// SLOVAKIA
	#[serde(rename = "SK")]
	Slovakia,
	/// SLOVENIA
	#[serde(rename = "SI")]
	Slovenia,
	/// SOLOMON ISLANDS
	#[serde(rename = "SB")]
	SolomonIslands,
	/// SOMALIA
	#[serde(rename = "SO")]
	Somalia,
	/// SOUTH AFRICA
	#[serde(rename = "ZA")]
	SouthAfrica,
	/// SOUTH GEORGIA AND THE SOUTH SANDWICH ISLANDS
	#[serde(rename = "GS")]
	SouthGeorgiaAndTheSouthSandwichIslands,
	/// SPAIN
	#[serde(rename = "ES")]
	Spain,
	/// SRI LANKA
	#[serde(rename = "LK")]
	SriLanka,
	/// SUDAN
	#[serde(rename = "SD")]
	Sudan,
	/// SURINAME
	#[serde(rename = "SR")]
	Suriname,
	/// SVALBARD AND JAN MAYEN
	#[serde(rename = "SJ")]
	SvalbardAndJanMayen,
	/// SWAZILAND
	#[serde(rename = "SZ")]
	Swaziland,
	/// SWEDEN
	#[serde(rename = "SE")]
	Sweden,
	/// SWITZERLAND
	#[serde(rename = "CH")]
	Switzerland,
	/// SYRIAN ARAB REPUBLIC
	#[serde(rename = "SY")]
	SyrianArabRepublic,
	/// TAIWAN, PROVINCE OF CHINA
	#[serde(rename = "TW")]
	TaiwanProvinceOfChina,
	/// TAJIKISTAN
	#[serde(rename = "TJ")]
	Tajikistan,
	/// TANZANIA, UNITED REPUBLIC OF
	#[serde(rename = "TZ")]
	TanzaniaUnitedRepublicOf,
	/// THAILAND
	#[serde(rename = "TH")]
	Thailand,
	/// TIMOR-LESTE
	#[serde(rename = "TL")]
	TimorLeste,
	/// TOGO
	#[serde(rename = "TG")]
	Togo,
	/// TOKELAU
	#[serde(rename = "TK")]
	Tokelau,
	/// TONGA
	#[serde(rename = "TO")]
	Tonga,
	/// TRINIDAD AND TOBAGO
	#[serde(rename = "TT")]
	TrinidadAndTobago,
	/// TUNISIA
	#[serde(rename = "TN")]
	Tunisia,
	/// TURKEY
	#[serde(rename = "TR")]
	Turkey,
	/// TURKMENISTAN
	#[serde(rename = "TM")]
	Turkmenistan,
	/// TURKS AND CAICOS ISLANDS
	#[serde(rename = "TC")]
	TurksAndCaicosIslands,
	/// TUVALU
	#[serde(rename = "TV")]
	Tuvalu,
	/// UGANDA
	#[serde(rename = "UG")]
	Uganda,
	/// UKRAINE
	#[serde(rename = "UA")]
	Ukraine,
	/// UNITED ARAB EMIRATES
	#[serde(rename = "AE")]
	UnitedArabEmirates,
	/// UNITED KINGDOM
	#[serde(rename = "GB")]
	UnitedKingdom,
	/// UNITED STATES
	#[serde(rename = "US")]
	UnitedStates,
	/// UNITED STATES MINOR OUTLYING ISLANDS
	#[serde(rename = "UM")]
	UnitedStatesMinorOutlyingIslands,
	/// URUGUAY
	#[serde(rename = "UY")]
	Uruguay,
	/// UZBEKISTAN
	#[serde(rename = "UZ")]
	Uzbekistan,
	/// VANUATU
	#[serde(rename = "VU")]
	Vanuatu,
	/// VENEZUELA
	#[serde(rename = "VE")]
	Venezuela,
	/// VIET NAM
	#[serde(rename = "VN")]
	VietNam,
	/// VIRGIN ISLANDS, BRITISH
	#[serde(rename = "VG")]
	VirginIslandsBritish,
	/// VIRGIN ISLANDS, U.S.
	#[serde(rename = "VI")]
	VirginIslandsUS,
	/// WALLIS AND FUTUNA
	#[serde(rename = "WF")]
	WallisAndFutuna,
	/// WESTERN SAHARA
	#[serde(rename = "EH")]
	WesternSahara,
	/// YEMEN
	#[serde(rename = "YE")]
	Yemen,
	/// YUGOSLAVIA
	#[serde(rename = "YU")]
	Yugoslavia,
	/// ZAMBIA
	#[serde(rename = "ZM")]
	Zambia,
	/// ZIMBABWE
	#[serde(rename = "ZW")]
	Zimbabwe,
}

impl Default for LegCountryOfIssue {
	fn default() -> Self {
		LegCountryOfIssue::Afghanistan
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegSide {
	/// Buy
	#[serde(rename = "1")]
	Buy,
	/// Sell
	#[serde(rename = "2")]
	Sell,
	/// Buy minus
	#[serde(rename = "3")]
	BuyMinus,
	/// Sell plus
	#[serde(rename = "4")]
	SellPlus,
	/// Sell short
	#[serde(rename = "5")]
	SellShort,
	/// Sell short exempt
	#[serde(rename = "6")]
	SellShortExempt,
	/// Undisclosed (valid for IOI and List Order messages only)
	#[serde(rename = "7")]
	Undisclosed,
	/// Cross (orders where counterparty is an exchange, valid for all messages except IOIs)
	#[serde(rename = "8")]
	Cross,
	/// Cross short
	#[serde(rename = "9")]
	CrossShort,
	/// Cross short exempt
	#[serde(rename = "A")]
	CrossShortExempt,
	/// "As Defined" (for use with multileg instruments)
	#[serde(rename = "B")]
	AsDefined,
	/// "Opposite" (for use with multileg instruments)
	#[serde(rename = "C")]
	Opposite,
}

impl Default for LegSide {
	fn default() -> Self {
		LegSide::Buy
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegSecurityAltIDSource {
	/// CUSIP
	#[serde(rename = "1")]
	Cusip,
	/// SEDOL
	#[serde(rename = "2")]
	Sedol,
	/// QUIK
	#[serde(rename = "3")]
	Quik,
	/// ISIN number
	#[serde(rename = "4")]
	IsinNumber,
	/// RIC code
	#[serde(rename = "5")]
	RicCode,
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
}

impl Default for LegSecurityAltIDSource {
	fn default() -> Self {
		LegSecurityAltIDSource::Cusip
	}
}
