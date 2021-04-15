
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SettlementInstructionRequest {
	/// MsgType = AV
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'A', 'V'>,
	/// Unique message ID
	#[serde(rename = "791")]
	pub settl_inst_req_id: String,
	/// Date/Time this request message was generated
	#[serde(rename = "60")]
	pub transact_time: fix_common::UTCTimestamp,
	/// Used here for party whose instructions this message is requesting and (optionally) for settlement location. Not required if
	/// database identifiers are being used to request settlement instructions. Required otherwise.
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// Should not be populated if <a href="tag_169_StandInstDbType.html" target="bottom">StandInstDbType&nbsp;(169)</a> is populated
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "79")]
	pub alloc_account: Option<String>,
	/// Required if <a href="tag_79_AllocAccount.html" target="bottom">AllocAccount&nbsp;(79)</a> populated. Should not be populated if <a href="tag_169_StandInstDbType.html" target="bottom">StandInstDbType&nbsp;(169)</a> is populated
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "661")]
	pub alloc_acct_id_source: Option<AllocAcctIDSource>,
	/// Should not be populated if <a href="tag_169_StandInstDbType.html" target="bottom">StandInstDbType&nbsp;(169)</a> is populated
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "54")]
	pub side: Option<Side>,
	/// Should not be populated if <a href="tag_169_StandInstDbType.html" target="bottom">StandInstDbType&nbsp;(169)</a> is populated
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "460")]
	pub product: Option<Product>,
	/// Should not be populated if <a href="tag_169_StandInstDbType.html" target="bottom">StandInstDbType&nbsp;(169)</a> is populated
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "167")]
	pub security_type: Option<SecurityType>,
	/// Should not be populated if <a href="tag_169_StandInstDbType.html" target="bottom">StandInstDbType&nbsp;(169)</a> is populated
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "461")]
	pub cfi_code: Option<String>,
	/// Should not be populated if <a href="tag_169_StandInstDbType.html" target="bottom">StandInstDbType&nbsp;(169)</a> is populated
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "168")]
	pub effective_time: Option<fix_common::UTCTimestamp>,
	/// Should not be populated if <a href="tag_169_StandInstDbType.html" target="bottom">StandInstDbType&nbsp;(169)</a> is populated
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "126")]
	pub expire_time: Option<fix_common::UTCTimestamp>,
	/// Should not be populated if <a href="tag_169_StandInstDbType.html" target="bottom">StandInstDbType&nbsp;(169)</a> is populated
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "779")]
	pub last_update_time: Option<fix_common::UTCTimestamp>,
	/// Should not be populated if any of <a href="tag_79_AllocAccount.html" target="bottom">AllocAccount&nbsp;(79)</a> through to <a href="tag_779_LastUpdateTime.html" target="bottom">LastUpdateTime&nbsp;(779)</a> are populated
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "169")]
	pub stand_inst_db_type: Option<StandInstDbType>,
	/// Should not be populated if any of <a href="tag_79_AllocAccount.html" target="bottom">AllocAccount&nbsp;(79)</a> through to <a href="tag_779_LastUpdateTime.html" target="bottom">LastUpdateTime&nbsp;(779)</a> are populated
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "170")]
	pub stand_inst_db_name: Option<String>,
	/// The identifier of the standing instructions within the database specified in <a href="tag_169_StandInstDbType.html" target="bottom">StandInstDbType&nbsp;(169)</a> . Required if <a href="tag_169_StandInstDbType.html" target="bottom">StandInstDbType&nbsp;(169)</a> populated. Should not be populated if any of <a href="tag_79_AllocAccount.html" target="bottom">AllocAccount&nbsp;(79)</a> through to <a href="tag_779_LastUpdateTime.html" target="bottom">LastUpdateTime&nbsp;(779)</a> are populated
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "171")]
	pub stand_inst_db_id: Option<String>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum AllocAcctIDSource {
	/// BIC
	#[serde(rename = "1")]
	Bic,
	/// SID code
	#[serde(rename = "2")]
	SidCode,
	/// TFM (GSPTA)
	#[serde(rename = "3")]
	Tfm,
	/// OMGEO (AlertID)
	#[serde(rename = "4")]
	Omgeo,
	/// DTCC code
	#[serde(rename = "5")]
	DtccCode,
	/// Other (custom or proprietary)
	#[serde(rename = "99")]
	Other,
}

impl Default for AllocAcctIDSource {
	fn default() -> Self {
		AllocAcctIDSource::Bic
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
	/// Subscribe (e.g. CIV)
	#[serde(rename = "D")]
	Subscribe,
	/// Redeem (e.g. CIV)
	#[serde(rename = "E")]
	Redeem,
	/// Lend (FINANCING - identifies direction of collateral)
	#[serde(rename = "F")]
	Lend,
	/// Borrow (FINANCING - identifies direction of collateral)
	#[serde(rename = "G")]
	Borrow,
}

impl Default for Side {
	fn default() -> Self {
		Side::Buy
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
	/// FINANCING
	#[serde(rename = "13")]
	Financing,
}

impl Default for Product {
	fn default() -> Self {
		Product::Agency
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SecurityType {
	/// Federal government or treasury
	#[serde(rename = "TREASURY")]
	Treasury,
	/// State, province, region, etc.
	#[serde(rename = "PROVINCE")]
	Province,
	/// Federal agency
	#[serde(rename = "AGENCY")]
	Agency,
	/// Mortgage passthrough
	#[serde(rename = "MORTGAGE")]
	Mortgage,
	/// Equity
	#[serde(rename = "EQUITY")]
	Equity,
	/// Cash
	#[serde(rename = "CASH")]
	Cash,
	/// Euro Supranational Coupons (Identify the Issuer in the "Issuer" field(106))
	#[serde(rename = "EUSUPRA")]
	Eusupra,
	/// Federal Agency Coupon
	#[serde(rename = "FAC")]
	Fac,
	/// Federal Agency Discount Note
	#[serde(rename = "FADN")]
	Fadn,
	/// Private Export Funding (Identify the Issuer in the 'Issuer' field(106))
	#[serde(rename = "PEF")]
	Pef,
	/// Supra-national agency/USD Supranational Coupons (Identify the Issuer in the "Issuer" field(106))
	#[serde(rename = "SUPRA")]
	Supra,
	/// Corporate Bond
	#[serde(rename = "CORP")]
	Corp,
	/// Corporate Private Placement
	#[serde(rename = "CPP")]
	Cpp,
	/// Convertible Bond
	#[serde(rename = "CB")]
	Cb,
	/// Dual Currency
	#[serde(rename = "DUAL")]
	Dual,
	/// Euro Corporate Bond
	#[serde(rename = "EUCORP")]
	Eucorp,
	/// Indexed Linked
	#[serde(rename = "XLINKD")]
	Xlinkd,
	/// Structured Notes
	#[serde(rename = "STRUCT")]
	Struct,
	/// Yankee Corporate Bond
	#[serde(rename = "YANK")]
	Yank,
	/// Foreign Exchange Contract
	#[serde(rename = "FOR")]
	For,
	/// Common Stock
	#[serde(rename = "CS")]
	Cs,
	/// Preferred Stock
	#[serde(rename = "PS")]
	Ps,
	/// Brady Bond
	#[serde(rename = "BRANDY")]
	Brandy,
	/// Euro Sovereigns (Identify the Issue Name in Issuer (106))
	#[serde(rename = "EUSOV")]
	Eusov,
	/// US Treasury Bond
	#[serde(rename = "TBOND")]
	Tbond,
	/// Interest strip from any bond or note
	#[serde(rename = "TINT")]
	Tint,
	/// Treasury Inflation Protected Securities
	#[serde(rename = "TIPS")]
	Tips,
	/// Principal strip of a callable bond or note
	#[serde(rename = "TCAL")]
	Tcal,
	/// Principal strip from a non-callable bond or note
	#[serde(rename = "TPRN")]
	Tprn,
	/// US Treasury Note (deprecated value, use "TNOTE")
	#[serde(rename = "UST")]
	Ust,
	/// US Treasury Bill (deprecated value, use "TBILL")
	#[serde(rename = "USTB")]
	Ustb,
	/// US Treasury Note
	#[serde(rename = "TNOTE")]
	Tnote,
	/// US Treasury Bill
	#[serde(rename = "TBILL")]
	Tbill,
	/// Repurchase
	#[serde(rename = "REPO")]
	Repo,
	/// Forward
	#[serde(rename = "FORWARD")]
	Forward,
	/// Buy Sellback
	#[serde(rename = "BUYSELL")]
	Buysell,
	/// Securities Loan
	#[serde(rename = "SECLOAN")]
	Secloan,
	/// Securities Pledge
	#[serde(rename = "SECPLEDGE")]
	Secpledge,
	/// Term Loan
	#[serde(rename = "TERM")]
	Term,
	/// Revolver Loan
	#[serde(rename = "RVLV")]
	Rvlv,
	/// Revolver/Term Loan
	#[serde(rename = "RVLVTRM")]
	Rvlvtrm,
	/// Bridge Loan
	#[serde(rename = "BRIDGE")]
	Bridge,
	/// Letter of Credit
	#[serde(rename = "LOFC")]
	Lofc,
	/// Swing Line Facility
	#[serde(rename = "SWING")]
	Swing,
	/// Debtor in Possession
	#[serde(rename = "DINP")]
	Dinp,
	/// Defaulted
	#[serde(rename = "DEFLTED")]
	Deflted,
	/// Withdrawn
	#[serde(rename = "WITHDRN")]
	Withdrn,
	/// Replaced
	#[serde(rename = "REPLACD")]
	Replacd,
	/// Matured
	#[serde(rename = "MATURED")]
	Matured,
	/// Amended &amp; Restated
	#[serde(rename = "AMENDED")]
	Amended,
	/// Retired
	#[serde(rename = "RETIRED")]
	Retired,
	/// Bankers Acceptance
	#[serde(rename = "BA")]
	Ba,
	/// Bank Notes
	#[serde(rename = "BN")]
	Bn,
	/// Bill of Exchanges
	#[serde(rename = "BOX")]
	Box,
	/// Certificate of Deposit
	#[serde(rename = "CD")]
	Cd,
	/// Call Loans
	#[serde(rename = "CL")]
	Cl,
	/// Commercial Paper
	#[serde(rename = "CP")]
	Cp,
	/// Deposit Notes
	#[serde(rename = "DN")]
	Dn,
	/// Euro Certificate of Deposit
	#[serde(rename = "EUCD")]
	Eucd,
	/// Euro Commercial Paper
	#[serde(rename = "EUCP")]
	Eucp,
	/// Liquidity Note
	#[serde(rename = "LQN")]
	Lqn,
	/// Medium Term Notes
	#[serde(rename = "MTN")]
	Mtn,
	/// Overnight
	#[serde(rename = "ONITE")]
	Onite,
	/// Promissory Note
	#[serde(rename = "PN")]
	Pn,
	/// Plazos Fijos
	#[serde(rename = "PZFJ")]
	Pzfj,
	/// Short Term Loan Note
	#[serde(rename = "STN")]
	Stn,
	/// Time Deposit
	#[serde(rename = "TD")]
	Td,
	/// Extended Comm Note
	#[serde(rename = "XCN")]
	Xcn,
	/// Yankee Certificate of Deposit
	#[serde(rename = "YCD")]
	Ycd,
	/// Asset-backed Securities
	#[serde(rename = "ABS")]
	Abs,
	/// Corp. Mortgage-backed Securities
	#[serde(rename = "CMBS")]
	Cmbs,
	/// Collateralized Mortgage Obligation
	#[serde(rename = "CMO")]
	Cmo,
	/// IOETTE Mortgage
	#[serde(rename = "IET")]
	Iet,
	/// Mortgage-backed Securities
	#[serde(rename = "MBS")]
	Mbs,
	/// Mortgage Interest Only
	#[serde(rename = "MIO")]
	Mio,
	/// Mortgage Principal Only
	#[serde(rename = "MPO")]
	Mpo,
	/// Mortgage Private Placement
	#[serde(rename = "MPP")]
	Mpp,
	/// Miscellaneous Pass-through
	#[serde(rename = "MPT")]
	Mpt,
	/// Pfandbriefe (Identify the Issue Name in Issuer (106))
	#[serde(rename = "PFAND")]
	Pfand,
	/// To be Announced
	#[serde(rename = "TBA")]
	Tba,
	/// Other Anticipation Notes BAN, GAN, etc.
	#[serde(rename = "AN")]
	An,
	/// Certificate of Obligation
	#[serde(rename = "COFO")]
	Cofo,
	/// Certificate of Participation
	#[serde(rename = "COFP")]
	Cofp,
	/// General Obligation Bonds
	#[serde(rename = "GO")]
	Go,
	/// Mandatory Tender
	#[serde(rename = "MT")]
	Mt,
	/// Revenue Anticipation Note
	#[serde(rename = "RAN")]
	Ran,
	/// Revenue Bonds
	#[serde(rename = "REV")]
	Rev,
	/// Special Assessment
	#[serde(rename = "SPCLA")]
	Spcla,
	/// Special Obligation
	#[serde(rename = "SPCLO")]
	Spclo,
	/// Special Tax
	#[serde(rename = "SPCLT")]
	Spclt,
	/// Tax Anticipation Note
	#[serde(rename = "TAN")]
	Tan,
	/// Tax Allocation
	#[serde(rename = "TAXA")]
	Taxa,
	/// Tax Exempt Commercial Paper
	#[serde(rename = "TECP")]
	Tecp,
	/// Tax &amp; Revenue Anticipation Note
	#[serde(rename = "TRAN")]
	Tran,
	/// Variable Rate Demand Note
	#[serde(rename = "VRDN")]
	Vrdn,
	/// Warrant
	#[serde(rename = "WAR")]
	War,
	/// Mutual Fund (i.e. any kind of open-ended "Collective Investment Vehicle")
	#[serde(rename = "MF")]
	Mf,
	/// Multi-leg instrument (e.g. options strategy or futures spread. <a href="tag_461_CFICode.html" target="bottom">CFICode&nbsp;(461)</a> can be used to identify if options-based, futures-based, etc.)
	#[serde(rename = "MLEG")]
	Mleg,
	/// No Security Type
	#[serde(rename = "NONE")]
	None,
	/// "Wildcard" entry (used on <a href="message_Security_Definition_Request_c.html" target="main">Security Definition Request&nbsp;(c)</a> message)
	#[serde(rename = "?")]
	WildcardEntryAMessage,
}

impl Default for SecurityType {
	fn default() -> Self {
		SecurityType::Treasury
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
	/// AccountNet
	#[serde(rename = "4")]
	AccountNet,
}

impl Default for StandInstDbType {
	fn default() -> Self {
		StandInstDbType::Other
	}
}
