
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecurityTypes {
	/// MsgType = w
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// SecurityReqID
	#[serde(rename = "320")]
	pub security_req_id: String,
	/// Identifier for the security response message
	#[serde(rename = "322")]
	pub security_response_id: String,
	/// The result of the security request identified by <a href="tag_320_SecurityReqID.html" target="bottom">SecurityReqID&nbsp;(320)</a>
	#[serde(rename = "323")]
	pub security_response_type: SecurityResponseType,
	/// Indicates total number of security types in the event that multiple <a href="message_Security_Types_w.html" target="main">Security Type&nbsp;(w)</a> messages are used to return results
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "557")]
	pub total_num_security_types: Option<i32>,
	/// NoSecurityTypes
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "558")]
	pub security_types: Option<fix_common::RepeatingValues<SecurityType>>,
	/// Comment, instructions, or other identifying information.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if <a href="tag_355_EncodedText.html" target="bottom">EncodedText&nbsp;(355)</a> field is specified and must immediately precede it.
	#[serde(rename = "354")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_58_Text.html" target="bottom">Text&nbsp;(58)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
	/// Optional Trading Session Identifier to specify a particular trading session for which you want to obtain a list of securities
	/// that are tradeable.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "336")]
	pub trading_session_id: Option<String>,
	/// TradingSessionSubID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "625")]
	pub trading_session_sub_id: Option<String>,
	/// Subscribe or unsubscribe for security status to security specified in request.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "263")]
	pub subscription_request_type: Option<SubscriptionRequestType>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecurityType {
	/// Required if <a href="tag_558_NoSecurityTypes.html" target="bottom">NoSecurityTypes&nbsp;(558)</a> &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "167")]
	pub security_type_item: Option<SecurityTypeItem>,
	/// Product
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "460")]
	pub product: Option<Product>,
	/// CFICode
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "461")]
	pub cfi_code: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SecurityResponseType {
	/// Accept security proposal as is
	#[serde(rename = "1")]
	AcceptSecurityProposalAsIs,
	/// Accept security proposal with revisions as indicated in the message
	#[serde(rename = "2")]
	AcceptSecurityProposalWithRevisionsAsIndicatedInTheMessage,
	/// List of security types returned per request
	#[serde(rename = "3")]
	ListOfSecurityTypesReturnedPerRequest,
	/// List of securities returned per request
	#[serde(rename = "4")]
	ListOfSecuritiesReturnedPerRequest,
	/// Reject security proposal
	#[serde(rename = "5")]
	RejectSecurityProposal,
	/// Can not match selection criteria
	#[serde(rename = "6")]
	CanNotMatchSelectionCriteria,
}

impl Default for SecurityResponseType {
	fn default() -> Self {
		SecurityResponseType::AcceptSecurityProposalAsIs
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SubscriptionRequestType {
	/// Snapshot
	#[serde(rename = "0")]
	Snapshot,
	/// Snapshot + Updates (Subscribe)
	#[serde(rename = "1")]
	SnapshotUpdates,
	/// Disable previous Snapshot + Update Request (Unsubscribe)
	#[serde(rename = "2")]
	DisablePreviousSnapshotUpdateRequest,
}

impl Default for SubscriptionRequestType {
	fn default() -> Self {
		SubscriptionRequestType::Snapshot
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SecurityTypeItem {
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

impl Default for SecurityTypeItem {
	fn default() -> Self {
		SecurityTypeItem::FederalAgencyCoupon
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
