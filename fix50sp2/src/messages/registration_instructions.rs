
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RegistrationInstructions {
	/// MsgType = o
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// RegistID
	#[serde(rename = "513")]
	pub regist_id: String,
	/// RegistTransType
	#[serde(rename = "514")]
	pub regist_trans_type: RegistTransType,
	/// Required for Cancel and Replace <a href="tag_514_RegistTransType.html" target="bottom">RegistTransType&nbsp;(514)</a> messages
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "508")]
	pub regist_ref_id: Option<String>,
	/// Unique identifier of the order as assigned by institution or intermediary to which Registration relates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "11")]
	pub cl_ord_id: Option<String>,
	/// Insert here the set of "Parties" (firm identification) fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// Account
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1")]
	pub account: Option<String>,
	/// AcctIDSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "660")]
	pub acct_id_source: Option<AcctIDSource>,
	/// RegistAcctType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "493")]
	pub regist_acct_type: Option<String>,
	/// TaxAdvantageType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "495")]
	pub tax_advantage_type: Option<TaxAdvantageType>,
	/// OwnershipType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "517")]
	pub ownership_type: Option<OwnershipType>,
	/// Number of registration details in this message (number of repeating groups to follow)
	#[serde(flatten)]
	pub rgst_dtls_grp: Option<super::super::rgst_dtls_grp::RgstDtlsGrp>,
	/// Number of Distribution instructions in this message (number of repeating groups to follow)
	#[serde(flatten)]
	pub rgst_dist_inst_grp: Option<super::super::rgst_dist_inst_grp::RgstDistInstGrp>,
	/// ClearingBusinessDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "715")]
	pub clearing_business_date: Option<fix_common::LocalMktDate>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RegistTransType {
	/// New
	#[serde(rename = "0")]
	New,
	/// Cancel
	#[serde(rename = "2")]
	Cancel,
	/// Replace
	#[serde(rename = "1")]
	Replace,
}

impl Default for RegistTransType {
	fn default() -> Self {
		RegistTransType::New
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AcctIDSource {
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
	/// Special Segregated Account ID
	#[serde(rename = "6")]
	SpecialSegregatedAccountId,
	/// Other (custom or proprietary)
	#[serde(rename = "99")]
	Other,
}

impl Default for AcctIDSource {
	fn default() -> Self {
		AcctIDSource::Bic
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TaxAdvantageType {
	/// None/Not Applicable (default)
	#[serde(rename = "0")]
	N0,
	/// Maxi ISA (UK)
	#[serde(rename = "1")]
	N1,
	/// TESSA (UK)
	#[serde(rename = "2")]
	N2,
	/// Mini Cash ISA (UK)
	#[serde(rename = "3")]
	N3,
	/// Mini Stocks And Shares ISA (UK)
	#[serde(rename = "4")]
	N4,
	/// Mini Insurance ISA (UK)
	#[serde(rename = "5")]
	N5,
	/// Current Year Payment (US)
	#[serde(rename = "6")]
	N6,
	/// Prior Year Payment (US)
	#[serde(rename = "7")]
	N7,
	/// Asset Transfer (US)
	#[serde(rename = "8")]
	N8,
	/// Employee - prior year (US)
	#[serde(rename = "9")]
	N9,
	/// Employee - current year (US)
	#[serde(rename = "10")]
	N10,
	/// Employer - prior year (US)
	#[serde(rename = "11")]
	N11,
	/// Employer - current year (US)
	#[serde(rename = "12")]
	N12,
	/// Non-fund prototype IRA (US)
	#[serde(rename = "13")]
	N13,
	/// Non-fund qualified plan (US)
	#[serde(rename = "14")]
	N14,
	/// Defined contribution plan (US)
	#[serde(rename = "15")]
	N15,
	/// Individual Retirement Account (US)
	#[serde(rename = "16")]
	N16,
	/// Individual Retirement Account - Rollover (US)
	#[serde(rename = "17")]
	N17,
	/// KEOGH (US)
	#[serde(rename = "18")]
	N18,
	/// Profit Sharing Plan (US)
	#[serde(rename = "19")]
	N19,
	/// 401(k) (US)
	#[serde(rename = "20")]
	N20,
	/// Self-directed IRA (US)
	#[serde(rename = "21")]
	N21,
	/// 403(b) (US)
	#[serde(rename = "22")]
	N22,
	/// 457 (US)
	#[serde(rename = "23")]
	N23,
	/// Roth IRA (Fund Prototype) (US)
	#[serde(rename = "24")]
	N24,
	/// Roth IRA (Non-prototype) (US)
	#[serde(rename = "25")]
	N25,
	/// Roth Conversion IRA (Fund Prototype) (US)
	#[serde(rename = "26")]
	N26,
	/// Roth Conversion IRA (Non-prototype) (US)
	#[serde(rename = "27")]
	N27,
	/// Education IRA (Fund Prototype) (US)
	#[serde(rename = "28")]
	N28,
	/// Education IRA (Non-prototype) (US)
	#[serde(rename = "29")]
	N29,
	/// Other
	#[serde(rename = "999")]
	N999,
}

impl Default for TaxAdvantageType {
	fn default() -> Self {
		TaxAdvantageType::N0
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OwnershipType {
	/// Joint Investors
	#[serde(rename = "J")]
	JointInvestors,
	/// Tenants in Common
	#[serde(rename = "T")]
	TenantsInCommon,
	/// Joint Trustees
	#[serde(rename = "2")]
	JointTrustees,
}

impl Default for OwnershipType {
	fn default() -> Self {
		OwnershipType::JointInvestors
	}
}
