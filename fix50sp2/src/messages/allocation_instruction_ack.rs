
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Allocation {
	/// MsgType = P
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// AllocID
	#[serde(rename = "70")]
	pub alloc_id: String,
	/// May be used to link to a previously submitted AllocationInstructionAlertReques t(35=DU) message.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2758")]
	pub alloc_request_id: Option<String>,
	/// Parties
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// Optional second identifier for the allocation instruction being acknowledged (need not be unique)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "793")]
	pub secondary_alloc_id: Option<String>,
	/// TradeDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "75")]
	pub trade_date: Option<fix_common::LocalMktDate>,
	/// Date/Time <a href="message_Allocation_Instruction_Ack_P.html" target="main">Allocation Instruction Ack&nbsp;(P)</a> generated
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<fix_common::UTCTimestamp>,
	/// Denotes the status of the allocation instruction; received (but not yet processed), rejected (at block or account level) or
	/// accepted (and processed).
	#[serde(rename = "87")]
	pub alloc_status: AllocStatus,
	/// Required for <a href="tag_87_AllocStatus.html" target="bottom">AllocStatus&nbsp;(87)</a> = 1 ( block level reject) and for <a href="tag_87_AllocStatus.html" target="bottom">AllocStatus&nbsp;(87)</a> 2 (account level reject) if the individual accounts and reject reasons are not provided in this message
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "88")]
	pub alloc_rej_code: Option<AllocRejCode>,
	/// AllocType
	#[serde(rename = "626")]
	pub alloc_type: AllocType,
	/// Required if <a href="tag_626_AllocType.html" target="bottom">AllocType&nbsp;(626)</a> = 8 (Request to Intermediary) Indicates status that is requested to be transmitted to counterparty by the intermediary (i.e.
	/// clearing house)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "808")]
	pub alloc_intermed_req_type: Option<AllocIntermedReqType>,
	/// Denotes whether the financial details provided on the <a href="message_AllocationInstruction_J.html" target="main">Allocation Instruction&nbsp;(J)</a> were successfully matched.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "573")]
	pub match_status: Option<MatchStatus>,
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
	/// This repeating group is optionally used for messages with <a href="tag_87_AllocStatus.html" target="bottom">AllocStatus (87)&nbsp;(87)</a> = 2 (account level reject) to provide details of the individual accounts that caused the rejection, together with reject reasons.
	/// This group should not be populated when AllocStatus has any other value. Indicates number of allocation groups to follow
	#[serde(flatten)]
	pub alloc_ack_grp: Option<super::super::alloc_ack_grp::AllocAckGrp>,
	/// <p>Firm assigned entity identifier for the allocation</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1728")]
	pub firm_group_id: Option<String>,
	/// <p>Group identifier assigned by the clearing house</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1730")]
	pub alloc_group_id: Option<String>,
	/// <p>Firm designated group identifier for average pricing</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1731")]
	pub avg_px_group_id: Option<String>,
	/// Instrument
	#[serde(flatten)]
	pub instrument: Option<super::super::instrument::Instrument>,
	/// RejectText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1328")]
	pub reject_text: Option<String>,
	/// Must be set if <a href="tag_1665_EncodedRejectText.html" target="bottom">EncodedRejectTextLen(1665)&nbsp;(1665)</a> field is specified and must immediately precede it.
	#[serde(rename = "1664")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_1328_RejectText.html" target="bottom">RejectedText(1328)&nbsp;(1328)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding(347)&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "1665")]
	pub encoded_reject_text: Option<fix_common::EncodedText<1665>>,
	/// RegulatoryTradeIDGrp
	#[serde(flatten)]
	pub regulatory_trade_id_grp: Option<super::super::regulatory_trade_id_grp::RegulatoryTradeIDGrp>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
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
	/// Cancelled by intermediary
	#[serde(rename = "8")]
	CancelledByIntermediary,
	/// Claimed
	#[serde(rename = "9")]
	Claimed,
	/// Refused
	#[serde(rename = "10")]
	Refused,
	/// Pending give-up approval
	#[serde(rename = "11")]
	PendingGiveUpApproval,
	/// Cancelled
	#[serde(rename = "12")]
	Cancelled,
	/// Pending take-up approval
	#[serde(rename = "13")]
	PendingTakeUpApproval,
	/// Reversal pending
	#[serde(rename = "14")]
	ReversalPending,
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
	N0,
	/// Incorrect quantity
	#[serde(rename = "1")]
	N1,
	/// Incorrect averageg price
	#[serde(rename = "2")]
	N2,
	/// Unknown executing broker mnemonic
	#[serde(rename = "3")]
	N3,
	/// Commission difference
	#[serde(rename = "4")]
	N4,
	/// Unknown OrderID (37)
	#[serde(rename = "5")]
	N5,
	/// Unknown ListID (66)
	#[serde(rename = "6")]
	N6,
	/// Other (further in Text (58))
	#[serde(rename = "7")]
	N7,
	/// Incorrect allocated quantity
	#[serde(rename = "8")]
	N8,
	/// Calculation difference
	#[serde(rename = "9")]
	N9,
	/// Unknown or stale ExecID
	#[serde(rename = "10")]
	N10,
	/// Mismatched data
	#[serde(rename = "11")]
	N11,
	/// Unknown ClOrdID
	#[serde(rename = "12")]
	N12,
	/// Warehouse request rejected
	#[serde(rename = "13")]
	N13,
	/// Other
	#[serde(rename = "99")]
	N99,
	/// Duplicate or missing IndividualAllocId(467)
	#[serde(rename = "14")]
	N14,
	/// Trade not recognized
	#[serde(rename = "15")]
	N15,
	/// Trade previously allocated
	#[serde(rename = "16")]
	N16,
	/// Incorrect or missing instrument
	#[serde(rename = "17")]
	N17,
	/// Incorrect or missing settlement date
	#[serde(rename = "18")]
	N18,
	/// Incorrect or missing fund ID or fund name
	#[serde(rename = "19")]
	N19,
	/// Incorrect or missing settlement instructions
	#[serde(rename = "20")]
	N20,
	/// Incorrect or missing fees
	#[serde(rename = "21")]
	N21,
	/// Incorrect or missing tax
	#[serde(rename = "22")]
	N22,
	/// Unknown or missing party
	#[serde(rename = "23")]
	N23,
	/// Incorrect or missing side
	#[serde(rename = "24")]
	N24,
	/// Incorrect or missing net-money
	#[serde(rename = "25")]
	N25,
	/// Incorrect or missing trade date
	#[serde(rename = "26")]
	N26,
	/// Incorrect or missing settlement currency instructions
	#[serde(rename = "27")]
	N27,
	/// Incorrect or missing ProcessCode
	#[serde(rename = "28")]
	N28,
}

impl Default for AllocRejCode {
	fn default() -> Self {
		AllocRejCode::N0
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AllocType {
	/// Calculated (includes MiscFees and NetMoney)
	#[serde(rename = "1")]
	Calculated,
	/// Preliminary (without MiscFees and NetMoney)
	#[serde(rename = "2")]
	Preliminary,
	/// Sellside Calculated Using Preliminary (includes MiscFees and NetMoney) (Replaced)
	#[serde(rename = "3")]
	SellsideCalculatedUsingPreliminary,
	/// Sellside Calculated Without Preliminary (sent unsolicited by sellside, includes MiscFees and NetMoney) (Replaced)
	#[serde(rename = "4")]
	SellsideCalculatedWithoutPreliminary,
	/// Ready-To-Book - Single Order
	#[serde(rename = "5")]
	ReadyToBookSingleOrder,
	/// Buyside Ready-To-Book - Combined Set of Orders (Replaced)
	#[serde(rename = "6")]
	BuysideReadyToBookCombinedSetOfOrders,
	/// Warehouse Instruction
	#[serde(rename = "7")]
	WarehouseInstruction,
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
	/// Incomplete Group
	#[serde(rename = "12")]
	IncompleteGroup,
	/// Complete Group
	#[serde(rename = "13")]
	CompleteGroup,
	/// Reversal Pending
	#[serde(rename = "14")]
	ReversalPending,
	/// Reopen Group
	#[serde(rename = "15")]
	ReopenGroup,
	/// Cancel Group
	#[serde(rename = "16")]
	CancelGroup,
	/// Giveup
	#[serde(rename = "17")]
	Giveup,
	/// Takeup
	#[serde(rename = "18")]
	Takeup,
	/// Refuse Takeup
	#[serde(rename = "19")]
	RefuseTakeup,
	/// Initiate Reversal
	#[serde(rename = "20")]
	InitiateReversal,
	/// Reverse
	#[serde(rename = "21")]
	Reverse,
	/// Refuse Reversal
	#[serde(rename = "22")]
	RefuseReversal,
	/// Sub Allocation Giveup
	#[serde(rename = "23")]
	SubAllocationGiveup,
	/// Approve Giveup
	#[serde(rename = "24")]
	ApproveGiveup,
	/// Approve Takeup
	#[serde(rename = "25")]
	ApproveTakeup,
	/// Notional value average price group allocation
	#[serde(rename = "26")]
	NotionalValueAveragePriceGroupAllocation,
}

impl Default for AllocType {
	fn default() -> Self {
		AllocType::Calculated
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
	/// Mismatched (Indicates that data points from the AllocationInstruction(35=J) and Confirmation(35=AK) are matched but there
	/// are variances. MatchExceptionGrp component may be used to detail on the mis-matched data fields)
	#[serde(rename = "3")]
	MismatchedAndConfirmationAreMatchedButThereAreVariancesMatchExceptionGrpComponentMayBeUsedToDetailOnTheMisMatchedDataFields,
}

impl Default for MatchStatus {
	fn default() -> Self {
		MatchStatus::ComparedMatchedOrAffirmed
	}
}
