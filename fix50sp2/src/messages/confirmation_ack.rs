
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Confirmation {
	/// MsgType = AU
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// ConfirmID
	#[serde(rename = "664")]
	pub confirm_id: String,
	/// TradeDate
	#[serde(rename = "75")]
	pub trade_date: crate::entities::LocalMktDate,
	/// Date/Time <a href="message_Allocation_Instruction_Ack_P.html" target="main">Allocation Instruction Ack&nbsp;(P)</a> generated
	#[serde(rename = "60")]
	pub transact_time: crate::entities::UTCTimestamp,
	/// AffirmStatus
	#[serde(rename = "940")]
	pub affirm_status: AffirmStatus,
	/// Required for <a href="tag_940_AffirmStatus.html" target="bottom">AffirmStatus&nbsp;(940)</a> = 1 (rejected)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "774")]
	pub confirm_rej_reason: Option<ConfirmRejReason>,
	/// Denotes whether the financial details provided on the <a href="message_Confirmation_AK.html" target="main">Confirmation&nbsp;(AK)</a> were successfully matched.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "573")]
	pub match_status: Option<MatchStatus>,
	/// MatchExceptionGrp
	#[serde(flatten)]
	pub match_exception_grp: Option<super::super::match_exception_grp::MatchExceptionGrp>,
	/// MatchingDataPointGrp
	#[serde(flatten)]
	pub matching_data_point_grp: Option<super::super::matching_data_point_grp::MatchingDataPointGrp>,
	/// Can include explanation for <a href="tag_774_ConfirmRejReason.html" target="bottom">ConfirmRejReason(774)&nbsp;(774)</a> = 99 (Other)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if <a href="tag_355_EncodedText.html" target="bottom">EncodedText&nbsp;(355)</a> field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "354")]
	pub encoded_text_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the <a href="tag_58_Text.html" target="bottom">Text&nbsp;(58)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "355")]
	pub encoded_text: Option<String>,
	/// TradeConfirmationReferenceID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2390")]
	pub trade_confirmation_reference_id: Option<String>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AffirmStatus {
	/// Received
	#[serde(rename = "1")]
	Received,
	/// Confirm rejected, i.e. not affirmed
	#[serde(rename = "2")]
	ConfirmRejectedIENotAffirmed,
	/// Affirmed
	#[serde(rename = "3")]
	Affirmed,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ConfirmRejReason {
	/// Mismatched account
	#[serde(rename = "1")]
	MismatchedAccount,
	/// Missing settlement instructions
	#[serde(rename = "2")]
	MissingSettlementInstructions,
	/// Other
	#[serde(rename = "99")]
	Other,
	/// Unknown or missing IndividualAllocId(467)
	#[serde(rename = "3")]
	UnknownOrMissingIndividualAllocId,
	/// Transaction not recognized
	#[serde(rename = "4")]
	TransactionNotRecognized,
	/// Duplicate transaction
	#[serde(rename = "5")]
	DuplicateTransaction,
	/// Incorrect or missing instrument
	#[serde(rename = "6")]
	IncorrectOrMissingInstrument,
	/// Incorrect or missing price
	#[serde(rename = "7")]
	IncorrectOrMissingPrice,
	/// Incorrect or missing commission
	#[serde(rename = "8")]
	IncorrectOrMissingCommission,
	/// Incorrect or missing settlement date
	#[serde(rename = "9")]
	IncorrectOrMissingSettlementDate,
	/// Incorrect or missing fund ID or fund name
	#[serde(rename = "10")]
	IncorrectOrMissingFundIdOrFundName,
	/// Incorrect or missing quantity
	#[serde(rename = "11")]
	IncorrectOrMissingQuantity,
	/// Incorrect or missing fees
	#[serde(rename = "12")]
	IncorrectOrMissingFees,
	/// Incorrect or missing tax
	#[serde(rename = "13")]
	IncorrectOrMissingTax,
	/// Incorrect or missing party
	#[serde(rename = "14")]
	IncorrectOrMissingParty,
	/// Incorrect or missing side
	#[serde(rename = "15")]
	IncorrectOrMissingSide,
	/// Incorrect or missing net-money
	#[serde(rename = "16")]
	IncorrectOrMissingNetMoney,
	/// Incorrect or missing date
	#[serde(rename = "17")]
	IncorrectOrMissingDate,
	/// Incorrect or missing settlement currency instructions
	#[serde(rename = "18")]
	IncorrectOrMissingSettlementCurrencyInstructions,
	/// Incorrect or missing capacity
	#[serde(rename = "19")]
	IncorrectOrMissingCapacity,
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
	/// Mismatched (Indicates that data points from the AllocationInstruction(35=J) and Confirmation(35=AK) are matched but there
	/// are variances. MatchExceptionGrp component may be used to detail on the mis-matched data fields)
	#[serde(rename = "3")]
	MismatchedAndConfirmationAreMatchedButThereAreVariancesMatchExceptionGrpComponentMayBeUsedToDetailOnTheMisMatchedDataFields,
}
