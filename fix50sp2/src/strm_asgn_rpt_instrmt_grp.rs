
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StrmAsgnRptInstrmtGrp {
	/// NoRelatedSym
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "146")]
	pub related_sym: Option<fix_common::RepeatingValues<RelatedSy>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RelatedSy {
	/// SettlType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "63")]
	pub settl_type: Option<SettlType>,
	/// StreamAsgnType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1617")]
	pub stream_asgn_type: Option<StreamAsgnType>,
	/// MDStreamID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1500")]
	pub md_stream_id: Option<String>,
	/// StreamAsgnRejReason
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1502")]
	pub stream_asgn_rej_reason: Option<StreamAsgnRejReason>,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// EncodedTextLen
	#[serde(rename = "354")]
	/// EncodedText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SettlType {
	/// Regular / FX Spot settlement (T+1 or T+2 depending on currency)
	#[serde(rename = "0")]
	RegularFxSpotSettlement,
	/// Cash (TOD / T+0)
	#[serde(rename = "1")]
	Cash,
	/// Next Day (TOM / T+1)
	#[serde(rename = "2")]
	NextDay,
	/// T+2
	#[serde(rename = "3")]
	T2,
	/// T+3
	#[serde(rename = "4")]
	T3,
	/// T+4
	#[serde(rename = "5")]
	T4,
	/// Future
	#[serde(rename = "6")]
	Future,
	/// When And If Issued
	#[serde(rename = "7")]
	WhenAndIfIssued,
	/// Sellers Option
	#[serde(rename = "8")]
	SellersOption,
	/// T+5
	#[serde(rename = "9")]
	T5,
	/// Broken date - for FX expressing non-standard tenor, <a href="tag_64_SettlDate.html" target="bottom">SettlDate&nbsp;(64)</a> must be specified
	#[serde(rename = "B")]
	BrokenDateForFxExpressingNonStandardTenorAHrefTag64SettlDateHtmlTargetBottomSettlDateNbspAMustBeSpecified,
	/// FX Spot Next settlement (Spot+1, aka next day)
	#[serde(rename = "C")]
	FxSpotNextSettlement,
}

impl Default for SettlType {
	fn default() -> Self {
		SettlType::RegularFxSpotSettlement
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StreamAsgnType {
	/// Assignment
	#[serde(rename = "1")]
	Assignment,
	/// Rejected
	#[serde(rename = "2")]
	Rejected,
	/// Terminate/Unassign
	#[serde(rename = "3")]
	TerminateUnassign,
}

impl Default for StreamAsgnType {
	fn default() -> Self {
		StreamAsgnType::Assignment
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StreamAsgnRejReason {
	/// Unknown client
	#[serde(rename = "0")]
	UnknownClient,
	/// Exceeds maximum size
	#[serde(rename = "1")]
	ExceedsMaximumSize,
	/// Unknown or Invalid currency pair
	#[serde(rename = "2")]
	UnknownOrInvalidCurrencyPair,
	/// No available stream
	#[serde(rename = "3")]
	NoAvailableStream,
	/// Other
	#[serde(rename = "99")]
	Other,
}

impl Default for StreamAsgnRejReason {
	fn default() -> Self {
		StreamAsgnRejReason::UnknownClient
	}
}
