
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SettlementInstructions {
	/// MsgType = T
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'T', ' '>,
	/// Unique message ID regardless of <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a>
	#[serde(rename = "162")]
	pub settl_inst_id: String,
	/// New, Replace, or Cancel
	#[serde(rename = "163")]
	pub settl_inst_trans_type: SettlInstTransType,
	/// Required for Cancel and Replace <a href="tag_163_SettlInstTransType.html" target="bottom">SettlInstTransType&nbsp;(163)</a> messages
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "214")]
	pub settl_inst_ref_id: Option<String>,
	/// 1=Standing Instructions, 2=Specific Allocation Account Overriding, 3=Specific Allocation Account Standing , 4=Specific Order
	#[serde(rename = "160")]
	pub settl_inst_mode: SettlInstMode,
	/// 1=Broker's Settlement Instructions, 2=Institution's Settlement Instructions , 3=Investor
	#[serde(rename = "165")]
	pub settl_inst_source: SettlInstSource,
	/// Required for <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> =1, 2, or 3
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "79")]
	pub alloc_account: Option<String>,
	/// IndividualAllocID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "467")]
	pub individual_alloc_id: Option<String>,
	/// Required for <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> =4.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "11")]
	pub cl_ord_id: Option<String>,
	/// Required for <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> =2 or 3
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "75")]
	pub trade_date: Option<fix_common::LocalMktDate>,
	/// Required for <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> =2 or 3
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "70")]
	pub alloc_id: Option<String>,
	/// Required for <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> =2 or 3, may be required for <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> =1
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "30")]
	pub last_mkt: Option<String>,
	/// TradingSessionID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "336")]
	pub trading_session_id: Option<String>,
	/// TradingSessionSubID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "625")]
	pub trading_session_sub_id: Option<String>,
	/// Required for <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> =2 or 3, may be required for <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> =1
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "54")]
	pub side: Option<Side>,
	/// May be required for <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> =1
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "167")]
	pub security_type: Option<SecurityType>,
	/// May be required for <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> =1 (timestamp when it goes in to effect)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "168")]
	pub effective_time: Option<fix_common::UTCTimestamp>,
	/// Date/Time Settlement Instructions were generated
	#[serde(rename = "60")]
	pub transact_time: fix_common::UTCTimestamp,
	/// Parties
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// 1=DTC SID, 2=Thomson ALERT, 3=Global Custodian's, etc.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "169")]
	pub stand_inst_db_type: Option<StandInstDbType>,
	/// Name of <a href="tag_169_StandInstDbType.html" target="bottom">StandInstDbType&nbsp;(169)</a> (i.e. DTC, Global Custodian's name)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "170")]
	pub stand_inst_db_name: Option<String>,
	/// Identifier used within the <a href="tag_169_StandInstDbType.html" target="bottom">StandInstDbType&nbsp;(169)</a>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "171")]
	pub stand_inst_db_id: Option<String>,
	/// SettlDeliveryType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "172")]
	pub settl_delivery_type: Option<SettlDeliveryType>,
	/// Applicable when <a href="tag_452_PartyRole.html" target="bottom">PartyRole&nbsp;(452)</a> ="Settlement Location" and <a href="tag_448_PartyID.html" target="bottom">PartyID&nbsp;(448)</a> value is a depository
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "173")]
	pub settl_depository_code: Option<String>,
	/// SettlBrkrCode
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "174")]
	pub settl_brkr_code: Option<String>,
	/// SettlInstCode
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "175")]
	pub settl_inst_code: Option<String>,
	/// Applicable when settlement is being performed at a country vs. a depository
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "176")]
	pub security_settl_agent_name: Option<String>,
	/// Applicable when settlement is being performed at a country vs. a depository
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "177")]
	pub security_settl_agent_code: Option<String>,
	/// Applicable when settlement is being performed at a country vs. a depository
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "178")]
	pub security_settl_agent_acct_num: Option<String>,
	/// Applicable when settlement is being performed at a country vs. a depository
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "179")]
	pub security_settl_agent_acct_name: Option<String>,
	/// Applicable when settlement is being performed at a country vs. a depository
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "180")]
	pub security_settl_agent_contact_name: Option<String>,
	/// Applicable when settlement is being performed at a country vs. a depository
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "181")]
	pub security_settl_agent_contact_phone: Option<String>,
	/// Applicable when <a href="tag_172_SettlDeliveryType.html" target="bottom">SettlDeliveryType&nbsp;(172)</a> =Free. For CIV - applicable when settlement is between fund manager and intermediary, investor etc.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "182")]
	pub cash_settl_agent_name: Option<String>,
	/// Applicable when <a href="tag_172_SettlDeliveryType.html" target="bottom">SettlDeliveryType&nbsp;(172)</a> =Free. For CIV - applicable when settlement is between fund manager and intermediary, investor etc.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "183")]
	pub cash_settl_agent_code: Option<String>,
	/// Applicable when <a href="tag_172_SettlDeliveryType.html" target="bottom">SettlDeliveryType&nbsp;(172)</a> =Free. For CIV - applicable when settlement is between fund manager and intermediary, investor etc.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "184")]
	pub cash_settl_agent_acct_num: Option<String>,
	/// Applicable when <a href="tag_172_SettlDeliveryType.html" target="bottom">SettlDeliveryType&nbsp;(172)</a> =Free. For CIV - applicable when settlement is between fund manager and intermediary, investor etc.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "185")]
	pub cash_settl_agent_acct_name: Option<String>,
	/// Applicable when <a href="tag_172_SettlDeliveryType.html" target="bottom">SettlDeliveryType&nbsp;(172)</a> =Free. For CIV - applicable when settlement is between fund manager and intermediary, investor etc.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "186")]
	pub cash_settl_agent_contact_name: Option<String>,
	/// Applicable when <a href="tag_172_SettlDeliveryType.html" target="bottom">SettlDeliveryType&nbsp;(172)</a> =Free. For CIV - applicable when settlement is between fund manager and intermediary, investor etc.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "187")]
	pub cash_settl_agent_contact_phone: Option<String>,
	/// PaymentMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "492")]
	pub payment_method: Option<PaymentMethod>,
	/// PaymentRef
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "476")]
	pub payment_ref: Option<String>,
	/// CardHolderName
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "488")]
	pub card_holder_name: Option<String>,
	/// CardNumber
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "489")]
	pub card_number: Option<String>,
	/// CardStartDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "503")]
	pub card_start_date: Option<fix_common::LocalMktDate>,
	/// CardExpDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "490")]
	pub card_exp_date: Option<fix_common::LocalMktDate>,
	/// CardIssNum
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "491")]
	pub card_iss_num: Option<String>,
	/// PaymentDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "504")]
	pub payment_date: Option<fix_common::LocalMktDate>,
	/// PaymentRemitterID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "505")]
	pub payment_remitter_id: Option<String>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SettlInstTransType {
	/// New
	#[serde(rename = "N")]
	New,
	/// Cancel
	#[serde(rename = "C")]
	Cancel,
	/// Replace
	#[serde(rename = "R")]
	Replace,
}

impl Default for SettlInstTransType {
	fn default() -> Self {
		SettlInstTransType::New
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SettlInstMode {
	/// Default
	#[serde(rename = "0")]
	Default,
	/// Standing Instructions Provided
	#[serde(rename = "1")]
	StandingInstructionsProvided,
	/// Specific Allocation Account Overriding
	#[serde(rename = "2")]
	SpecificAllocationAccountOverriding,
	/// Specific Allocation Account Standing
	#[serde(rename = "3")]
	SpecificAllocationAccountStanding,
	/// Specific Order for a single account (for CIV)
	#[serde(rename = "4")]
	SpecificOrderForASingleAccount,
}

impl Default for SettlInstMode {
	fn default() -> Self {
		SettlInstMode::Default
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SettlInstSource {
	/// Broker's Instructions
	#[serde(rename = "1")]
	BrokerSInstructions,
	/// Institution's Instructions
	#[serde(rename = "2")]
	InstitutionSInstructions,
	/// Investor (e.g. CIV use)
	#[serde(rename = "3")]
	Investor,
}

impl Default for SettlInstSource {
	fn default() -> Self {
		SettlInstSource::BrokerSInstructions
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Side {
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

impl Default for Side {
	fn default() -> Self {
		Side::Buy
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
pub enum StandInstDbType {
	/// Other
	#[serde(rename = "0")]
	Other,
	/// DTC SID
	#[serde(rename = "1")]
	DtcSid,
	/// Thomson ALERT
	#[serde(rename = "2")]
	ThomsonAlert,
	/// A Global Custodian ( <a href="tag_170_StandInstDbName.html" target="bottom">StandInstDbName&nbsp;(170)</a> must be provided)
	#[serde(rename = "3")]
	AGlobalCustodianAMustBeProvided,
}

impl Default for StandInstDbType {
	fn default() -> Self {
		StandInstDbType::Other
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SettlDeliveryType {
	/// "Versus. Payment": Deliver (if Sell) or Receive (if Buy) vs. (Against) Payment
	#[serde(rename = "0")]
	VersusPaymentDeliverOrReceiveVsPayment,
	/// "Free": Deliver (if Sell) or Receive (if Buy) Free
	#[serde(rename = "1")]
	FreeDeliverOrReceiveFree,
}

impl Default for SettlDeliveryType {
	fn default() -> Self {
		SettlDeliveryType::VersusPaymentDeliverOrReceiveVsPayment
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PaymentMethod {
	/// CREST
	#[serde(rename = "1")]
	Crest,
	/// NSCC
	#[serde(rename = "2")]
	Nscc,
	/// Euroclear
	#[serde(rename = "3")]
	Euroclear,
	/// Clearstream
	#[serde(rename = "4")]
	Clearstream,
	/// Cheque
	#[serde(rename = "5")]
	Cheque,
	/// Telegraphic Transfer
	#[serde(rename = "6")]
	TelegraphicTransfer,
	/// Fed Wire
	#[serde(rename = "7")]
	FedWire,
	/// Debit Card
	#[serde(rename = "8")]
	DebitCard,
	/// Direct Debit (BECS)
	#[serde(rename = "9")]
	DirectDebit,
	/// Direct Credit (BECS)
	#[serde(rename = "10")]
	DirectCredit,
	/// Credit Card
	#[serde(rename = "11")]
	CreditCard,
	/// ACH Debit
	#[serde(rename = "12")]
	AchDebit,
	/// ACH Credit
	#[serde(rename = "13")]
	AchCredit,
	/// BPAY
	#[serde(rename = "14")]
	Bpay,
	/// High Value Clearing System (HVACS)
	#[serde(rename = "15")]
	HighValueClearingSystem,
}

impl Default for PaymentMethod {
	fn default() -> Self {
		PaymentMethod::Crest
	}
}
