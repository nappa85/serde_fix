
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Allocation {
	/// MsgType = AT
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// AllocReportID
	#[serde(rename = "755")]
	pub alloc_report_id: String,
	/// AllocID
	#[serde(rename = "70")]
	pub alloc_id: String,
	/// Indicates Clearing Business Date for which transaction will be settled.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "715")]
	pub clearing_business_date: Option<fix_common::LocalMktDate>,
	/// Indicates if an allocation is to be average priced. Is also used to indicate if average price allocation group is complete
	/// or incomplete.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "819")]
	pub avg_px_indicator: Option<AvgPxIndicator>,
	/// Quantity
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "53")]
	pub quantity: Option<f64>,
	/// AllocTransType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "71")]
	pub alloc_trans_type: Option<AllocTransType>,
	/// Insert here the set of "Parties" (firm identification) fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// Optional second identifier for the allocation report being acknowledged (need not be unique)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "793")]
	pub secondary_alloc_id: Option<String>,
	/// TradeDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "75")]
	pub trade_date: Option<fix_common::LocalMktDate>,
	/// Date/Time Allocation Report Ack generated
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<fix_common::UTCTimestamp>,
	/// Denotes the status of the allocation report; received (but not yet processed), rejected (at block or account level) or accepted
	/// (and processed). AllocStatus will be conditionally required in a 2-party model when used by a counterparty to convey a change
	/// in status. It will be optional in a 3-party model in which only the central counterparty may issue the status of an allocation.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "87")]
	pub alloc_status: Option<AllocStatus>,
	/// Required for <a href="tag_87_AllocStatus.html" target="bottom">AllocStatus&nbsp;(87)</a> = 1 ( block level reject) and for <a href="tag_87_AllocStatus.html" target="bottom">AllocStatus&nbsp;(87)</a> 2 (account level reject) if the individual accounts and reject reasons are not provided in this message.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "88")]
	pub alloc_rej_code: Option<AllocRejCode>,
	/// AllocReportType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "794")]
	pub alloc_report_type: Option<AllocReportType>,
	/// Required if <a href="tag_794_AllocReportType.html" target="bottom">AllocReportType&nbsp;(794)</a> = 8 (Request to Intermediary) Indicates status that is requested to be transmitted to counterparty by the intermediary (i.e.
	/// clearing house)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "808")]
	pub alloc_intermed_req_type: Option<AllocIntermedReqType>,
	/// Denotes whether the financial details provided on the Allocation Report were successfully matched.
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
	/// This repeating group is optionally used for messages with <a href="tag_87_AllocStatus.html" target="bottom">AllocStatus&nbsp;(87)</a> = 2 (account level reject), <a href="tag_87_AllocStatus.html" target="bottom">AllocStatus&nbsp;(87)</a> = 0 (accepted), to provide details of the individual accounts that were accepted or rejected. In the case of a reject, the
	/// reasons for the rejection should be specified. This group should not be populated where <a href="tag_87_AllocStatus.html" target="bottom">AllocStatus&nbsp;(87)</a> has any other value. Indicates number of allocation groups to follow.
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
	/// AllocAcctIDSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "661")]
	pub alloc_acct_id_source: Option<AllocAcctIDSource>,
	/// Used when performing "executed price" vs. "average price" allocations (e.g. Japan). <a href="tag_79_AllocAccount.html" target="bottom">AllocAccount&nbsp;(79)</a> plus <a href="tag_366_AllocPrice.html" target="bottom">AllocPrice&nbsp;(366)</a> form a unique Allocs entry. Used in lieu of AllocAvgPx.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "366")]
	pub alloc_price: Option<f64>,
	/// AllocPositionEffect
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1047")]
	pub alloc_position_effect: Option<AllocPositionEffect>,
	/// IndividualAllocID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "467")]
	pub individual_alloc_id: Option<String>,
	/// Required if <a href="tag_78_NoAllocs.html" target="bottom">NoAllocs&nbsp;(78)</a> &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "776")]
	pub individual_alloc_rej_code: Option<IndividualAllocRejCode>,
	/// Free format text field related to this <a href="tag_79_AllocAccount.html" target="bottom">AllocAccount&nbsp;(79)</a> (can be used here to hold text relating to the rejection of this AllocAccount)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "161")]
	pub alloc_text: Option<String>,
	/// Must be set if <a href="tag_361_EncodedAllocText.html" target="bottom">EncodedAllocText&nbsp;(361)</a> field is specified and must immediately precede it.
	#[serde(rename = "360")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_161_AllocText.html" target="bottom">AllocText&nbsp;(161)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "361")]
	pub encoded_alloc_text: Option<fix_common::EncodedText<361>>,
	/// Will allow the intermediary to specify an allocation ID generated by the system
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "989")]
	pub secondary_individual_alloc_id: Option<String>,
	/// Will allow for granular reporting of separate allocation detail within a single trade report or allocation message.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "993")]
	pub alloc_customer_capacity: Option<String>,
	/// Identifies whether the allocation is to be sub-allocated or allocated to a third party.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "992")]
	pub individual_alloc_type: Option<IndividualAllocType>,
	/// Quantity to be allocated to specific sub-account
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "80")]
	pub alloc_qty: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AvgPxIndicator {
	/// No Average Pricing
	#[serde(rename = "0")]
	NoAveragePricing,
	/// Trade is part of an average price group identified by the <a href="tag_820_TradeLinkID.html" target="bottom">TradeLinkID&nbsp;(820)</a>
	#[serde(rename = "1")]
	TradeIsPartOfAnAveragePriceGroupIdentifiedByTheAHrefTag820TradeLinkIdHtmlTargetBottomTradeLinkIdNbspA,
	/// Last trade is the average price group identified by the <a href="tag_820_TradeLinkID.html" target="bottom">TradeLinkID&nbsp;(820)</a>
	#[serde(rename = "2")]
	LastTradeIsTheAveragePriceGroupIdentifiedByTheAHrefTag820TradeLinkIdHtmlTargetBottomTradeLinkIdNbspA,
}

impl Default for AvgPxIndicator {
	fn default() -> Self {
		AvgPxIndicator::NoAveragePricing
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AllocTransType {
	/// New
	#[serde(rename = "0")]
	New,
	/// Replace
	#[serde(rename = "1")]
	Replace,
	/// Cancel
	#[serde(rename = "2")]
	Cancel,
	/// Preliminary (without MiscFees and NetMoney) (Removed/Replaced)
	#[serde(rename = "3")]
	Preliminary,
	/// Calculated (includes MiscFees and NetMoney) (Removed/Replaced)
	#[serde(rename = "4")]
	Calculated,
	/// Calculated without Preliminary (sent unsolicited by broker, includes MiscFees and NetMoney) (Removed/Replaced)
	#[serde(rename = "5")]
	CalculatedWithoutPreliminary,
	/// Reversal
	#[serde(rename = "6")]
	Reversal,
}

impl Default for AllocTransType {
	fn default() -> Self {
		AllocTransType::New
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
	/// allocation pending
	#[serde(rename = "6")]
	AllocationPending,
	/// reversed
	#[serde(rename = "7")]
	Reversed,
}

impl Default for AllocStatus {
	fn default() -> Self {
		AllocStatus::Accepted
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AllocRejCode {
	/// Unknown account(s)
	#[serde(rename = "0")]
	UnknownAccount,
	/// Incorrect quantity
	#[serde(rename = "1")]
	IncorrectQuantity,
	/// Incorrect averageg price
	#[serde(rename = "2")]
	IncorrectAveragegPrice,
	/// Unknown executing broker mnemonic
	#[serde(rename = "3")]
	UnknownExecutingBrokerMnemonic,
	/// Commission difference
	#[serde(rename = "4")]
	CommissionDifference,
	/// Unknown OrderID (37)
	#[serde(rename = "5")]
	UnknownOrderId,
	/// Unknown ListID (66)
	#[serde(rename = "6")]
	UnknownListId,
	/// Other (further in Text (58))
	#[serde(rename = "7")]
	Other,
	/// Incorrect allocated quantity
	#[serde(rename = "8")]
	IncorrectAllocatedQuantity,
	/// Calculation difference
	#[serde(rename = "9")]
	CalculationDifference,
	/// Unknown or stale ExecID
	#[serde(rename = "10")]
	UnknownOrStaleExecId,
	/// Mismatched data
	#[serde(rename = "11")]
	MismatchedData,
	/// Unknown ClOrdID
	#[serde(rename = "12")]
	UnknownClOrdId,
	/// Warehouse request rejected
	#[serde(rename = "13")]
	WarehouseRequestRejected,
}

impl Default for AllocRejCode {
	fn default() -> Self {
		AllocRejCode::UnknownAccount
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AllocReportType {
	/// Preliminary Request to Intermediary
	#[serde(rename = "2")]
	PreliminaryRequestToIntermediary,
	/// Sellside Calculated Using Preliminary (includes MiscFees and NetMoney)
	#[serde(rename = "3")]
	SellsideCalculatedUsingPreliminary,
	/// Sellside Calculated Without Preliminary (sent unsolicited by sellside, includes MiscFees and NetMoney)
	#[serde(rename = "4")]
	SellsideCalculatedWithoutPreliminary,
	/// Warehouse Recap
	#[serde(rename = "5")]
	WarehouseRecap,
	/// Request to Intermediary
	#[serde(rename = "8")]
	RequestToIntermediary,
	/// Accept
	#[serde(rename = "9")]
	Accept,
	/// Reject
	#[serde(rename = "10")]
	Reject,
	/// Accept Pending
	#[serde(rename = "11")]
	AcceptPending,
	/// Complete
	#[serde(rename = "12")]
	Complete,
	/// Reverse Pending
	#[serde(rename = "14")]
	ReversePending,
}

impl Default for AllocReportType {
	fn default() -> Self {
		AllocReportType::PreliminaryRequestToIntermediary
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MatchStatus {
	/// Compared, matched or affirmed
	#[serde(rename = "0")]
	ComparedMatchedOrAffirmed,
	/// Uncompared, unmatched, or unaffirmed
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SecurityType {
	/// Future
	#[serde(rename = "FUT")]
	Fut,
	/// Option
	#[serde(rename = "OPT")]
	Opt,
	/// US Treasury Note (Deprecated Value Use TNOTE)
	#[serde(rename = "UST")]
	Ust,
	/// US Treasury Bill (Deprecated Value Use TBILL)
	#[serde(rename = "USTB")]
	Ustb,
	/// Euro Supranational Coupons *
	#[serde(rename = "EUSUPRA")]
	Eusupra,
	/// Federal Agency Coupon
	#[serde(rename = "FAC")]
	Fac,
	/// Federal Agency Discount Note
	#[serde(rename = "FADN")]
	Fadn,
	/// Private Export Funding *
	#[serde(rename = "PEF")]
	Pef,
	/// USD Supranational Coupons *
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
	/// Brady Bond
	#[serde(rename = "BRADY")]
	Brady,
	/// Euro Sovereigns *
	#[serde(rename = "EUSOV")]
	Eusov,
	/// US Treasury Bond
	#[serde(rename = "TBOND")]
	Tbond,
	/// Interest Strip From Any Bond Or Note
	#[serde(rename = "TINT")]
	Tint,
	/// Treasury Inflation Protected Securities
	#[serde(rename = "TIPS")]
	Tips,
	/// Principal Strip Of A Callable Bond Or Note
	#[serde(rename = "TCAL")]
	Tcal,
	/// Principal Strip From A Non-Callable Bond Or Note
	#[serde(rename = "TPRN")]
	Tprn,
	/// US Treasury Note
	#[serde(rename = "TNOTE")]
	Tnote,
	/// US Treasury Bill
	#[serde(rename = "TBILL")]
	Tbill,
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
	/// Letter Of Credit
	#[serde(rename = "LOFC")]
	Lofc,
	/// Swing Line Facility
	#[serde(rename = "SWING")]
	Swing,
	/// Debtor In Possession
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
	/// Bill Of Exchanges
	#[serde(rename = "BOX")]
	Box,
	/// Certificate Of Deposit
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
	/// Euro Certificate Of Deposit
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
	/// Yankee Certificate Of Deposit
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
	/// Pfandbriefe *
	#[serde(rename = "PFAND")]
	Pfand,
	/// To Be Announced
	#[serde(rename = "TBA")]
	Tba,
	/// Other Anticipation Notes (BAN, GAN, etc.)
	#[serde(rename = "AN")]
	An,
	/// Certificate Of Obligation
	#[serde(rename = "COFO")]
	Cofo,
	/// Certificate Of Participation
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
	/// Tax Revenue Anticipation Note
	#[serde(rename = "TRAN")]
	Tran,
	/// Variable Rate Demand Note
	#[serde(rename = "VRDN")]
	Vrdn,
	/// Warrant
	#[serde(rename = "WAR")]
	War,
	/// Mutual Fund
	#[serde(rename = "MF")]
	Mf,
	/// Multileg Instrument
	#[serde(rename = "MLEG")]
	Mleg,
	/// No Security Type
	#[serde(rename = "NONE")]
	None,
	/// Options on Futures
	#[serde(rename = "OOF")]
	Oof,
	/// Options on Physical
	#[serde(rename = "OOP")]
	Oop,
	/// Wildcard Entry (was "?" in 4.4, used on Security Definition Request message)
	#[serde(rename = "WLD")]
	Wld,
	/// Cash
	#[serde(rename = "CASH")]
	Cash,
	/// Interest Rate Swap
	#[serde(rename = "IRS")]
	Irs,
	/// Bank Depository Note
	#[serde(rename = "BDN")]
	Bdn,
	/// Canadian Money Markets
	#[serde(rename = "CAMM")]
	Camm,
	/// Canadian Treasury Notes
	#[serde(rename = "CAN")]
	Can,
	/// Canadian Treasury Bills
	#[serde(rename = "CTB")]
	Ctb,
	/// Credit Default Swap
	#[serde(rename = "CDS")]
	Cds,
	/// Canadian Mortgage Bonds
	#[serde(rename = "CMB")]
	Cmb,
	/// Euro Corporate Floating Rate Notes
	#[serde(rename = "EUFRN")]
	Eufrn,
	/// US Corporate Floating Rate Notes
	#[serde(rename = "FRN")]
	Frn,
	/// Canadian Provincial Bonds
	#[serde(rename = "PROV")]
	Prov,
	/// Secured Liquidity Note
	#[serde(rename = "SLQN")]
	Slqn,
	/// Treasury Bill - non US
	#[serde(rename = "TB")]
	Tb,
	/// Term Liquidity Note
	#[serde(rename = "TLQN")]
	Tlqn,
	/// Taxable Municipal CP
	#[serde(rename = "TMCP")]
	Tmcp,
}

impl Default for SecurityType {
	fn default() -> Self {
		SecurityType::Fut
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AllocPositionEffect {
	/// Open
	#[serde(rename = "O")]
	Open,
	/// Close
	#[serde(rename = "C")]
	Close,
	/// Rolled
	#[serde(rename = "R")]
	Rolled,
	/// FIFO
	#[serde(rename = "F")]
	Fifo,
}

impl Default for AllocPositionEffect {
	fn default() -> Self {
		AllocPositionEffect::Open
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum IndividualAllocRejCode {
	/// Unknown account(s)
	#[serde(rename = "0")]
	UnknownAccount,
	/// Incorrect quantity
	#[serde(rename = "1")]
	IncorrectQuantity,
	/// Incorrect averageg price
	#[serde(rename = "2")]
	IncorrectAveragegPrice,
	/// Unknown executing broker mnemonic
	#[serde(rename = "3")]
	UnknownExecutingBrokerMnemonic,
	/// Commission difference
	#[serde(rename = "4")]
	CommissionDifference,
	/// Unknown OrderID (37)
	#[serde(rename = "5")]
	UnknownOrderId,
	/// Unknown ListID (66)
	#[serde(rename = "6")]
	UnknownListId,
	/// Other (further in Text (58))
	#[serde(rename = "7")]
	Other,
	/// Incorrect allocated quantity
	#[serde(rename = "8")]
	IncorrectAllocatedQuantity,
	/// Calculation difference
	#[serde(rename = "9")]
	CalculationDifference,
	/// Unknown or stale ExecID
	#[serde(rename = "10")]
	UnknownOrStaleExecId,
	/// Mismatched data
	#[serde(rename = "11")]
	MismatchedData,
	/// Unknown ClOrdID
	#[serde(rename = "12")]
	UnknownClOrdId,
	/// Warehouse request rejected
	#[serde(rename = "13")]
	WarehouseRequestRejected,
}

impl Default for IndividualAllocRejCode {
	fn default() -> Self {
		IndividualAllocRejCode::UnknownAccount
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum IndividualAllocType {
	/// Sub Allocate
	#[serde(rename = "1")]
	SubAllocate,
	/// Third Party Allocation
	#[serde(rename = "2")]
	ThirdPartyAllocation,
}

impl Default for IndividualAllocType {
	fn default() -> Self {
		IndividualAllocType::SubAllocate
	}
}
