
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AllocationInstructionAck {
	/// MsgType = P
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'P', ' '>,
	/// AllocID
	#[serde(rename = "70")]
	pub alloc_id: String,
	/// Parties
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// Optional second identifier for the <a href="message_Allocation_Instruction_J.html" target="main">allocation instruction&nbsp;(J)</a> being acknowledged (need not be unique)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "793")]
	pub secondary_alloc_id: Option<String>,
	/// TradeDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "75")]
	pub trade_date: Option<fix_common::LocalMktDate>,
	/// Date/Time <a href="message_Allocation_Instruction_Ack_P.html" target="main">Allocation Instruction Ack&nbsp;(P)</a> generated
	#[serde(rename = "60")]
	pub transact_time: fix_common::UTCTimestamp,
	/// Denotes the status of the <a href="message_Allocation_Instruction_J.html" target="main">Allocation Instruction&nbsp;(J)</a> ; received (but not yet processed), rejected (at block or account level) or accepted (and processed).
	#[serde(rename = "87")]
	pub alloc_status: AllocStatus,
	/// Required for <a href="tag_87_AllocStatus.html" target="bottom">AllocStatus&nbsp;(87)</a> = 1 (block level reject) and for <a href="tag_87_AllocStatus.html" target="bottom">AllocStatus&nbsp;(87)</a> 2 (account level reject) if the individual accounts and reject reasons are not provided in this message
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "88")]
	pub alloc_rej_code: Option<AllocRejCode>,
	/// AllocType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "626")]
	pub alloc_type: Option<AllocType>,
	/// Required if <a href="tag_626_AllocType.html" target="bottom">AllocType&nbsp;(626)</a> = 8 (Request to Intermediary). Indicates status that is requested to be transmitted to counterparty by the intermediary (i.e.
	/// clearing house)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "808")]
	pub alloc_intermed_req_type: Option<AllocIntermedReqType>,
	/// Required for <a href="tag_87_AllocStatus.html" target="bottom">AllocStatus&nbsp;(87)</a> = 0 (accepted). Denotes whether the financial details provided on the allocation instruction were successfully matched.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "573")]
	pub match_status: Option<MatchStatus>,
	/// Product
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "460")]
	pub product: Option<Product>,
	/// SecurityType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "167")]
	pub security_type: Option<SecurityType>,
	/// Can include explanation for <a href="tag_88_AllocRejCode.html" target="bottom">AllocRejCode&nbsp;(88)</a> = 7 (other)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if <a href="tag_355_EncodedText.html" target="bottom">EncodedText&nbsp;(355)</a> field is specified and must immediately precede it.
	#[serde(rename = "354")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_58_Text.html" target="bottom">Text&nbsp;(58)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
	/// This repeating group is optionally used for messages with <a href="tag_87_AllocStatus.html" target="bottom">AllocStatus&nbsp;(87)</a> = 2 (account level reject) to provide details of the individual accounts that caused the rejection, together with reject reasons.
	/// This group should not be populated when <a href="tag_87_AllocStatus.html" target="bottom">AllocStatus&nbsp;(87)</a> has any other value. Indicates number of allocation groups to follow.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "78")]
	pub allocs: Option<fix_common::RepeatingValues<Alloc>>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Alloc {
	/// Required if <a href="tag_78_NoAllocs.html" target="bottom">NoAllocs&nbsp;(78)</a> &gt; 0. Must be first field in repeating group.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "79")]
	pub alloc_account: Option<String>,
	/// Required if <a href="tag_78_NoAllocs.html" target="bottom">NoAllocs&nbsp;(78)</a> &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "661")]
	pub alloc_acct_id_source: Option<AllocAcctIDSource>,
	/// Used when performing "executed price" vs. "average price" allocations (e.g. Japan). <a href="tag_79_AllocAccount.html" target="bottom">AllocAccount&nbsp;(79)</a> plus <a href="tag_366_AllocPrice.html" target="bottom">AllocPrice&nbsp;(366)</a> form a unique Allocs entry. Used in lieu of <a href="tag_153_AllocAvgPx.html" target="bottom">AllocAvgPx&nbsp;(153)</a> .
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "366")]
	pub alloc_price: Option<f64>,
	/// IndividualAllocID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "467")]
	pub individual_alloc_id: Option<String>,
	/// Required if <a href="tag_78_NoAllocs.html" target="bottom">NoAllocs&nbsp;(78)</a> &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "776")]
	pub individual_alloc_rej_code: Option<IndividualAllocRejCode>,
	/// Free format text field related to this <a href="tag_79_AllocAccount.html" target="bottom">AllocAccount&nbsp;(79)</a> (can be used here to hold text relating to the rejection of this <a href="tag_79_AllocAccount.html" target="bottom">AllocAccount&nbsp;(79)</a> )
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "161")]
	pub alloc_text: Option<String>,
	/// Must be set if <a href="tag_361_EncodedAllocText.html" target="bottom">EncodedAllocText&nbsp;(361)</a> field is specified and must immediately precede it.
	#[serde(rename = "360")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_161_AllocText.html" target="bottom">AllocText&nbsp;(161)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "361")]
	pub encoded_alloc_text: Option<fix_common::EncodedText<361>>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum AllocStatus {
	/// accepted (successfully processed)
	#[serde(rename = "0")]
	Accepted,
	/// block level reject
	#[serde(rename = "1")]
	BlockLevelReject,
	/// account level reject
	#[serde(rename = "2")]
	AccountLevelReject,
	/// received (received, not yet processed)
	#[serde(rename = "3")]
	Received,
	/// incomplete
	#[serde(rename = "4")]
	Incomplete,
	/// rejected by intermediary
	#[serde(rename = "5")]
	RejectedByIntermediary,
}

impl Default for AllocStatus {
	fn default() -> Self {
		AllocStatus::Accepted
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum AllocRejCode {
	/// unknown account(s)
	#[serde(rename = "0")]
	UnknownAccount,
	/// incorrect quantity
	#[serde(rename = "1")]
	IncorrectQuantity,
	/// incorrect average price
	#[serde(rename = "2")]
	IncorrectAveragePrice,
	/// unknown executing broker mnemonic
	#[serde(rename = "3")]
	UnknownExecutingBrokerMnemonic,
	/// commission difference
	#[serde(rename = "4")]
	CommissionDifference,
	/// unknown <a href="tag_37_OrderID.html" target="bottom">OrderID&nbsp;(37)</a>
	#[serde(rename = "5")]
	UnknownAHrefTag37OrderIdHtmlTargetBottomOrderIdNbspA,
	/// unknown <a href="tag_66_ListID.html" target="bottom">ListID&nbsp;(66)</a>
	#[serde(rename = "6")]
	UnknownAHrefTag66ListIdHtmlTargetBottomListIdNbspA,
	/// other (further in Note 58=)
	#[serde(rename = "7")]
	Other,
	/// incorrect allocated quantity
	#[serde(rename = "8")]
	IncorrectAllocatedQuantity,
	/// calculation difference
	#[serde(rename = "9")]
	CalculationDifference,
	/// unknown or stale <a href="tag_17_ExecID.html" target="bottom">ExecID&nbsp;(17)</a>
	#[serde(rename = "10")]
	UnknownOrStaleAHrefTag17ExecIdHtmlTargetBottomExecIdNbspA,
	/// mismatched data value (further in Note 58=)
	#[serde(rename = "11")]
	MismatchedDataValue,
	/// unknown <a href="tag_11_ClOrdID.html" target="bottom">ClOrdID&nbsp;(11)</a>
	#[serde(rename = "12")]
	UnknownAHrefTag11ClOrdIdHtmlTargetBottomClOrdIdNbspA,
	/// warehouse request rejected
	#[serde(rename = "13")]
	WarehouseRequestRejected,
}

impl Default for AllocRejCode {
	fn default() -> Self {
		AllocRejCode::UnknownAccount
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum AllocType {
	/// Calculated (includes MiscFees and NetMoney)
	#[serde(rename = "1")]
	Calculated,
	/// Preliminary (without MiscFees and NetMoney)
	#[serde(rename = "2")]
	Preliminary,
	/// Ready-To-Book
	#[serde(rename = "5")]
	ReadyToBook,
	/// Warehouse instruction
	#[serde(rename = "7")]
	WarehouseInstruction,
	/// Request to Intermediary
	#[serde(rename = "8")]
	RequestToIntermediary,
}

impl Default for AllocType {
	fn default() -> Self {
		AllocType::Calculated
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum AllocIntermedReqType {
	/// Pending Accept
	#[serde(rename = "1")]
	PendingAccept,
	/// Pending Release
	#[serde(rename = "2")]
	PendingRelease,
	/// Pending Reversal
	#[serde(rename = "3")]
	PendingReversal,
	/// Accept
	#[serde(rename = "4")]
	Accept,
	/// Block Level Reject
	#[serde(rename = "5")]
	BlockLevelReject,
	/// Account Level Reject
	#[serde(rename = "6")]
	AccountLevelReject,
}

impl Default for AllocIntermedReqType {
	fn default() -> Self {
		AllocIntermedReqType::PendingAccept
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MatchStatus {
	/// compared, matched or affirmed
	#[serde(rename = "0")]
	ComparedMatchedOrAffirmed,
	/// uncompared, unmatched, or unaffirmed
	#[serde(rename = "1")]
	UncomparedUnmatchedOrUnaffirmed,
	/// Advisory or alert
	#[serde(rename = "2")]
	AdvisoryOrAlert,
}

impl Default for MatchStatus {
	fn default() -> Self {
		MatchStatus::ComparedMatchedOrAffirmed
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
pub enum IndividualAllocRejCode {
	/// unknown account(s)
	#[serde(rename = "0")]
	UnknownAccount,
	/// incorrect quantity
	#[serde(rename = "1")]
	IncorrectQuantity,
	/// incorrect average price
	#[serde(rename = "2")]
	IncorrectAveragePrice,
	/// unknown executing broker mnemonic
	#[serde(rename = "3")]
	UnknownExecutingBrokerMnemonic,
	/// commission difference
	#[serde(rename = "4")]
	CommissionDifference,
	/// unknown <a href="tag_37_OrderID.html" target="bottom">OrderID&nbsp;(37)</a>
	#[serde(rename = "5")]
	UnknownAHrefTag37OrderIdHtmlTargetBottomOrderIdNbspA,
	/// unknown <a href="tag_66_ListID.html" target="bottom">ListID&nbsp;(66)</a>
	#[serde(rename = "6")]
	UnknownAHrefTag66ListIdHtmlTargetBottomListIdNbspA,
	/// other (further in Note 58=)
	#[serde(rename = "7")]
	Other,
	/// incorrect allocated quantity
	#[serde(rename = "8")]
	IncorrectAllocatedQuantity,
	/// calculation difference
	#[serde(rename = "9")]
	CalculationDifference,
	/// unknown or stale <a href="tag_17_ExecID.html" target="bottom">ExecID&nbsp;(17)</a>
	#[serde(rename = "10")]
	UnknownOrStaleAHrefTag17ExecIdHtmlTargetBottomExecIdNbspA,
	/// mismatched data value (further in Note 58=)
	#[serde(rename = "11")]
	MismatchedDataValue,
	/// unknown <a href="tag_11_ClOrdID.html" target="bottom">ClOrdID&nbsp;(11)</a>
	#[serde(rename = "12")]
	UnknownAHrefTag11ClOrdIdHtmlTargetBottomClOrdIdNbspA,
	/// warehouse request rejected
	#[serde(rename = "13")]
	WarehouseRequestRejected,
}

impl Default for IndividualAllocRejCode {
	fn default() -> Self {
		IndividualAllocRejCode::UnknownAccount
	}
}
