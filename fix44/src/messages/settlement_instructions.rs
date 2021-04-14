
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SettlementInstructions {
	/// MsgType = T
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// Unique identifier for this message
	#[serde(rename = "777")]
	pub settl_inst_msg_id: String,
	/// Only used when this message is used to respond to a <a href="message_Settlement_Instruction_Request_AV.html" target="main">settlement instruction request&nbsp;(AV)</a> (to which this ID refers)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "791")]
	pub settl_inst_req_id: Option<String>,
	/// 1=Standing Instructions, 4=Specific Order, 5=Reject SSI request
	#[serde(rename = "160")]
	pub settl_inst_mode: SettlInstMode,
	/// Required for <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> = 5. Used to provide reason for rejecting a <a href="message_Settlement_Instruction_Request_AV.html" target="main">Settlement Instruction Request&nbsp;(AV)</a> message.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "792")]
	pub settl_inst_req_rej_code: Option<SettlInstReqRejCode>,
	/// Can be used to provide any additional rejection text where rejecting a <a href="message_Settlement_Instruction_Request_AV.html" target="main">Settlement Instruction Request&nbsp;(AV)</a> message.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// EncodedTextLen
	#[serde(rename = "354")]
	/// EncodedText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
	/// Required for <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> =4.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "11")]
	pub cl_ord_id: Option<String>,
	/// Date/time this message was generated
	#[serde(rename = "60")]
	pub transact_time: fix_common::UTCTimestamp,
	/// Required except where <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> is 5=Reject SSI request
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "778")]
	pub settl_inst: Option<fix_common::RepeatingValues<SettlIns>>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SettlIns {
	/// Unique ID for this settlement instruction. Required except where <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> is 5=Reject SSI request
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "162")]
	pub settl_inst_id: Option<String>,
	/// New, Replace, Cancel or Restate. Required except where <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> is 5=Reject SSI request
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "163")]
	pub settl_inst_trans_type: Option<SettlInstTransType>,
	/// Required where <a href="tag_163_SettlInstTransType.html" target="bottom">SettlInstTransType&nbsp;(163)</a> is Cancel or Replace
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "214")]
	pub settl_inst_ref_id: Option<String>,
	/// Can be used for <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> 1 if SSIs are being provided for a particular side.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "54")]
	pub side: Option<Side>,
	/// Can be used for <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> 1 if SSIs are being provided for a particular product.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "460")]
	pub product: Option<Product>,
	/// Can be used for <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> 1 if SSIs are being provided for a particular security type (as alternative to <a href="tag_461_CFICode.html" target="bottom">CFICode&nbsp;(461)</a> ).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "167")]
	pub security_type: Option<SecurityType>,
	/// Can be used for <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> 1 if SSIs are being provided for a particular security type (as identified by CFI code).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "461")]
	pub cfi_code: Option<String>,
	/// Effective (start) date/time for this settlement instruction. Required except where <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> is 5=Reject SSI request
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "168")]
	pub effective_time: Option<fix_common::UTCTimestamp>,
	/// Termination date/time for this settlement instruction.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "126")]
	pub expire_time: Option<fix_common::UTCTimestamp>,
	/// Date/time this settlement instruction was last updated (or created if not updated since creation). Required except where <a href="tag_160_SettlInstMode.html" target="bottom">SettlInstMode&nbsp;(160)</a> is 5=Reject SSI request
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "779")]
	pub last_update_time: Option<fix_common::UTCTimestamp>,
	/// For use with CIV settlement instructions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "492")]
	pub payment_method: Option<PaymentMethod>,
	/// For use with CIV settlement instructions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "476")]
	pub payment_ref: Option<String>,
	/// For use with CIV settlement instructions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "488")]
	pub card_holder_name: Option<String>,
	/// For use with CIV settlement instructions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "489")]
	pub card_number: Option<String>,
	/// For use with CIV settlement instructions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "503")]
	pub card_start_date: Option<fix_common::LocalMktDate>,
	/// For use with CIV settlement instructions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "490")]
	pub card_exp_date: Option<fix_common::LocalMktDate>,
	/// For use with CIV settlement instructions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "491")]
	pub card_iss_num: Option<String>,
	/// For use with CIV settlement instructions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "504")]
	pub payment_date: Option<fix_common::LocalMktDate>,
	/// For use with CIV settlement instructions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "505")]
	pub payment_remitter_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SettlInstMode {
	/// Standing Instructions Provided
	#[serde(rename = "1")]
	StandingInstructionsProvided,
	/// Specific Order for a single account (for CIV)
	#[serde(rename = "4")]
	SpecificOrderForASingleAccount,
	/// Request reject
	#[serde(rename = "5")]
	RequestReject,
}

impl Default for SettlInstMode {
	fn default() -> Self {
		SettlInstMode::StandingInstructionsProvided
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SettlInstReqRejCode {
	/// unable to process request (e.g. database unavailable)
	#[serde(rename = "0")]
	UnableToProcessRequest,
	/// unknown account
	#[serde(rename = "1")]
	UnknownAccount,
	/// no matching settlement instructions found
	#[serde(rename = "2")]
	NoMatchingSettlementInstructionsFound,
	/// other
	#[serde(rename = "99")]
	Other,
}

impl Default for SettlInstReqRejCode {
	fn default() -> Self {
		SettlInstReqRejCode::UnableToProcessRequest
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
	/// Restate (used where the Settlement Instruction is being used to communicate standing instructions which have not been changed
	/// or added to)
	#[serde(rename = "T")]
	Restate,
}

impl Default for SettlInstTransType {
	fn default() -> Self {
		SettlInstTransType::New
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
