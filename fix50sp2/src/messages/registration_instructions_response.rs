
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RegistrationInstructionsResponse {
	/// MsgType = p
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'p', ' '>,
	/// Unique identifier of the original <a href="message_Registration_Instructions_o.html" target="main">Registration Instructions&nbsp;(o)</a> details
	#[serde(rename = "513")]
	pub regist_id: String,
	/// Identifies original <a href="message_Registration_Instructions_o.html" target="main">Registration Instructions&nbsp;(o)</a> transaction type
	#[serde(rename = "514")]
	pub regist_trans_type: RegistTransType,
	/// Required for Cancel and Replace <a href="tag_514_RegistTransType.html" target="bottom">RegistTransType&nbsp;(514)</a> messages
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "508")]
	pub regist_ref_id: Option<String>,
	/// Unique identifier of the order as assigned by institution or intermediary.
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
	/// RegistStatus
	#[serde(rename = "506")]
	pub regist_status: RegistStatus,
	/// RegistRejReasonCode
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "507")]
	pub regist_rej_reason_code: Option<RegistRejReasonCode>,
	/// RegistRejReasonText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "496")]
	pub regist_rej_reason_text: Option<String>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum RegistStatus {
	/// Accepted
	#[serde(rename = "A")]
	Accepted,
	/// Rejected
	#[serde(rename = "R")]
	Rejected,
	/// Held
	#[serde(rename = "H")]
	Held,
	/// Reminder - i.e. Registration Instructions are still outstanding
	#[serde(rename = "N")]
	ReminderIERegistrationInstructionsAreStillOutstanding,
}

impl Default for RegistStatus {
	fn default() -> Self {
		RegistStatus::Accepted
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum RegistRejReasonCode {
	/// Invalid/unacceptable Account Type
	#[serde(rename = "1")]
	InvalidUnacceptableAccountType,
	/// Invalid/unacceptable Tax Exempt Type
	#[serde(rename = "2")]
	InvalidUnacceptableTaxExemptType,
	/// Invalid/unacceptable Ownership Type
	#[serde(rename = "3")]
	InvalidUnacceptableOwnershipType,
	/// Invalid/unacceptable No Reg Detls
	#[serde(rename = "4")]
	InvalidUnacceptableNoRegDetls,
	/// Invalid/unacceptable Reg Seq No
	#[serde(rename = "5")]
	InvalidUnacceptableRegSeqNo,
	/// Invalid/unacceptable Reg Dtls
	#[serde(rename = "6")]
	InvalidUnacceptableRegDtls,
	/// Invalid/unacceptable Mailing Dtls
	#[serde(rename = "7")]
	InvalidUnacceptableMailingDtls,
	/// Invalid/unacceptable Mailing Inst
	#[serde(rename = "8")]
	InvalidUnacceptableMailingInst,
	/// Invalid/unacceptable Investor ID
	#[serde(rename = "9")]
	InvalidUnacceptableInvestorId,
	/// Invalid/unacceptable Investor ID Source
	#[serde(rename = "10")]
	InvalidUnacceptableInvestorIdSource,
	/// Invalid/unacceptable Date of Birth
	#[serde(rename = "11")]
	InvalidUnacceptableDateOfBirth,
	/// Invalid/unacceptable Investor Country Of Residence
	#[serde(rename = "12")]
	InvalidUnacceptableInvestorCountryOfResidence,
	/// Invalid/unacceptable NoDistribInstns
	#[serde(rename = "13")]
	InvalidUnacceptableNoDistribInstns,
	/// Invalid/unacceptable Distrib Percentage
	#[serde(rename = "14")]
	InvalidUnacceptableDistribPercentage,
	/// Invalid/unacceptable Distrib Payment Method
	#[serde(rename = "15")]
	InvalidUnacceptableDistribPaymentMethod,
	/// Invalid/unacceptable Cash Distrib Agent Acct Name
	#[serde(rename = "16")]
	InvalidUnacceptableCashDistribAgentAcctName,
	/// Invalid/unacceptable Cash Distrib Agent Code
	#[serde(rename = "17")]
	InvalidUnacceptableCashDistribAgentCode,
	/// Invalid/unacceptable Cash Distrib Agent Acct Num
	#[serde(rename = "18")]
	InvalidUnacceptableCashDistribAgentAcctNum,
	/// Other
	#[serde(rename = "99")]
	Other,
}

impl Default for RegistRejReasonCode {
	fn default() -> Self {
		RegistRejReasonCode::InvalidUnacceptableAccountType
	}
}
