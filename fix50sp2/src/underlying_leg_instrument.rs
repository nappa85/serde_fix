
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingLegInstrument {
	/// UnderlyingLegSymbol
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1330")]
	pub underlying_leg_symbol: Option<String>,
	/// UnderlyingLegSymbolSfx
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1331")]
	pub underlying_leg_symbol_sfx: Option<String>,
	/// UnderlyingLegSecurityID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1332")]
	pub underlying_leg_security_id: Option<String>,
	/// UnderlyingLegSecurityIDSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1333")]
	pub underlying_leg_security_id_source: Option<UnderlyingLegSecurityIDSource>,
	/// UnderlyingLegSecurityAltIDGrp
	#[serde(flatten)]
	pub underlying_leg_security_alt_id_grp: Option<super::underlying_leg_security_alt_id_grp::UnderlyingLegSecurityAltIDGrp>,
	/// UnderlyingLegCFICode
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1344")]
	pub underlying_leg_cfi_code: Option<String>,
	/// UnderlyingLegSecurityType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1337")]
	pub underlying_leg_security_type: Option<UnderlyingLegSecurityType>,
	/// UnderlyingLegSecuritySubType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1338")]
	pub underlying_leg_security_sub_type: Option<String>,
	/// UnderlyingLegMaturityMonthYear
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1339")]
	pub underlying_leg_maturity_month_year: Option<fix_common::MonthYear>,
	/// UnderlyingLegMaturityDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1345")]
	pub underlying_leg_maturity_date: Option<fix_common::LocalMktDate>,
	/// UnderlyingLegMaturityTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1405")]
	pub underlying_leg_maturity_time: Option<fix_common::TZTimeOnly>,
	/// UnderlyingLegStrikePrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1340")]
	pub underlying_leg_strike_price: Option<f64>,
	/// UnderlyingLegOptAttribute
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1391")]
	pub underlying_leg_opt_attribute: Option<char>,
	/// UnderlyingLegPutOrCall
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1343")]
	pub underlying_leg_put_or_call: Option<UnderlyingLegPutOrCall>,
	/// UnderlyingLegSecurityExchange
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1341")]
	pub underlying_leg_security_exchange: Option<String>,
	/// UnderlyingLegSecurityDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1392")]
	pub underlying_leg_security_desc: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingLegSecurityIDSource {
	/// CUSIP
	#[serde(rename = "1")]
	Cusip,
	/// SEDOL
	#[serde(rename = "2")]
	Sedol,
	/// QUIK
	#[serde(rename = "3")]
	Quik,
	/// ISIN
	#[serde(rename = "4")]
	Isin,
	/// RIC
	#[serde(rename = "5")]
	Ric,
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
	/// Clearing House / Clearing Organization
	#[serde(rename = "H")]
	ClearingHouseClearingOrganization,
	/// ISDA/FpML Product Specification
	#[serde(rename = "I")]
	IsdaFpMlProductSpecification,
	/// Option Price Reporting Authority
	#[serde(rename = "J")]
	OptionPriceReportingAuthority,
	/// ISDA/FpML Product URL (URL in SecurityID)
	#[serde(rename = "K")]
	IsdaFpMlProductUrl,
	/// Letter of Credit
	#[serde(rename = "L")]
	LetterOfCredit,
	/// Marketplace-assigned Identifier
	#[serde(rename = "M")]
	MarketplaceAssignedIdentifier,
	/// Markit RED entity CLIP
	#[serde(rename = "N")]
	MarkitRedEntityClip,
	/// Markit RED pair CLIP
	#[serde(rename = "P")]
	MarkitRedPairClip,
	/// CFTC commodity code
	#[serde(rename = "Q")]
	CftcCommodityCode,
	/// ISDA Commodity Reference Price
	#[serde(rename = "R")]
	IsdaCommodityReferencePrice,
	/// Financial Instrument Global Identifier
	#[serde(rename = "S")]
	FinancialInstrumentGlobalIdentifier,
	/// Legal Entity Identifier
	#[serde(rename = "T")]
	LegalEntityIdentifier,
	/// Synthetic
	#[serde(rename = "U")]
	Synthetic,
	/// Fidessa Instrument Mnemonic (FIM)
	#[serde(rename = "V")]
	FidessaInstrumentMnemonic,
	/// Index name
	#[serde(rename = "W")]
	IndexName,
	/// Uniform Symbol (UMTF Symbol)
	#[serde(rename = "X")]
	UniformSymbol,
}

impl Default for UnderlyingLegSecurityIDSource {
	fn default() -> Self {
		UnderlyingLegSecurityIDSource::Cusip
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingLegSecurityType {
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
	/// Wildcard entry for use on Security Definition Request
	#[serde(rename = "?")]
	WildcardEntryForUseOnSecurityDefinitionRequest,
	/// Options on Combo
	#[serde(rename = "OOC")]
	Ooc,
	/// Non-deliverable forward
	#[serde(rename = "FXNDF")]
	Fxndf,
	/// FX Spot
	#[serde(rename = "FXSPOT")]
	Fxspot,
	/// FX Forward
	#[serde(rename = "FXFWD")]
	Fxfwd,
	/// FX Swap
	#[serde(rename = "FXSWAP")]
	Fxswap,
	/// Deliver versus pledge
	#[serde(rename = "DVPLDG")]
	Dvpldg,
	/// Commodity swap
	#[serde(rename = "CMDTYSWAP")]
	Cmdtyswap,
	/// Futures option swap
	#[serde(rename = "SWAPTION")]
	Swaption,
	/// Derivative Forward
	#[serde(rename = "FWD")]
	Fwd,
	/// Total return swap
	#[serde(rename = "TRS")]
	Trs,
	/// Cap (In an interest rate cap, the buyer receives payments at the end of each period in which the rate indec exceeds the agreed
	/// strike rate)
	#[serde(rename = "CAP")]
	Cap,
	/// Collar (In an interest rate collar, this is a combination of a cap and a floor)
	#[serde(rename = "CLLR")]
	Cllr,
	/// Exotic
	#[serde(rename = "EXOTIC")]
	Exotic,
	/// Floor (In an interest rate floor, the buyer receives payments at the end of each period in which the rate index is below the
	/// agreed strike rate)
	#[serde(rename = "FLR")]
	Flr,
	/// Forward Rate Agreement
	#[serde(rename = "FRA")]
	Fra,
	/// Loan/lease
	#[serde(rename = "LOANLEASE")]
	Loanlease,
	/// Spot forward
	#[serde(rename = "SPOTFWD")]
	Spotfwd,
	/// Transmission
	#[serde(rename = "XMISSION")]
	Xmission,
	/// General type for a contract based on an established index
	#[serde(rename = "INDEX")]
	Index,
	/// Collateral basket
	#[serde(rename = "COLLBSKT")]
	Collbskt,
	/// Bond basket
	#[serde(rename = "BDBSKT")]
	Bdbskt,
	/// Contract for difference
	#[serde(rename = "CFD")]
	Cfd,
	/// Correlation swap
	#[serde(rename = "CRLTNSWAP")]
	Crltnswap,
	/// Dividend swap
	#[serde(rename = "DVDNDSWAP")]
	Dvdndswap,
	/// Equity basket
	#[serde(rename = "EQBSKT")]
	Eqbskt,
	/// Equity forward
	#[serde(rename = "EQFWD")]
	Eqfwd,
	/// Return swap
	#[serde(rename = "RTRNSWAP")]
	Rtrnswap,
	/// Variance swap
	#[serde(rename = "VARSWAP")]
	Varswap,
	/// Non-deliverable Swap
	#[serde(rename = "FXNDS")]
	Fxnds,
	/// Portfolio Swaps
	#[serde(rename = "PRTFLIOSWAP")]
	Prtflioswap,
	/// Futures on a Swap
	#[serde(rename = "FUTSWAP")]
	Futswap,
	/// Forwards on a Swap
	#[serde(rename = "FWDSWAP ")]
	Fwdswap,
	/// Forward Freight Agreement
	#[serde(rename = "FWDFRTAGMT ")]
	Fwdfrtagmt,
	/// Spread Betting
	#[serde(rename = "SPREADBET")]
	Spreadbet,
	/// Other
	#[serde(rename = "Other")]
	Other,
	/// Depository Receipts
	#[serde(rename = "DR")]
	Dr,
	/// Exchange traded commodity
	#[serde(rename = "ETC")]
	Etc,
	/// Exchange traded note
	#[serde(rename = "ETN")]
	Etn,
	/// Securitized derivative
	#[serde(rename = "SECDERIV")]
	Secderiv,
	/// Structured finance product
	#[serde(rename = "SFP")]
	Sfp,
}

impl Default for UnderlyingLegSecurityType {
	fn default() -> Self {
		UnderlyingLegSecurityType::Fut
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingLegPutOrCall {
	/// Put
	#[serde(rename = "0")]
	Put,
	/// Call
	#[serde(rename = "1")]
	Call,
}

impl Default for UnderlyingLegPutOrCall {
	fn default() -> Self {
		UnderlyingLegPutOrCall::Put
	}
}
