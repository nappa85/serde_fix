
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecurityTypeRequest {
	/// MsgType = v
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'v', ' '>,
	/// SecurityReqID
	#[serde(rename = "320")]
	pub security_req_id: String,
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
	/// Used to qualify which security types are returned
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "460")]
	pub product: Option<Product>,
	/// Used to qualify which security type is returned
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "167")]
	pub security_type: Option<SecurityType>,
	/// Used to qualify which security types are returned
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "762")]
	pub security_sub_type: Option<String>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
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
