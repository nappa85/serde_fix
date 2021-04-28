
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Instrument {
	/// Common, "human understood" representation of the security. <a href="tag_48_SecurityID.html" target="bottom">SecurityID&nbsp;(48)</a> value can be specified if no symbol exists (e.g. non-exchange traded Collective Investment Vehicles)
	#[serde(rename = "55")]
	pub symbol: String,
	/// SymbolSfx
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "65")]
	pub symbol_sfx: Option<String>,
	/// Takes precedence in identifying security to counterparty over <a href="tag_455_SecurityAltID.html" target="bottom">SecurityAltID&nbsp;(455)</a> block. Requires <a href="tag_22_SecurityIDSource.html" target="bottom">SecurityIDSource&nbsp;(22)</a> if specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "48")]
	pub security_id: Option<String>,
	/// Required if <a href="tag_48_SecurityID.html" target="bottom">SecurityID&nbsp;(48)</a> is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "22")]
	pub security_id_source: Option<SecurityIDSource>,
	/// Number of alternate Security Identifiers
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "454")]
	pub security_alt_id: Option<fix_common::RepeatingValues<SecurityAltI>>,
	/// Indicates the type of product the security is associated with (high-level category)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "460")]
	pub product: Option<Product>,
	/// Indicates the type of security using ISO 10962 standard, Classification of Financial Instruments (CFI code) values. It is
	/// recommended that <a href="tag_461_CFICode.html" target="bottom">CFICode&nbsp;(461)</a> be used instead of <a href="tag_167_SecurityType.html" target="bottom">SecurityType&nbsp;(167)</a> for non-Fixed Income instruments.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "461")]
	pub cfi_code: Option<String>,
	/// It is recommended that <a href="tag_461_CFICode.html" target="bottom">CFICode&nbsp;(461)</a> be used instead of SecurityType for non-Fixed Income instruments. Futures and Options should be specified using the <a href="tag_461_CFICode.html" target="bottom">CFICode&nbsp;(461)</a> field instead of <a href="tag_167_SecurityType.html" target="bottom">SecurityType&nbsp;(167)</a> (Refer to Volume 7 - Recommendations and Guidelines for Futures and Options Markets.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "167")]
	pub security_type: Option<SecurityType>,
	/// Specifies the month and year of maturity. Applicable for standardized derivatives which are typically only referenced by month
	/// and year (e.g. S&amp;P futures). Note <a href="tag_541_MaturityDate.html" target="bottom">MaturityDate&nbsp;(541)</a> (a full date) can also be specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "200")]
	pub maturity_month_year: Option<fix_common::MonthYear>,
	/// Specifies date of maturity (a full date). Note that standardized derivatives which are typically only referenced by month
	/// and year (e.g. S&amp;P futures) may use <a href="tag_200_MaturityMonthYear.html" target="bottom">MaturityMonthYear&nbsp;(200)</a> and/or this field. When using <a href="tag_200_MaturityMonthYear.html" target="bottom">MaturityMonthYear&nbsp;(200)</a> , it is recommended that markets and sell sides report the <a href="tag_541_MaturityDate.html" target="bottom">MaturityDate&nbsp;(541)</a> on all outbound messages as a means of data enrichment.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "541")]
	pub maturity_date: Option<fix_common::LocalMktDate>,
	/// Date interest is to be paid. Used in identifying Corporate Bond issues.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "224")]
	pub coupon_payment_date: Option<fix_common::UTCDateOnly>,
	/// Date instrument was issued. For Fixed Income IOIs for new issues, specifies the issue date.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "225")]
	pub issue_date: Option<fix_common::UTCDateOnly>,
	/// Identifies the collateral used in the transaction. For Fixed Income, required for RP and RVRP security types.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "239")]
	pub repo_collateral_security_type: Option<RepoCollateralSecurityType>,
	/// Number of business days before repurchase of a repo.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "226")]
	pub repurchase_term: Option<i32>,
	/// Percent of par at which a Repo will be repaid. Represented as a percent, e.g. .9525 represents 95-1/4 percent of par.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "227")]
	pub repurchase_rate: Option<f32>,
	/// Fraction for deriving Current face from Original face for TIPS, ABS or MBS Fixed Income securities. Note the fraction may
	/// be greater than, equal to or less than 1.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "228")]
	pub factor: Option<f64>,
	/// CreditRating
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "255")]
	pub credit_rating: Option<String>,
	/// The location at which records of ownership are maintained for this instrument, and at which ownership changes must be recorded.
	/// Can be used in conjunction with ISIN to address ISIN uniqueness issues.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "543")]
	pub instr_registry: Option<String>,
	/// ISO Country code of instrument issue (e.g. the country portion typically used in ISIN). Can be used in conjunction with non-ISIN <a href="tag_48_SecurityID.html" target="bottom">SecurityID&nbsp;(48)</a> (e.g. CUSIP for Municipal Bonds without ISIN) to provide uniqueness.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "470")]
	pub country_of_issue: Option<CountryOfIssue>,
	/// A two-character state or province abbreviation.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "471")]
	pub state_or_province_of_issue: Option<String>,
	/// The three-character IATA code for a locale (e.g. airport code for Municipal Bonds).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "472")]
	pub locale_of_issue: Option<String>,
	/// Return of investor's principal in a security. Bond redemption can occur before maturity date.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "240")]
	pub redemption_date: Option<fix_common::UTCDateOnly>,
	/// Used for derivatives, such as options and covered warrants
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "202")]
	pub strike_price: Option<f64>,
	/// Used for derivatives, such as options and covered warrants to indicate a versioning of the contract when required due to corporate
	/// actions to the underlying. Should not be used to indicate type of option - use the <a href="tag_461_CFICode.html" target="bottom">CFICode&nbsp;(461)</a> for this purpose.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "206")]
	pub opt_attribute: Option<char>,
	/// For Fixed Income, Convertible Bonds, Derivatives, etc. Note: If used, quantities should be expressed in the "nominal" (e.g.
	/// contracts vs. shares) amount.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "231")]
	pub contract_multiplier: Option<f64>,
	/// For Fixed Income.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "223")]
	pub coupon_rate: Option<f32>,
	/// Can be used to identify the security.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "207")]
	pub security_exchange: Option<String>,
	/// Issuer
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "106")]
	pub issuer: Option<String>,
	/// Must be set if <a href="tag_349_EncodedIssuer.html" target="bottom">EncodedIssuer&nbsp;(349)</a> field is specified and must immediately precede it.
	#[serde(rename = "348")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_106_Issuer.html" target="bottom">Issuer&nbsp;(106)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "349")]
	pub encoded_issuer: Option<fix_common::EncodedText<349>>,
	/// SecurityDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "107")]
	pub security_desc: Option<String>,
	/// Must be set if <a href="tag_351_EncodedSecurityDesc.html" target="bottom">EncodedSecurityDesc&nbsp;(351)</a> field is specified and must immediately precede it.
	#[serde(rename = "350")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_107_SecurityDesc.html" target="bottom">SecurityDesc&nbsp;(107)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "351")]
	pub encoded_security_desc: Option<fix_common::EncodedText<351>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecurityAltI {
	/// Security Alternate identifier for this security. First member of repeating group - must be specified if <a href="tag_454_NoSecurityAltID.html" target="bottom">NoSecurityAltID&nbsp;(454)</a> &gt; 0. The Security Alternative identifier block should not be populated unless <a href="tag_48_SecurityID.html" target="bottom">SecurityID&nbsp;(48)</a> and <a href="tag_22_SecurityIDSource.html" target="bottom">SecurityIDSource&nbsp;(22)</a> are populated and should not duplicate the <a href="tag_48_SecurityID.html" target="bottom">SecurityID&nbsp;(48)</a> and <a href="tag_22_SecurityIDSource.html" target="bottom">SecurityIDSource&nbsp;(22)</a> values contained in the <a href="tag_48_SecurityID.html" target="bottom">SecurityID&nbsp;(48)</a> / <a href="tag_22_SecurityIDSource.html" target="bottom">SecurityIDSource&nbsp;(22)</a> tags. Use of <a href="tag_455_SecurityAltID.html" target="bottom">SecurityAltID&nbsp;(455)</a> may be used if bilaterally agreed to assist in security identification, and does not imply an obligation on the receiver of
	/// the message to ensure validity or consistency with the <a href="tag_48_SecurityID.html" target="bottom">SecurityID&nbsp;(48)</a> and <a href="tag_22_SecurityIDSource.html" target="bottom">SecurityIDSource&nbsp;(22)</a> fields which take precedence.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "455")]
	pub security_alt_id: Option<String>,
	/// Source of <a href="tag_455_SecurityAltID.html" target="bottom">SecurityAltID&nbsp;(455)</a> . Required if <a href="tag_455_SecurityAltID.html" target="bottom">SecurityAltID&nbsp;(455)</a> is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "456")]
	pub security_alt_id_source: Option<SecurityAltIDSource>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SecurityIDSource {
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

impl Default for SecurityIDSource {
	fn default() -> Self {
		SecurityIDSource::Cusip
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Product {
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

impl Default for Product {
	fn default() -> Self {
		Product::Agency
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SecurityType {
	/// Federal Agency Coupon
	#[serde(rename = "FAC")]
	FederalAgencyCoupon,
	/// Federal Agency Discount Note
	#[serde(rename = "FADN")]
	FederalAgencyDiscountNote,
	/// Private Export Funding Identify the Issuer in the <a href="tag_106_Issuer.html" target="bottom">Issuer&nbsp;(106)</a> field
	#[serde(rename = "PEF")]
	PrivateExportFundingIdentifyTheIssuerInTheIssuerField,
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

impl Default for SecurityType {
	fn default() -> Self {
		SecurityType::FederalAgencyCoupon
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum RepoCollateralSecurityType {
	/// Federal Agency Coupon
	#[serde(rename = "FAC")]
	FederalAgencyCoupon,
	/// Federal Agency Discount Note
	#[serde(rename = "FADN")]
	FederalAgencyDiscountNote,
	/// Private Export Funding Identify the Issuer in the <a href="tag_106_Issuer.html" target="bottom">Issuer&nbsp;(106)</a> field
	#[serde(rename = "PEF")]
	PrivateExportFundingIdentifyTheIssuerInTheIssuerField,
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

impl Default for RepoCollateralSecurityType {
	fn default() -> Self {
		RepoCollateralSecurityType::FederalAgencyCoupon
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum CountryOfIssue {
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

impl Default for CountryOfIssue {
	fn default() -> Self {
		CountryOfIssue::Afghanistan
	}
}



#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SecurityAltIDSource {
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

impl Default for SecurityAltIDSource {
	fn default() -> Self {
		SecurityAltIDSource::Cusip
	}
}
