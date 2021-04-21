
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SettlementInstructions {
	/// MsgType = T
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'T'>,
	/// Unique message ID regardless of <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a>
	#[serde(rename = "162")]
	pub settl_inst_id: String,
	/// New, Replace, or Cancel
	#[serde(rename = "163")]
	pub settl_inst_trans_type: SettlInstTransType,
	/// SettlInstMode
	#[serde(rename = "160")]
	pub settl_inst_mode: SettlInstMode,
	/// 1=Broker's Settlement Instructions, 2=Institution's Settlement Instructions
	#[serde(rename = "165")]
	pub settl_inst_source: SettlInstSource,
	/// Required for <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> =1, 2, or 3
	#[serde(rename = "79")]
	pub alloc_account: String,
	/// Required for <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> =2 or 3, may be required for <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> =1 (i.e. may not be required if <a href="tag_169_StandInstDbType.html" target="bottom">StandInstDbType&nbsp;(169)</a> and <a href="tag_171_StandInstDbID.html" target="bottom">StandInstDbID&nbsp;(171)</a> are used)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "166")]
	pub settl_location: Option<SettlLocation>,
	/// Required for <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> =2 or 3
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "75")]
	pub trade_date: Option<fix_common::UTCDateOnly>,
	/// Required for <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> =2 or 3
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "70")]
	pub alloc_id: Option<String>,
	/// Required for <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> =2 or 3, May be required for <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> =1
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "30")]
	pub last_mkt: Option<String>,
	/// Required for <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> =2 or 3, May be required for <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> =1
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
	pub effective_time: Option<fix_common::UTCTimeOnly>,
	/// Date/Time <a href="message_Settlement_Instructions_T.html" target="main">Settlement Instructions&nbsp;(T)</a> were generated
	#[serde(rename = "60")]
	pub transact_time: fix_common::UTCTimeOnly,
	/// Used for firm identification in third-party transactions.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "109")]
	pub client_id: Option<String>,
	/// Used for firm identification in third-party transactions.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "76")]
	pub exec_broker: Option<String>,
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
	/// Applicable when <a href="tag_166_SettlLocation.html" target="bottom">SettlLocation&nbsp;(166)</a> is a depository
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
	/// Applicable when <a href="tag_172_SettlDeliveryType.html" target="bottom">SettlDeliveryType&nbsp;(172)</a> =Free
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "182")]
	pub cash_settl_agent_name: Option<String>,
	/// Applicable when <a href="tag_172_SettlDeliveryType.html" target="bottom">SettlDeliveryType&nbsp;(172)</a> =Free
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "183")]
	pub cash_settl_agent_code: Option<String>,
	/// Applicable when <a href="tag_172_SettlDeliveryType.html" target="bottom">SettlDeliveryType&nbsp;(172)</a> =Free
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "184")]
	pub cash_settl_agent_acct_num: Option<String>,
	/// Applicable when <a href="tag_172_SettlDeliveryType.html" target="bottom">SettlDeliveryType&nbsp;(172)</a> =Free
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "185")]
	pub cash_settl_agent_acct_name: Option<String>,
	/// Applicable when <a href="tag_172_SettlDeliveryType.html" target="bottom">SettlDeliveryType&nbsp;(172)</a> =Free
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "186")]
	pub cash_settl_agent_contact_name: Option<String>,
	/// Applicable when <a href="tag_172_SettlDeliveryType.html" target="bottom">SettlDeliveryType&nbsp;(172)</a> =Free
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "187")]
	pub cash_settl_agent_contact_phone: Option<String>,
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
}

impl Default for SettlInstSource {
	fn default() -> Self {
		SettlInstSource::BrokerSInstructions
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SettlLocation {
	/// CEDEL
	#[serde(rename = "CED")]
	Cedel,
	/// Depository Trust Company
	#[serde(rename = "DTC")]
	DepositoryTrustCompany,
	/// Euroclear
	#[serde(rename = "EUR")]
	Euroclear,
	/// Federal Book Entry
	#[serde(rename = "FED")]
	FederalBookEntry,
	/// Physical
	#[serde(rename = "PNY")]
	Physical,
	/// Participant Trust Company
	#[serde(rename = "PTC")]
	ParticipantTrustCompany,
	/// Local Market Settle Location
	#[serde(rename = "ISO Country Code")]
	LocalMarketSettleLocation,
}

impl Default for SettlLocation {
	fn default() -> Self {
		SettlLocation::Cedel
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
	/// Undisclosed (valid for <a href="message_Indication_of_Interest_6.html" target="main">Indication of Interest&nbsp;(6)</a> and <a href="message_New_Order_List_E.html" target="main">List Order&nbsp;(E)</a> messages only)
	#[serde(rename = "7")]
	UndisclosedAAndAHrefMessageNewOrderListEHtmlTargetMainListOrderNbspAMessagesOnly,
	/// Cross (orders where counterparty is an exchange, valid for all messages except IOIs)
	#[serde(rename = "8")]
	Cross,
}

impl Default for Side {
	fn default() -> Self {
		Side::Buy
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SecurityType {
	/// Bankers Acceptance
	#[serde(rename = "BA")]
	BankersAcceptance,
	/// Certificate Of Deposit
	#[serde(rename = "CD")]
	CertificateOfDeposit,
	/// Collateralized Mortgage Obligation
	#[serde(rename = "CMO")]
	CollateralizedMortgageObligation,
	/// Corporate Bond
	#[serde(rename = "CORP")]
	CorporateBond,
	/// Commercial Paper
	#[serde(rename = "CP")]
	CommercialPaper,
	/// Corporate Private Placement
	#[serde(rename = "CPP")]
	CorporatePrivatePlacement,
	/// Common Stock
	#[serde(rename = "CS")]
	CommonStock,
	/// Federal Housing Authority
	#[serde(rename = "FHA")]
	FederalHousingAuthority,
	/// Federal Home Loan
	#[serde(rename = "FHL")]
	FederalHomeLoan,
	/// Federal National Mortgage Association
	#[serde(rename = "FN")]
	FederalNationalMortgageAssociation,
	/// Foreign Exchange Contract
	#[serde(rename = "FOR")]
	ForeignExchangeContract,
	/// Future
	#[serde(rename = "FUT")]
	Future,
	/// Government National Mortgage Association
	#[serde(rename = "GN")]
	GovernmentNationalMortgageAssociation,
	/// Treasuries + Agency Debenture
	#[serde(rename = "GOVT")]
	TreasuriesAgencyDebenture,
	/// IOETTE Mortgage
	#[serde(rename = "IET")]
	IoetteMortgage,
	/// Mutual Fund
	#[serde(rename = "MF")]
	MutualFund,
	/// Mortgage Interest Only
	#[serde(rename = "MIO")]
	MortgageInterestOnly,
	/// Mortgage Principal Only
	#[serde(rename = "MPO")]
	MortgagePrincipalOnly,
	/// Mortgage Private Placement
	#[serde(rename = "MPP")]
	MortgagePrivatePlacement,
	/// Miscellaneous Pass-Thru
	#[serde(rename = "MPT")]
	MiscellaneousPassThru,
	/// Municipal Bond
	#[serde(rename = "MUNI")]
	MunicipalBond,
	/// No ISITC Security Type
	#[serde(rename = "NONE")]
	NoIsitcSecurityType,
	/// Option
	#[serde(rename = "OPT")]
	Option,
	/// Preferred Stock
	#[serde(rename = "PS")]
	PreferredStock,
	/// Repurchase Agreement
	#[serde(rename = "RP")]
	RepurchaseAgreement,
	/// Reverse Repurchase Agreement
	#[serde(rename = "RVRP")]
	ReverseRepurchaseAgreement,
	/// Student Loan Marketing Association
	#[serde(rename = "SL")]
	StudentLoanMarketingAssociation,
	/// Time Deposit
	#[serde(rename = "TD")]
	TimeDeposit,
	/// US Treasury Bill
	#[serde(rename = "USTB")]
	UsTreasuryBill,
	/// Warrant
	#[serde(rename = "WAR")]
	Warrant,
	/// Cats, Tigers &amp; Lions (a real code: US Treasury Receipts)
	#[serde(rename = "ZOO")]
	CatsTigersAmpLions,
}

impl Default for SecurityType {
	fn default() -> Self {
		SecurityType::BankersAcceptance
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
	/// A Global Custodian (StandInstDbName must be provided)
	#[serde(rename = "3")]
	AGlobalCustodian,
}

impl Default for StandInstDbType {
	fn default() -> Self {
		StandInstDbType::Other
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SettlDeliveryType {
	/// 'Versus. Payment': Deliver (if Sell) or Receive (if Buy) vs. (Against) Payment
	#[serde(rename = "0")]
	VersusPaymentDeliverOrReceiveVsPayment,
	/// 'Free': Deliver (if Sell) or Receive (if Buy) Free
	#[serde(rename = "1")]
	FreeDeliverOrReceiveFree,
}

impl Default for SettlDeliveryType {
	fn default() -> Self {
		SettlDeliveryType::VersusPaymentDeliverOrReceiveVsPayment
	}
}
