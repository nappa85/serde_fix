
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AllocationInstructionAlert {
	/// MsgType = BM
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// Unique identifier for this allocation instruction alert message
	#[serde(rename = "70")]
	pub alloc_id: String,
	/// i.e. New, Cancel, Replace
	#[serde(rename = "71")]
	pub alloc_trans_type: AllocTransType,
	/// Specifies the purpose or type of Allocation message
	#[serde(rename = "626")]
	pub alloc_type: AllocType,
	/// Optional second identifier for this allocation instruction (need not be unique)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "793")]
	pub secondary_alloc_id: Option<String>,
	/// Required for <a href="tag_71_AllocTransType.html" target="bottom">AllocTransType&nbsp;(71)</a> = Replace or Cancel
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "72")]
	pub ref_alloc_id: Option<String>,
	/// Required for <a href="tag_71_AllocTransType.html" target="bottom">AllocTransType&nbsp;(71)</a> = Replace or Cancel Gives the reason for replacing or cancelling the allocation instruction
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "796")]
	pub alloc_canc_replace_reason: Option<AllocCancReplaceReason>,
	/// Required if <a href="tag_626_AllocType.html" target="bottom">AllocType&nbsp;(626)</a> = 8 (Request to Intermediary) Indicates status that is requested to be transmitted to counterparty by the intermediary (i.e.
	/// clearing house)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "808")]
	pub alloc_intermed_req_type: Option<AllocIntermedReqType>,
	/// Can be used to link two different Allocation messages (each with unique AllocID) together, i.e. for F/X "Netting" or "Swaps".
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "196")]
	pub alloc_link_id: Option<String>,
	/// Can be used to link two different Allocation messages and identifies the type of link. Required if <a href="tag_196_AllocLinkID.html" target="bottom">AllocLinkID&nbsp;(196)</a> is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "197")]
	pub alloc_link_type: Option<AllocLinkType>,
	/// Can be used with AllocType="Ready-To-Book".
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "466")]
	pub booking_ref_id: Option<String>,
	/// Indicates how the orders being booked and allocated by this message are identified, i.e. by explicit definition in the <a href="tag_73_NoOrders.html" target="bottom">NoOrders&nbsp;(73)</a> group or not.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "857")]
	pub alloc_no_orders_type: Option<AllocNoOrdersType>,
	/// Indicates number of orders to be combined for allocation. If order(s) were manually delivered set to 1 (one).Required when <a href="tag_857_AllocNoOrdersType.html" target="bottom">AllocNoOrdersType&nbsp;(857)</a> = 1
	#[serde(flatten)]
	pub ord_alloc_grp: Option<super::super::ord_alloc_grp::OrdAllocGrp>,
	/// <p>Indicates number of individual execution or trade entries. Absence indicates that no execution or trade entries are included.
	/// Primarily used to support step-outs. Indicates number of individual execution repeating group entries to follow. Absence of
	/// this field indicates that no individual execution entries are included. Primarily used to support step-outs.
	/// </p>
	#[serde(flatten)]
	pub exec_alloc_grp: Option<super::super::exec_alloc_grp::ExecAllocGrp>,
	/// PreviouslyReported
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "570")]
	pub previously_reported: Option<PreviouslyReported>,
	/// ReversalIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "700")]
	pub reversal_indicator: Option<crate::entities::Boolean>,
	/// MatchType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "574")]
	pub match_type: Option<MatchType>,
	/// Side
	#[serde(rename = "54")]
	pub side: Side,
	/// Insert here the set of "Instrument" fields defined in "common components of application messages".
	#[serde(flatten)]
	pub instrument: super::super::instrument::Instrument,
	/// Insert here the set of "InstrumentExtension" fields defined in "common components of application messages".
	#[serde(flatten)]
	pub instrument_extension: Option<super::super::instrument_extension::InstrumentExtension>,
	/// Insert here the set of "FinancingDetails" fields defined in "common components of application messages".
	#[serde(flatten)]
	pub financing_details: Option<super::super::financing_details::FinancingDetails>,
	/// UndInstrmtGrp
	#[serde(flatten)]
	pub und_instrmt_grp: Option<super::super::und_instrmt_grp::UndInstrmtGrp>,
	/// InstrmtLegGrp
	#[serde(flatten)]
	pub instrmt_leg_grp: Option<super::super::instrmt_leg_grp::InstrmtLegGrp>,
	/// <p>When not using allocation groups, this is the total quantity (e.g. number of shares) allocated to all accounts, or that is
	/// Ready-ToBook. When using allocation groups, this is the quantity added or removed when trades are added to or removed from
	/// an allocation group. To remove quantity from the allocation group a negative value is specified in Quantity(53). When the
	/// allocation group quantity is unchanged, such as when AllocType(626) changes from 12(Incomplete group) to 13(Complete group)
	/// , the value for Quantity(53) should be zero (0).
	/// </p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "53")]
	pub quantity: Option<f64>,
	/// QtyType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "854")]
	pub qty_type: Option<QtyType>,
	/// Market of the executions.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "30")]
	pub last_mkt: Option<String>,
	/// TradeOriginationDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "229")]
	pub trade_origination_date: Option<crate::entities::LocalMktDate>,
	/// TradingSessionID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "336")]
	pub trading_session_id: Option<TradingSessionID>,
	/// TradingSessionSubID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "625")]
	pub trading_session_sub_id: Option<TradingSessionSubID>,
	/// PriceType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "423")]
	pub price_type: Option<PriceType>,
	/// For F/X orders, should be the "all-in" rate (spot rate adjusted for forward points). For 3rd party allocations used to convey
	/// either basic price or averaged price Optional for average price allocations in the listed derivatives markets where the central
	/// counterparty calculates and manages average price across an allocation group.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "6")]
	pub avg_px: Option<f64>,
	/// AvgParPx
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "860")]
	pub avg_par_px: Option<f64>,
	/// Insert here the set of "SpreadOrBenchmarkCurveData" fields defined in "common components of application messages".
	#[serde(flatten)]
	pub spread_or_benchmark_curve_data: Option<super::super::spread_or_benchmark_curve_data::SpreadOrBenchmarkCurveData>,
	/// Currency of AvgPx. Should be the currency of the local market or exchange where the trade was conducted.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "15")]
	pub currency: Option<Currency>,
	/// Absence of this field indicates that default precision arranged by the broker/institution is to be used
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "74")]
	pub avg_px_precision: Option<i32>,
	/// Insert here the set of "Parties" (firm identification) fields defined in "common components of application messages".
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// TradeDate
	#[serde(rename = "75")]
	pub trade_date: crate::entities::LocalMktDate,
	/// Date/time when allocation is generated
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<crate::entities::UTCTimestamp>,
	/// SettlType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "63")]
	pub settl_type: Option<SettlType>,
	/// Takes precedence over <a href="tag_63_SettlType.html" target="bottom">SettlType&nbsp;(63)</a> value and conditionally required/omitted for specific <a href="tag_63_SettlType.html" target="bottom">SettlType&nbsp;(63)</a> values.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "64")]
	pub settl_date: Option<crate::entities::LocalMktDate>,
	/// Method for booking. Used to provide notification that this is to be booked out as an OTC derivative (e.g. CFD or similar).
	/// Absence of this field implies regular booking.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "775")]
	pub booking_type: Option<BookingType>,
	/// Expressed in same currency as AvgPx. Sum of (AllocQty * AllocAvgPx or AllocPrice).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "381")]
	pub gross_trade_amt: Option<f64>,
	/// Concession
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "238")]
	pub concession: Option<f64>,
	/// TotalTakedown
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "237")]
	pub total_takedown: Option<f64>,
	/// Expressed in same currency as AvgPx. Sum of AllocNetMoney.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "118")]
	pub net_money: Option<f64>,
	/// PositionEffect
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "77")]
	pub position_effect: Option<PositionEffect>,
	/// Indicates if Allocation has been automatically accepted on behalf of the Carry Firm by the Clearing House
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "754")]
	pub auto_accept_indicator: Option<crate::entities::Boolean>,
	/// Text
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
	/// Applicable for Convertible Bonds and fixed income
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "157")]
	pub num_days_interest: Option<i32>,
	/// Applicable for Convertible Bonds and fixed income
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "158")]
	pub accrued_interest_rate: Option<f32>,
	/// Applicable for Convertible Bonds and fixed income (REMOVED FROM THIS LOCATION AS OF FIX 4.4, REPLACED BY AllocAccruedInterest)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "159")]
	pub accrued_interest_amt: Option<f64>,
	/// (Deprecated) use <a href="tag_159_AccruedInterestAmt.html" target="bottom">AccruedInterestAmt&nbsp;(159)</a> Sum of <a href="tag_159_AccruedInterestAmt.html" target="bottom">AccruedInterestAmt&nbsp;(159)</a> within repeating group.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "540")]
	pub total_accrued_interest_amt: Option<f64>,
	/// InterestAtMaturity
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "738")]
	pub interest_at_maturity: Option<f64>,
	/// For repurchase agreements the accrued interest on termination.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "920")]
	pub end_accrued_interest_amt: Option<f64>,
	/// For repurchase agreements the start (dirty) cash consideration
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "921")]
	pub start_cash: Option<f64>,
	/// For repurchase agreements the end (dirty) cash consideration
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "922")]
	pub end_cash: Option<f64>,
	/// LegalConfirm
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "650")]
	pub legal_confirm: Option<LegalConfirm>,
	/// Stipulations
	#[serde(flatten)]
	pub stipulations: Option<super::super::stipulations::Stipulations>,
	/// YieldData
	#[serde(flatten)]
	pub yield_data: Option<super::super::yield_data::YieldData>,
	/// Insert here here the set of "Position Amount Data" fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub position_amount_data: Option<super::super::position_amount_data::PositionAmountData>,
	/// Indicates total number of allocation groups (used to support fragmentation). Must equal the sum of all <a href="tag_78_NoAllocs.html" target="bottom">NoAllocs&nbsp;(78)</a> values across all message fragments making up this allocation instruction. Only required where message has been fragmented.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "892")]
	pub tot_no_allocs: Option<i32>,
	/// Indicates whether this is the last fragment in a sequence of message fragments. Only required where message has been fragmented.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "893")]
	pub last_fragment: Option<LastFragment>,
	/// Indicates number of allocation groups to follow. Not required for AllocTransType=Cancel Not required for AllocType="Ready-To-Book"
	/// or "Warehouse instruction".
	#[serde(flatten)]
	pub alloc_grp: Option<super::super::alloc_grp::AllocGrp>,
	/// Indicates if an allocation is to be average priced. Is also used to indicate if average price allocation group is complete
	/// or incomplete.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "819")]
	pub avg_px_indicator: Option<AvgPxIndicator>,
	/// Indicates Clearing Business Date for which transaction will be settled.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "715")]
	pub clearing_business_date: Option<crate::entities::LocalMktDate>,
	/// Indicates Trade Type of Allocation.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "828")]
	pub trd_type: Option<TrdType>,
	/// Indicates TradeSubType of Allocation. Necessary for defining groups.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "829")]
	pub trd_sub_type: Option<TrdSubType>,
	/// Indicates CTI of original trade marked for allocation.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "582")]
	pub cust_order_capacity: Option<CustOrderCapacity>,
	/// Indicates input source of original trade marked for allocation.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "578")]
	pub trade_input_source: Option<String>,
	/// Indicates MultiLegReportType of original trade marked for allocation.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "442")]
	pub multi_leg_reporting_type: Option<MultiLegReportingType>,
	/// Used to identify the event or source which gave rise to a message.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1011")]
	pub message_event_source: Option<String>,
	/// Specifies the rounded price to quoted precision.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "991")]
	pub rnd_px: Option<f64>,
	/// Identifies status of allocation.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "87")]
	pub alloc_status: Option<AllocStatus>,
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
	/// <p>Indicates the total quantity of an allocation group. Includes any allocated quantity.</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1736")]
	pub alloc_group_quantity: Option<f64>,
	/// <p>Indicates the remaining quantity of an allocation group that has not yet been allocated</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1737")]
	pub alloc_group_remaining_quantity: Option<f64>,
	/// CustOrderHandlingInst
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1031")]
	pub cust_order_handling_inst: Option<crate::entities::SeparatedValues<CustOrderHandlingInst>>,
	/// OrderHandlingInstSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1032")]
	pub order_handling_inst_source: Option<OrderHandlingInstSource>,
	/// PriceQualifierGrp
	#[serde(flatten)]
	pub price_qualifier_grp: Option<super::super::price_qualifier_grp::PriceQualifierGrp>,
	/// Identifier of the request this message is responding to when responding to an AllocationInstructionAlertRequest(35=DU).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2758")]
	pub alloc_request_id: Option<String>,
	/// GroupAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2759")]
	pub group_amount: Option<f64>,
	/// GroupRemainingAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2760")]
	pub group_remaining_amount: Option<f64>,
	/// Maybe used to indicate the highest price within the specified allocation group.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "332")]
	pub high_px: Option<f64>,
	/// Maybe used to indicate the lowest price within the specified allocation group.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "333")]
	pub low_px: Option<f64>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AllocCancReplaceReason {
	/// Original details incomplete/incorrect
	#[serde(rename = "1")]
	OriginalDetailsIncompleteIncorrect,
	/// Change in underlying order details
	#[serde(rename = "2")]
	ChangeInUnderlyingOrderDetails,
	/// Other
	#[serde(rename = "99")]
	Other,
	/// Cancelled by give-up firm
	#[serde(rename = "3")]
	CancelledByGiveUpFirm,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AllocLinkType {
	/// F/X Netting
	#[serde(rename = "0")]
	FXNetting,
	/// F/X Swap
	#[serde(rename = "1")]
	FXSwap,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AllocNoOrdersType {
	/// Not Specified
	#[serde(rename = "0")]
	NotSpecified,
	/// Explicit List Provided
	#[serde(rename = "1")]
	ExplicitListProvided,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PreviouslyReported {
	/// Not reported to counterparty or market
	#[serde(rename = "N")]
	NotReportedToCounterpartyOrMarket,
	/// Previously reported to counterparty or market
	#[serde(rename = "Y")]
	PreviouslyReportedToCounterpartyOrMarket,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MatchType {
	/// ACT Accepted Trade
	#[serde(rename = "M3")]
	ActAcceptedTrade,
	/// ACT Default Trade
	#[serde(rename = "M4")]
	ActDefaultTrade,
	/// ACT Default After M2
	#[serde(rename = "M5")]
	ActDefaultAfterM2,
	/// ACT M6 Match
	#[serde(rename = "M6")]
	ActM6Match,
	/// Exact match on Trade Date, Stock Symbol, Quantity, Price, Trade Type, and Special Trade Indicator plus four badges and execution
	/// time (within two-minute window)
	#[serde(rename = "A1")]
	ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusFourBadgesAndExecutionTime,
	/// Exact match on Trade Date, Stock Symbol, Quantity, Price, Trade Type, and Special Trade Indicator plus four badges
	#[serde(rename = "A2")]
	ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusFourBadges,
	/// Exact match on Trade Date, Stock Symbol, Quantity, Price, Trade Type, and Special Trade Indicator plus two badges and execution
	/// time (within two-minute window)
	#[serde(rename = "A3")]
	ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusTwoBadgesAndExecutionTime,
	/// Exact match on Trade Date, Stock Symbol, Quantity, Price, Trade Type, and Special Trade Indicator plus two badges
	#[serde(rename = "A4")]
	ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusTwoBadges,
	/// Exact match on Trade Date, Stock Symbol, Quantity, Price, Trade Type, and Special Trade Indicator plus execution time (within
	/// two-minute window)
	#[serde(rename = "A5")]
	ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusExecutionTime,
	/// Compared records resulting from stamped advisories or specialist accepts/pair-offs
	#[serde(rename = "AQ")]
	ComparedRecordsResultingFromStampedAdvisoriesOrSpecialistAcceptsPairOffs,
	/// Summarized Match using A1 exact match criteria except quantity is summarized
	#[serde(rename = "S1")]
	SummarizedMatchUsingA1ExactMatchCriteriaExceptQuantityIsSummarized,
	/// Summarized Match using A2 exact match criteria except quantity is summarized
	#[serde(rename = "S2")]
	SummarizedMatchUsingA2ExactMatchCriteriaExceptQuantityIsSummarized,
	/// Summarized Match using A3 exact match criteria except quantity is summarized
	#[serde(rename = "S3")]
	SummarizedMatchUsingA3ExactMatchCriteriaExceptQuantityIsSummarized,
	/// Summarized Match using A4 exact match criteria except quantity is summarized
	#[serde(rename = "S4")]
	SummarizedMatchUsingA4ExactMatchCriteriaExceptQuantityIsSummarized,
	/// Summarized Match using A5 exact match criteria except quantity is summarized
	#[serde(rename = "S5")]
	SummarizedMatchUsingA5ExactMatchCriteriaExceptQuantityIsSummarized,
	/// Exact Match on Trade Date, Stock Symbol, Quantity, Price, Trade Type, and Special Trade Indicator minus badges and times
	#[serde(rename = "M1")]
	ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorMinusBadgesAndTimes,
	/// Summarized Match minus badges and times
	#[serde(rename = "M2")]
	SummarizedMatchMinusBadgesAndTimes,
	/// OCS Locked In
	#[serde(rename = "MT")]
	OcsLockedIn,
	/// One-Party Trade Report (privately negotiated trade)
	#[serde(rename = "1")]
	OnePartyTradeReport,
	/// Two-Party Trade Report (privately negotiated trade)
	#[serde(rename = "2")]
	TwoPartyTradeReport,
	/// Confirmed Trade Report (reporting from recognized markets)
	#[serde(rename = "3")]
	ConfirmedTradeReport,
	/// Auto-match
	#[serde(rename = "4")]
	AutoMatch,
	/// Cross Auction
	#[serde(rename = "5")]
	CrossAuction,
	/// Counter-Order Selection
	#[serde(rename = "6")]
	CounterOrderSelection,
	/// Call Auction
	#[serde(rename = "7")]
	CallAuction,
	/// Issuing/Buy Back Auction
	#[serde(rename = "8")]
	IssuingBuyBackAuction,
	/// Systematic Internaliser (SI)
	#[serde(rename = "9")]
	SystematicInternaliser,
	/// Auto-match with last look
	#[serde(rename = "10")]
	AutoMatchWithLastLook,
	/// Cross auction with last look
	#[serde(rename = "11")]
	CrossAuctionWithLastLook,
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
	/// Sell undisclosed
	#[serde(rename = "H")]
	SellUndisclosed,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum QtyType {
	/// Units (shares, par, currency)
	#[serde(rename = "0")]
	Units,
	/// Contracts (if used - must specify <a href="tag_231_ContractMultiplier.html" target="bottom">ContractMultiplier&nbsp;(231)</a> )
	#[serde(rename = "1")]
	ContractsA,
	/// Units of Measure per Time Unit (if used - must specify <a href="tag_996_UnitOfMeasure.html" target="bottom">UnitofMeasure&nbsp;(996)</a> and <a href="tag_997_TimeUnit.html" target="bottom">TimeUnit&nbsp;(997)</a> )
	#[serde(rename = "2")]
	UnitsOfMeasurePerTimeUnitAAndAHrefTag997TimeUnitHtmlTargetBottomTimeUnitNbspA,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TradingSessionID {
	/// Day
	#[serde(rename = "1")]
	Day,
	/// HalfDay
	#[serde(rename = "2")]
	HalfDay,
	/// Morning
	#[serde(rename = "3")]
	Morning,
	/// Afternoon
	#[serde(rename = "4")]
	Afternoon,
	/// Evening
	#[serde(rename = "5")]
	Evening,
	/// After-hours
	#[serde(rename = "6")]
	AfterHours,
	/// Holiday
	#[serde(rename = "7")]
	Holiday,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TradingSessionSubID {
	/// Pre-Trading
	#[serde(rename = "1")]
	PreTrading,
	/// Opening or opening auction
	#[serde(rename = "2")]
	OpeningOrOpeningAuction,
	/// (Continuous) Trading
	#[serde(rename = "3")]
	Trading,
	/// Closing or closing auction
	#[serde(rename = "4")]
	ClosingOrClosingAuction,
	/// Post-Trading
	#[serde(rename = "5")]
	PostTrading,
	/// Intraday Auction
	#[serde(rename = "6")]
	IntradayAuction,
	/// Quiescent
	#[serde(rename = "7")]
	Quiescent,
	/// Any auction
	#[serde(rename = "8")]
	AnyAuction,
	/// Unscheduled intraday auction (Elaboration: An unscheduled intraday auction might be triggered by a circuit breaker)
	#[serde(rename = "9")]
	UnscheduledIntradayAuction,
	/// Out of main session trading (Elaboration: In the context of Market Model Typology "Out of main session trading" refers to
	/// both before and after session, neither auction nor continuous trading)
	#[serde(rename = "10")]
	OutOfMainSessionTrading,
	/// Private auction
	#[serde(rename = "11")]
	PrivateAuction,
	/// Public auction
	#[serde(rename = "12")]
	PublicAuction,
	/// Group auction
	#[serde(rename = "13")]
	GroupAuction,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PriceType {
	/// Percentage (e.g. percent of par) (often called "dollar price" for fixed income)
	#[serde(rename = "1")]
	Percentage,
	/// Per unit (i.e. per share or contract)
	#[serde(rename = "2")]
	PerUnit,
	/// Fixed Amount (absolute value)
	#[serde(rename = "3")]
	FixedAmount,
	/// Discount - percentage points below par
	#[serde(rename = "4")]
	DiscountPercentagePointsBelowPar,
	/// Premium - percentage points over par
	#[serde(rename = "5")]
	PremiumPercentagePointsOverPar,
	/// Spread
	#[serde(rename = "6")]
	Spread,
	/// TED price
	#[serde(rename = "7")]
	TedPrice,
	/// TED yield
	#[serde(rename = "8")]
	TedYield,
	/// Yield
	#[serde(rename = "9")]
	Yield,
	/// Fixed cabinet trade price (primarily for listed futures and options)
	#[serde(rename = "10")]
	FixedCabinetTradePrice,
	/// Variable cabinet trade price (primarily for listed futures and options)
	#[serde(rename = "11")]
	VariableCabinetTradePrice,
	/// Price spread
	#[serde(rename = "12")]
	PriceSpread,
	/// Product ticks in halves
	#[serde(rename = "13")]
	ProductTicksInHalves,
	/// Product ticks in fourths
	#[serde(rename = "14")]
	ProductTicksInFourths,
	/// Product ticks in eighths
	#[serde(rename = "15")]
	ProductTicksInEighths,
	/// Product ticks in sixteenths
	#[serde(rename = "16")]
	ProductTicksInSixteenths,
	/// Product ticks in thirty-seconds
	#[serde(rename = "17")]
	ProductTicksInThirtySeconds,
	/// Product ticks in sixty-fourths
	#[serde(rename = "18")]
	ProductTicksInSixtyFourths,
	/// Product ticks in one-twenty-eighths
	#[serde(rename = "19")]
	ProductTicksInOneTwentyEighths,
	/// Normal rate representation (e.g. FX rate) (represents the number of units of currency 2 that is required to purchase one unit
	/// of currency 1.)
	#[serde(rename = "20")]
	NormalRateRepresentation,
	/// Inverse rate representation (e.g. FX rate) (represents the number of units of 1 that is needed to purchase one unit of currency
	/// 2.)
	#[serde(rename = "21")]
	InverseRateRepresentation,
	/// Basis points (when the price is not spread based)
	#[serde(rename = "22")]
	BasisPoints,
	/// Upfront points (used specifically for CDS pricing)
	#[serde(rename = "23")]
	UpfrontPoints,
	/// Interest rate (Elaboration: When the price is an interest rate. For example, used with benchmark reference rate)
	#[serde(rename = "24")]
	InterestRate,
	/// Percentage of notional
	#[serde(rename = "25")]
	PercentageOfNotional,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Currency {
	/// Afghani
	#[serde(rename = "AFA")]
	Afa,
	/// Algerian Dinar
	#[serde(rename = "DZD")]
	Dzd,
	/// Andorran Peseta
	#[serde(rename = "ADP")]
	Adp,
	/// Argentine Peso
	#[serde(rename = "ARS")]
	Ars,
	/// Armenian Dram
	#[serde(rename = "AMD")]
	Amd,
	/// Aruban Guilder
	#[serde(rename = "AWG")]
	Awg,
	/// Australian Dollar
	#[serde(rename = "AUD")]
	Aud,
	/// Azerbaijanian Manat
	#[serde(rename = "AZM")]
	Azm,
	/// Bahamian Dollar
	#[serde(rename = "BSD")]
	Bsd,
	/// Bahraini Dinar
	#[serde(rename = "BHD")]
	Bhd,
	/// Baht
	#[serde(rename = "THB")]
	Thb,
	/// Balboa
	#[serde(rename = "PAB")]
	Pab,
	/// Barbados Dollar
	#[serde(rename = "BBD")]
	Bbd,
	/// Belarussian Ruble
	#[serde(rename = "BYB")]
	Byb,
	/// Belgian Franc
	#[serde(rename = "BEF")]
	Bef,
	/// Belize Dollar
	#[serde(rename = "BZD")]
	Bzd,
	/// Bermudian Dollar
	#[serde(rename = "BMD")]
	Bmd,
	/// Bolivar
	#[serde(rename = "VEB")]
	Veb,
	/// Boliviano
	#[serde(rename = "BOB")]
	Bob,
	/// Brazilian Real
	#[serde(rename = "BRL")]
	Brl,
	/// Brunei Dollar
	#[serde(rename = "BND")]
	Bnd,
	/// Burundi Franc
	#[serde(rename = "BIF")]
	Bif,
	/// CFA Franc BCEAO+
	#[serde(rename = "XOF")]
	Xof,
	/// CFA Franc BEAC#
	#[serde(rename = "XAF")]
	Xaf,
	/// CFP Franc
	#[serde(rename = "XPF")]
	Xpf,
	/// Canadian Dollar
	#[serde(rename = "CAD")]
	Cad,
	/// Cape Verde Escudo
	#[serde(rename = "CVE")]
	Cve,
	/// Cayman Islands Dollar
	#[serde(rename = "KYD")]
	Kyd,
	/// Cedi
	#[serde(rename = "GHC")]
	Ghc,
	/// Chilean Peso
	#[serde(rename = "CLP")]
	Clp,
	/// Colombian Peso
	#[serde(rename = "COP")]
	Cop,
	/// Comoro Franc
	#[serde(rename = "KMF")]
	Kmf,
	/// Convertible Marks
	#[serde(rename = "BAM")]
	Bam,
	/// Cordoba Oro
	#[serde(rename = "NIO")]
	Nio,
	/// Costa Rican Colon
	#[serde(rename = "CRC")]
	Crc,
	/// Cuban Peso
	#[serde(rename = "CUP")]
	Cup,
	/// Cyprus Pound
	#[serde(rename = "CYP")]
	Cyp,
	/// Czech Koruna
	#[serde(rename = "CZK")]
	Czk,
	/// Dalasi
	#[serde(rename = "GMD")]
	Gmd,
	/// Danish Krone
	#[serde(rename = "DKK")]
	Dkk,
	/// Denar
	#[serde(rename = "MKD")]
	Mkd,
	/// Deutsche Mark
	#[serde(rename = "DEM")]
	Dem,
	/// Djibouti Franc
	#[serde(rename = "DJF")]
	Djf,
	/// Dobra
	#[serde(rename = "STD")]
	Std,
	/// Dominican Peso
	#[serde(rename = "DOP")]
	Dop,
	/// Dong
	#[serde(rename = "VND")]
	Vnd,
	/// Drachma
	#[serde(rename = "GRD")]
	Grd,
	/// East Caribbean Dollar
	#[serde(rename = "XCD")]
	Xcd,
	/// Egyptian Pound
	#[serde(rename = "EGP")]
	Egp,
	/// El Salvador Colon
	#[serde(rename = "SVC")]
	Svc,
	/// Ethiopian Birr
	#[serde(rename = "ETB")]
	Etb,
	/// Euro
	#[serde(rename = "EUR")]
	Eur,
	/// Falkland Islands Pound
	#[serde(rename = "FKP")]
	Fkp,
	/// Fiji Dollar
	#[serde(rename = "FJD")]
	Fjd,
	/// Forint
	#[serde(rename = "HUF")]
	Huf,
	/// Franc Congolais
	#[serde(rename = "CDF")]
	Cdf,
	/// French Franc
	#[serde(rename = "FRF")]
	Frf,
	/// Gibraltar Pound
	#[serde(rename = "GIP")]
	Gip,
	/// Gourde
	#[serde(rename = "HTG")]
	Htg,
	/// Guarani
	#[serde(rename = "PYG")]
	Pyg,
	/// Guinea Franc
	#[serde(rename = "GNF")]
	Gnf,
	/// Guinea-Bissau Peso
	#[serde(rename = "GWP")]
	Gwp,
	/// Guyana Dollar
	#[serde(rename = "GYD")]
	Gyd,
	/// Hong Kong Dollar
	#[serde(rename = "HKD")]
	Hkd,
	/// Hryvnia
	#[serde(rename = "UAH")]
	Uah,
	/// Iceland Krona
	#[serde(rename = "ISK")]
	Isk,
	/// Indian Rupee
	#[serde(rename = "INR")]
	Inr,
	/// Iranian Rial
	#[serde(rename = "IRR")]
	Irr,
	/// Iraqi Dinar
	#[serde(rename = "IQD")]
	Iqd,
	/// Irish Pound
	#[serde(rename = "IEP")]
	Iep,
	/// Italian Lira
	#[serde(rename = "ITL")]
	Itl,
	/// Jamaican Dollar
	#[serde(rename = "JMD")]
	Jmd,
	/// Jordanian Dinar
	#[serde(rename = "JOD")]
	Jod,
	/// Kenyan Shilling
	#[serde(rename = "KES")]
	Kes,
	/// Kina
	#[serde(rename = "PGK")]
	Pgk,
	/// Kip
	#[serde(rename = "LAK")]
	Lak,
	/// Kroon
	#[serde(rename = "EEK")]
	Eek,
	/// Kuna
	#[serde(rename = "HRK")]
	Hrk,
	/// Kuwaiti Dinar
	#[serde(rename = "KWD")]
	Kwd,
	/// Kwacha
	#[serde(rename = "MWK")]
	Mwk,
	/// Kwacha
	#[serde(rename = "ZMK")]
	Zmk,
	/// Kwanza Reajustado
	#[serde(rename = "AOR")]
	Aor,
	/// Kyat
	#[serde(rename = "MMK")]
	Mmk,
	/// Lari
	#[serde(rename = "GEL")]
	Gel,
	/// Latvian Lats
	#[serde(rename = "LVL")]
	Lvl,
	/// Lebanese Pound
	#[serde(rename = "LBP")]
	Lbp,
	/// Lek
	#[serde(rename = "ALL")]
	All,
	/// Lempira
	#[serde(rename = "HNL")]
	Hnl,
	/// Leone
	#[serde(rename = "SLL")]
	Sll,
	/// Leu
	#[serde(rename = "ROL")]
	Rol,
	/// Lev
	#[serde(rename = "BGL")]
	Bgl,
	/// Liberian Dollar
	#[serde(rename = "LRD")]
	Lrd,
	/// Libyan Dinar
	#[serde(rename = "LYD")]
	Lyd,
	/// Lilangeni
	#[serde(rename = "SZL")]
	Szl,
	/// Lithuanian Litas
	#[serde(rename = "LTL")]
	Ltl,
	/// Loti
	#[serde(rename = "LSL")]
	Lsl,
	/// Luxembourg Franc
	#[serde(rename = "LUF")]
	Luf,
	/// Malagasy Franc
	#[serde(rename = "MGF")]
	Mgf,
	/// Malaysian Ringgit
	#[serde(rename = "MYR")]
	Myr,
	/// Maltese Lira
	#[serde(rename = "MTL")]
	Mtl,
	/// Manat
	#[serde(rename = "TMM")]
	Tmm,
	/// Markka
	#[serde(rename = "FIM")]
	Fim,
	/// Mauritius Rupee
	#[serde(rename = "MUR")]
	Mur,
	/// Metical
	#[serde(rename = "MZM")]
	Mzm,
	/// Mexican Peso
	#[serde(rename = "MXN")]
	Mxn,
	/// Mexican Unidad de Inversion (UDI)
	#[serde(rename = "MXV")]
	Mxv,
	/// Moldovan Leu
	#[serde(rename = "MDL")]
	Mdl,
	/// Moroccan Dirham
	#[serde(rename = "MAD")]
	Mad,
	/// Mvdol
	#[serde(rename = "BOV")]
	Bov,
	/// Naira
	#[serde(rename = "NGN")]
	Ngn,
	/// Nakfa
	#[serde(rename = "ERN")]
	Ern,
	/// Namibia Dollar
	#[serde(rename = "NAD")]
	Nad,
	/// Nepalese Rupee
	#[serde(rename = "NPR")]
	Npr,
	/// Netherlands Antillian Guilder
	#[serde(rename = "ANG")]
	Ang,
	/// Netherlands Guilder
	#[serde(rename = "NLG")]
	Nlg,
	/// New Dinar
	#[serde(rename = "YUM")]
	Yum,
	/// New Israeli Sheqel
	#[serde(rename = "ILS")]
	Ils,
	/// New Kwanza
	#[serde(rename = "AON")]
	Aon,
	/// New Taiwan Dollar
	#[serde(rename = "TWD")]
	Twd,
	/// New Zaire
	#[serde(rename = "ZRN")]
	Zrn,
	/// New Zealand Dollar
	#[serde(rename = "NZD")]
	Nzd,
	/// Next day
	#[serde(rename = "USN")]
	Usn,
	/// Ngultrum
	#[serde(rename = "BTN")]
	Btn,
	/// North Korean Won
	#[serde(rename = "KPW")]
	Kpw,
	/// Norwegian Krone
	#[serde(rename = "NOK")]
	Nok,
	/// Nuevo Sol
	#[serde(rename = "PEN")]
	Pen,
	/// Ouguiya
	#[serde(rename = "MRO")]
	Mro,
	/// Pa'anga
	#[serde(rename = "TOP")]
	Top,
	/// Pakistan Rupee
	#[serde(rename = "PKR")]
	Pkr,
	/// Pataca
	#[serde(rename = "MOP")]
	Mop,
	/// Peso Uruguayo
	#[serde(rename = "UYU")]
	Uyu,
	/// Philippine Peso
	#[serde(rename = "PHP")]
	Php,
	/// Portuguese Escudo
	#[serde(rename = "PTE")]
	Pte,
	/// Pound Sterling
	#[serde(rename = "GBP")]
	Gbp,
	/// Pula
	#[serde(rename = "BWP")]
	Bwp,
	/// Qatari Rial
	#[serde(rename = "QAR")]
	Qar,
	/// Quetzal
	#[serde(rename = "GTQ")]
	Gtq,
	/// Rand
	#[serde(rename = "ZAR")]
	Zar,
	/// Rial Omani
	#[serde(rename = "OMR")]
	Omr,
	/// Riel
	#[serde(rename = "KHR")]
	Khr,
	/// Rufiyaa
	#[serde(rename = "MVR")]
	Mvr,
	/// Rupiah
	#[serde(rename = "IDR")]
	Idr,
	/// Russian Ruble
	#[serde(rename = "RUB")]
	Rub,
	/// Russian Ruble
	#[serde(rename = "RUR")]
	Rur,
	/// Rwanda Franc
	#[serde(rename = "RWF")]
	Rwf,
	/// SDR
	#[serde(rename = "XDR")]
	Xdr,
	/// Same day
	#[serde(rename = "USS")]
	Uss,
	/// Saudi Riyal
	#[serde(rename = "SAR")]
	Sar,
	/// Schilling
	#[serde(rename = "ATS")]
	Ats,
	/// Seychelles Rupee
	#[serde(rename = "SCR")]
	Scr,
	/// Singapore Dollar
	#[serde(rename = "SGD")]
	Sgd,
	/// Slovak Koruna
	#[serde(rename = "SKK")]
	Skk,
	/// Solomon Islands Dollar
	#[serde(rename = "SBD")]
	Sbd,
	/// Som
	#[serde(rename = "KGS")]
	Kgs,
	/// Somali Shilling
	#[serde(rename = "SOS")]
	Sos,
	/// Spanish Peseta
	#[serde(rename = "ESP")]
	Esp,
	/// Sri Lanka Rupee
	#[serde(rename = "LKR")]
	Lkr,
	/// St Helena Pound
	#[serde(rename = "SHP")]
	Shp,
	/// Sucre
	#[serde(rename = "ECS")]
	Ecs,
	/// Sudanese Dinar
	#[serde(rename = "SDD")]
	Sdd,
	/// Surinam Guilder
	#[serde(rename = "SRG")]
	Srg,
	/// Swedish Krona
	#[serde(rename = "SEK")]
	Sek,
	/// Swiss Franc
	#[serde(rename = "CHF")]
	Chf,
	/// Syrian Pound
	#[serde(rename = "SYP")]
	Syp,
	/// Tajik Ruble
	#[serde(rename = "TJR")]
	Tjr,
	/// Taka
	#[serde(rename = "BDT")]
	Bdt,
	/// Tala
	#[serde(rename = "WST")]
	Wst,
	/// Tanzanian Shilling
	#[serde(rename = "TZS")]
	Tzs,
	/// Tenge
	#[serde(rename = "KZT")]
	Kzt,
	/// Timor Escudo
	#[serde(rename = "TPE")]
	Tpe,
	/// Tolar
	#[serde(rename = "SIT")]
	Sit,
	/// Trinidad and Tobago Dollar
	#[serde(rename = "TTD")]
	Ttd,
	/// Tugrik
	#[serde(rename = "MNT")]
	Mnt,
	/// Tunisian Dinar
	#[serde(rename = "TND")]
	Tnd,
	/// Turkish Lira
	#[serde(rename = "TRL")]
	Trl,
	/// UAE Dirham
	#[serde(rename = "AED")]
	Aed,
	/// US Dollar
	#[serde(rename = "USD")]
	Usd,
	/// Uganda Shilling
	#[serde(rename = "UGX")]
	Ugx,
	/// Unidad de Valor Constante (UVC)
	#[serde(rename = "ECV")]
	Ecv,
	/// Unidades de fomento
	#[serde(rename = "CLF")]
	Clf,
	/// Uzbekistan Sum
	#[serde(rename = "UZS")]
	Uzs,
	/// Vatu
	#[serde(rename = "VUV")]
	Vuv,
	/// Won
	#[serde(rename = "KRW")]
	Krw,
	/// Yemeni Rial
	#[serde(rename = "YER")]
	Yer,
	/// Yen
	#[serde(rename = "JPY")]
	Jpy,
	/// Yuan Renminbi
	#[serde(rename = "CNY")]
	Cny,
	/// Zimbabwe Dollar
	#[serde(rename = "ZWD")]
	Zwd,
	/// Zloty
	#[serde(rename = "PLN")]
	Pln,
	/// financial Rand
	#[serde(rename = "ZAL")]
	Zal,
	/// Afghani
	#[serde(rename = "004")]
	N004,
	/// Algerian Dinar
	#[serde(rename = "01")]
	N01,
	/// Andorran Peseta
	#[serde(rename = "020")]
	N020,
	/// Argentine Peso
	#[serde(rename = "032")]
	N032,
	/// Armenian Dram
	#[serde(rename = "051")]
	N051,
	/// Aruban Guilder
	#[serde(rename = "533")]
	N533,
	/// Australian Dollar
	#[serde(rename = "036")]
	N036,
	/// Azerbaijanian Manat
	#[serde(rename = "031")]
	N031,
	/// Bahamian Dollar
	#[serde(rename = "044")]
	N044,
	/// Bahraini Dinar
	#[serde(rename = "048")]
	N048,
	/// Baht
	#[serde(rename = "764")]
	N764,
	/// Balboa
	#[serde(rename = "590")]
	N590,
	/// Barbados Dollar
	#[serde(rename = "052")]
	N052,
	/// Belarussian Ruble
	#[serde(rename = "112")]
	N112,
	/// Belgian Franc
	#[serde(rename = "056")]
	N056,
	/// Belize Dollar
	#[serde(rename = "084")]
	N084,
	/// Bermudian Dollar
	#[serde(rename = "060")]
	N060,
	/// Bolivar
	#[serde(rename = "862")]
	N862,
	/// Boliviano
	#[serde(rename = "068")]
	N068,
	/// Brazilian Real
	#[serde(rename = "986")]
	N986,
	/// Brunei Dollar
	#[serde(rename = "096")]
	N096,
	/// Burundi Franc
	#[serde(rename = "108")]
	N108,
	/// CFA Franc BCEAO+
	#[serde(rename = "952")]
	N952,
	/// CFA Franc BEAC#
	#[serde(rename = "950")]
	N950,
	/// CFP Franc
	#[serde(rename = "953")]
	N953,
	/// Canadian Dollar
	#[serde(rename = "124")]
	N124,
	/// Cape Verde Escudo
	#[serde(rename = "132")]
	N132,
	/// Cayman Islands Dollar
	#[serde(rename = "136")]
	N136,
	/// Cedi
	#[serde(rename = "288")]
	N288,
	/// Chilean Peso
	#[serde(rename = "152")]
	N152,
	/// Colombian Peso
	#[serde(rename = "170")]
	N170,
	/// Comoro Franc
	#[serde(rename = "174")]
	N174,
	/// Convertible Marks
	#[serde(rename = "977")]
	N977,
	/// Cordoba Oro
	#[serde(rename = "558")]
	N558,
	/// Costa Rican Colon
	#[serde(rename = "188")]
	N188,
	/// Cuban Peso
	#[serde(rename = "192")]
	N192,
	/// Cyprus Pound
	#[serde(rename = "196")]
	N196,
	/// Czech Koruna
	#[serde(rename = "203")]
	N203,
	/// Dalasi
	#[serde(rename = "270")]
	N270,
	/// Danish Krone
	#[serde(rename = "208")]
	N208,
	/// Denar
	#[serde(rename = "807")]
	N807,
	/// Deutsche Mark
	#[serde(rename = "280")]
	N280,
	/// Djibouti Franc
	#[serde(rename = "262")]
	N262,
	/// Dobra
	#[serde(rename = "678")]
	N678,
	/// Dominican Peso
	#[serde(rename = "214")]
	N214,
	/// Dong
	#[serde(rename = "704")]
	N704,
	/// Drachma
	#[serde(rename = "300")]
	N300,
	/// East Caribbean Dollar
	#[serde(rename = "951")]
	N951,
	/// Egyptian Pound
	#[serde(rename = "818")]
	N818,
	/// El Salvador Colon
	#[serde(rename = "222")]
	N222,
	/// Ethiopian Birr
	#[serde(rename = "230")]
	N230,
	/// Euro
	#[serde(rename = "978")]
	N978,
	/// Falkland Islands Pound
	#[serde(rename = "238")]
	N238,
	/// Fiji Dollar
	#[serde(rename = "242")]
	N242,
	/// Forint
	#[serde(rename = "348")]
	N348,
	/// Franc Congolais
	#[serde(rename = "976")]
	N976,
	/// French Franc
	#[serde(rename = "250")]
	N250,
	/// Gibraltar Pound
	#[serde(rename = "292")]
	N292,
	/// Gourde
	#[serde(rename = "332")]
	N332,
	/// Guarani
	#[serde(rename = "600")]
	N600,
	/// Guinea Franc
	#[serde(rename = "324")]
	N324,
	/// Guinea-Bissau Peso
	#[serde(rename = "624")]
	N624,
	/// Guyana Dollar
	#[serde(rename = "328")]
	N328,
	/// Hong Kong Dollar
	#[serde(rename = "344")]
	N344,
	/// Hryvnia
	#[serde(rename = "980")]
	N980,
	/// Iceland Krona
	#[serde(rename = "352")]
	N352,
	/// Indian Rupee
	#[serde(rename = "356")]
	N356,
	/// Iranian Rial
	#[serde(rename = "364")]
	N364,
	/// Iraqi Dinar
	#[serde(rename = "368")]
	N368,
	/// Irish Pound
	#[serde(rename = "372")]
	N372,
	/// Italian Lira
	#[serde(rename = "380")]
	N380,
	/// Jamaican Dollar
	#[serde(rename = "388")]
	N388,
	/// Jordanian Dinar
	#[serde(rename = "400")]
	N400,
	/// Kenyan Shilling
	#[serde(rename = "404")]
	N404,
	/// Kina
	#[serde(rename = "598")]
	N598,
	/// Kip
	#[serde(rename = "418")]
	N418,
	/// Kroon
	#[serde(rename = "233")]
	N233,
	/// Kuna
	#[serde(rename = "191")]
	N191,
	/// Kuwaiti Dinar
	#[serde(rename = "414")]
	N414,
	/// Kwacha
	#[serde(rename = "454")]
	N454,
	/// Kwacha
	#[serde(rename = "894")]
	N894,
	/// Kwanza Reajustado
	#[serde(rename = "982")]
	N982,
	/// Kyat
	#[serde(rename = "104")]
	N104,
	/// Lari
	#[serde(rename = "981")]
	N981,
	/// Latvian Lats
	#[serde(rename = "428")]
	N428,
	/// Lebanese Pound
	#[serde(rename = "422")]
	N422,
	/// Lek
	#[serde(rename = "008")]
	N008,
	/// Lempira
	#[serde(rename = "340")]
	N340,
	/// Leone
	#[serde(rename = "694")]
	N694,
	/// Leu
	#[serde(rename = "642")]
	N642,
	/// Lev
	#[serde(rename = "100")]
	N100,
	/// Liberian Dollar
	#[serde(rename = "430")]
	N430,
	/// Libyan Dinar
	#[serde(rename = "434")]
	N434,
	/// Lilangeni
	#[serde(rename = "748")]
	N748,
	/// Lithuanian Litas
	#[serde(rename = "440")]
	N440,
	/// Loti
	#[serde(rename = "426")]
	N426,
	/// Luxembourg Franc
	#[serde(rename = "442")]
	N442,
	/// Malagasy Franc
	#[serde(rename = "450")]
	N450,
	/// Malaysian Ringgit
	#[serde(rename = "458")]
	N458,
	/// Maltese Lira
	#[serde(rename = "470")]
	N470,
	/// Manat
	#[serde(rename = "795")]
	N795,
	/// Markka
	#[serde(rename = "246")]
	N246,
	/// Mauritius Rupee
	#[serde(rename = "480")]
	N480,
	/// Metical
	#[serde(rename = "508")]
	N508,
	/// Mexican Peso
	#[serde(rename = "484")]
	N484,
	/// Mexican Unidad de Inversion (UDI)
	#[serde(rename = "979")]
	N979,
	/// Moldovan Leu
	#[serde(rename = "498")]
	N498,
	/// Moroccan Dirham
	#[serde(rename = "504")]
	N504,
	/// Mvdol
	#[serde(rename = "984")]
	N984,
	/// Naira
	#[serde(rename = "566")]
	N566,
	/// Nakfa
	#[serde(rename = "232")]
	N232,
	/// Namibia Dollar
	#[serde(rename = "516")]
	N516,
	/// Nepalese Rupee
	#[serde(rename = "524")]
	N524,
	/// Netherlands Antillian Guilder
	#[serde(rename = "532")]
	N532,
	/// Netherlands Guilder
	#[serde(rename = "528")]
	N528,
	/// New Dinar
	#[serde(rename = "891")]
	N891,
	/// New Israeli Sheqel
	#[serde(rename = "376")]
	N376,
	/// New Kwanza
	#[serde(rename = "02")]
	N02,
	/// New Taiwan Dollar
	#[serde(rename = "901")]
	N901,
	/// New Zaire
	#[serde(rename = "180")]
	N180,
	/// New Zealand Dollar
	#[serde(rename = "554")]
	N554,
	/// Next day
	#[serde(rename = "997")]
	N997,
	/// Ngultrum
	#[serde(rename = "064")]
	N064,
	/// North Korean Won
	#[serde(rename = "408")]
	N408,
	/// Norwegian Krone
	#[serde(rename = "578")]
	N578,
	/// Nuevo Sol
	#[serde(rename = "604")]
	N604,
	/// Ouguiya
	#[serde(rename = "478")]
	N478,
	/// Pa'anga
	#[serde(rename = "776")]
	N776,
	/// Pakistan Rupee
	#[serde(rename = "586")]
	N586,
	/// Pataca
	#[serde(rename = "446")]
	N446,
	/// Peso Uruguayo
	#[serde(rename = "858")]
	N858,
	/// Philippine Peso
	#[serde(rename = "608")]
	N608,
	/// Portuguese Escudo
	#[serde(rename = "620")]
	N620,
	/// Pound Sterling
	#[serde(rename = "826")]
	N826,
	/// Pula
	#[serde(rename = "072")]
	N072,
	/// Qatari Rial
	#[serde(rename = "634")]
	N634,
	/// Quetzal
	#[serde(rename = "320")]
	N320,
	/// Rand
	#[serde(rename = "710")]
	N710,
	/// Rial Omani
	#[serde(rename = "512")]
	N512,
	/// Riel
	#[serde(rename = "116")]
	N116,
	/// Rufiyaa
	#[serde(rename = "462")]
	N462,
	/// Rupiah
	#[serde(rename = "360")]
	N360,
	/// Russian Ruble
	#[serde(rename = "643")]
	N643,
	/// Russian Ruble
	#[serde(rename = "810")]
	N810,
	/// Rwanda Franc
	#[serde(rename = "646")]
	N646,
	/// SDR
	#[serde(rename = "960")]
	N960,
	/// Same day
	#[serde(rename = "998")]
	N998,
	/// Saudi Riyal
	#[serde(rename = "682")]
	N682,
	/// Schilling
	#[serde(rename = "040")]
	N040,
	/// Seychelles Rupee
	#[serde(rename = "690")]
	N690,
	/// Singapore Dollar
	#[serde(rename = "702")]
	N702,
	/// Slovak Koruna
	#[serde(rename = "703")]
	N703,
	/// Solomon Islands Dollar
	#[serde(rename = "090")]
	N090,
	/// Som
	#[serde(rename = "417")]
	N417,
	/// Somali Shilling
	#[serde(rename = "706")]
	N706,
	/// Spanish Peseta
	#[serde(rename = "724")]
	N724,
	/// Sri Lanka Rupee
	#[serde(rename = "144")]
	N144,
	/// St Helena Pound
	#[serde(rename = "654")]
	N654,
	/// Sucre
	#[serde(rename = "218")]
	N218,
	/// Sudanese Dinar
	#[serde(rename = "736")]
	N736,
	/// Surinam Guilder
	#[serde(rename = "740")]
	N740,
	/// Swedish Krona
	#[serde(rename = "752")]
	N752,
	/// Swiss Franc
	#[serde(rename = "756")]
	N756,
	/// Syrian Pound
	#[serde(rename = "760")]
	N760,
	/// Tajik Ruble
	#[serde(rename = "762")]
	N762,
	/// Taka
	#[serde(rename = "050")]
	N050,
	/// Tala
	#[serde(rename = "882")]
	N882,
	/// Tanzanian Shilling
	#[serde(rename = "834")]
	N834,
	/// Tenge
	#[serde(rename = "398")]
	N398,
	/// Timor Escudo
	#[serde(rename = "626")]
	N626,
	/// Tolar
	#[serde(rename = "705")]
	N705,
	/// Trinidad and Tobago Dollar
	#[serde(rename = "780")]
	N780,
	/// Tugrik
	#[serde(rename = "496")]
	N496,
	/// Tunisian Dinar
	#[serde(rename = "788")]
	N788,
	/// Turkish Lira
	#[serde(rename = "792")]
	N792,
	/// UAE Dirham
	#[serde(rename = "784")]
	N784,
	/// US Dollar
	#[serde(rename = "840")]
	N840,
	/// Uganda Shilling
	#[serde(rename = "800")]
	N800,
	/// Unidad de Valor Constante (UVC)
	#[serde(rename = "983")]
	N983,
	/// Unidades de fomento
	#[serde(rename = "990")]
	N990,
	/// Uzbekistan Sum
	#[serde(rename = "860")]
	N860,
	/// Vatu
	#[serde(rename = "548")]
	N548,
	/// Won
	#[serde(rename = "410")]
	N410,
	/// Yemeni Rial
	#[serde(rename = "886")]
	N886,
	/// Yen
	#[serde(rename = "392")]
	N392,
	/// Yuan Renminbi
	#[serde(rename = "156")]
	N156,
	/// Zimbabwe Dollar
	#[serde(rename = "716")]
	N716,
	/// Zloty
	#[serde(rename = "985")]
	N985,
	/// financial Rand
	#[serde(rename = "991")]
	N991,
	/// Gold
	#[serde(rename = "XAU")]
	Xau,
	/// European Composite Unit (EURCO)
	#[serde(rename = "XBA")]
	Xba,
	/// European Monetary Unit (E.M.U.-6)
	#[serde(rename = "XBB")]
	Xbb,
	/// European Unit of Account 9 (E.U.A.- 9)
	#[serde(rename = "XBC")]
	Xbc,
	/// European Unit of Account 17 (E.U.A.- 17)
	#[serde(rename = "XBD")]
	Xbd,
	/// Palladium
	#[serde(rename = "XPD")]
	Xpd,
	/// Platinum
	#[serde(rename = "XPT")]
	Xpt,
	/// Silver
	#[serde(rename = "XAG")]
	Xag,
	/// UIC-Franc
	#[serde(rename = "XFU")]
	Xfu,
	/// Gold-Franc
	#[serde(rename = "XFO")]
	Xfo,
	/// Codes specifically reserved for testing purposes
	#[serde(rename = "XTS")]
	Xts,
	/// Codes assigned for transactions where no currency is involved
	#[serde(rename = "XXX")]
	Xxx,
	/// Gold
	#[serde(rename = "959")]
	N959,
	/// European Composite Unit (EURCO)
	#[serde(rename = "955")]
	N955,
	/// European Monetary Unit (E.M.U.-6)
	#[serde(rename = "956")]
	N956,
	/// European Unit of Account 9 (E.U.A.- 9)
	#[serde(rename = "957")]
	N957,
	/// European Unit of Account 17 (E.U.A.- 17)
	#[serde(rename = "958")]
	N958,
	/// Palladium
	#[serde(rename = "964")]
	N964,
	/// Platinum
	#[serde(rename = "962")]
	N962,
	/// Silver
	#[serde(rename = "961")]
	N961,
	/// Codes specifically reserved for testing purposes
	#[serde(rename = "963")]
	N963,
	/// Codes assigned for transactions where no currency is involved
	#[serde(rename = "999")]
	N999,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BookingType {
	/// Regular booking
	#[serde(rename = "0")]
	RegularBooking,
	/// CFD (Contract for difference)
	#[serde(rename = "1")]
	Cfd,
	/// Total Return Swap
	#[serde(rename = "2")]
	TotalReturnSwap,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PositionEffect {
	/// Close
	#[serde(rename = "C")]
	Close,
	/// Default
	#[serde(rename = "D")]
	Default,
	/// FIFO
	#[serde(rename = "F")]
	Fifo,
	/// Close but notify on open
	#[serde(rename = "N")]
	CloseButNotifyOnOpen,
	/// Open
	#[serde(rename = "O")]
	Open,
	/// Rolled
	#[serde(rename = "R")]
	Rolled,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegalConfirm {
	/// Does not constitute a legal confirm
	#[serde(rename = "N")]
	DoesNotConstituteALegalConfirm,
	/// Legal Confirm
	#[serde(rename = "Y")]
	LegalConfirm,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LastFragment {
	/// Not Last Message
	#[serde(rename = "N")]
	NotLastMessage,
	/// Last Message
	#[serde(rename = "Y")]
	LastMessage,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AvgPxIndicator {
	/// No Average Pricing
	#[serde(rename = "0")]
	NoAveragePricing,
	/// Trade is part of an average price group identified by the AvgPxGroupID(1731)
	#[serde(rename = "1")]
	TradeIsPartOfAnAveragePriceGroupIdentifiedByTheAvgPxGroupId,
	/// Last trade of the average price group identified by the AvgPxGroupID(1731)
	#[serde(rename = "2")]
	LastTradeOfTheAveragePriceGroupIdentifiedByTheAvgPxGroupId,
	/// Trade is part of a notional value average price group
	#[serde(rename = "3")]
	TradeIsPartOfANotionalValueAveragePriceGroup,
	/// Trade is average priced
	#[serde(rename = "4")]
	TradeIsAveragePriced,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TrdType {
	/// Regular Trade
	#[serde(rename = "0")]
	N0,
	/// Block Trade
	#[serde(rename = "1")]
	N1,
	/// EFP (Exchange for physical)
	#[serde(rename = "2")]
	N2,
	/// Transfer
	#[serde(rename = "3")]
	N3,
	/// Late Trade
	#[serde(rename = "4")]
	N4,
	/// T Trade
	#[serde(rename = "5")]
	N5,
	/// Weighted Average Price Trade
	#[serde(rename = "6")]
	N6,
	/// Bunched Trade
	#[serde(rename = "7")]
	N7,
	/// Late Bunched Trade
	#[serde(rename = "8")]
	N8,
	/// Prior Reference Price Trade
	#[serde(rename = "9")]
	N9,
	/// After Hours Trade
	#[serde(rename = "10")]
	N10,
	/// Exchange for Risk (EFR)
	#[serde(rename = "11")]
	N11,
	/// Exchange for Swap (EFS )
	#[serde(rename = "12")]
	N12,
	/// Exchange of Futures for (in Market) Futures (EFM ) (e,g, full sized for mini)
	#[serde(rename = "13")]
	N13,
	/// Exchange of Options for Options (EOO)
	#[serde(rename = "14")]
	N14,
	/// Trading at Settlement
	#[serde(rename = "15")]
	N15,
	/// All or None
	#[serde(rename = "16")]
	N16,
	/// Futures Large Order Execution
	#[serde(rename = "17")]
	N17,
	/// Exchange of Futures for Futures (external market) (EFF)
	#[serde(rename = "18")]
	N18,
	/// Option Interim Trade
	#[serde(rename = "19")]
	N19,
	/// Option Cabinet Trade
	#[serde(rename = "20")]
	N20,
	/// Privately Negotiated Trades
	#[serde(rename = "22")]
	N22,
	/// Substitution of Futures for Forwards
	#[serde(rename = "23")]
	N23,
	/// Error trade
	#[serde(rename = "24")]
	N24,
	/// Special cum dividend (CD)
	#[serde(rename = "25")]
	N25,
	/// Special ex dividend (XD)
	#[serde(rename = "26")]
	N26,
	/// Special cum coupon (CC)
	#[serde(rename = "27")]
	N27,
	/// Special ex coupon (XC)
	#[serde(rename = "28")]
	N28,
	/// Cash settlement (CS)
	#[serde(rename = "29")]
	N29,
	/// Special price (usually net- or all-in price) (SP)
	#[serde(rename = "30")]
	N30,
	/// Guaranteed delivery (GD)
	#[serde(rename = "31")]
	N31,
	/// Special cum rights (CR)
	#[serde(rename = "32")]
	N32,
	/// Special ex rights (XR)
	#[serde(rename = "33")]
	N33,
	/// Special cum capital repayments (CP)
	#[serde(rename = "34")]
	N34,
	/// Special ex capital repayments (XP)
	#[serde(rename = "35")]
	N35,
	/// Special cum bonus (CB)
	#[serde(rename = "36")]
	N36,
	/// Special ex bonus (XB)
	#[serde(rename = "37")]
	N37,
	/// Block trade (same as large trade)
	#[serde(rename = "38")]
	N38,
	/// Worked principal trade (UK-specific)
	#[serde(rename = "39")]
	N39,
	/// Block Trades - after market
	#[serde(rename = "40")]
	N40,
	/// Name change
	#[serde(rename = "41")]
	N41,
	/// Portfolio transfer
	#[serde(rename = "42")]
	N42,
	/// Prorogation buy - Euronext Paris only. Is used to defer settlement under French SRD (deferred settlement system) . Trades
	/// must be reported as crosses at zero price
	#[serde(rename = "43")]
	N43,
	/// Prorogation sell - see prorogation buy
	#[serde(rename = "44")]
	N44,
	/// Option exercise
	#[serde(rename = "45")]
	N45,
	/// Delta neutral transaction
	#[serde(rename = "46")]
	N46,
	/// Financing transaction (includes repo and stock lending)
	#[serde(rename = "47")]
	N47,
	/// Non-standard settlement
	#[serde(rename = "48")]
	N48,
	/// Derivative related transaction
	#[serde(rename = "49")]
	N49,
	/// Portfolio trade
	#[serde(rename = "50")]
	N50,
	/// Volume weighted average trade
	#[serde(rename = "51")]
	N51,
	/// Exchange granted trade
	#[serde(rename = "52")]
	N52,
	/// Repurchase agreement
	#[serde(rename = "53")]
	N53,
	/// OTC
	#[serde(rename = "54")]
	N54,
	/// Exchange absis facility (EBF)
	#[serde(rename = "55")]
	N55,
	/// Opening Trade
	#[serde(rename = "56")]
	N56,
	/// Netted trade
	#[serde(rename = "57")]
	N57,
	/// Block swap trade or large notional off-facility swap
	#[serde(rename = "58")]
	N58,
	/// Give-up/Give-in trade
	#[serde(rename = "61")]
	N61,
	/// Dark trade (Elaboration: A Market Model Typology dark trade might also come from a lit/hybrid book, when an aggressive lit
	/// order hits a resting dark order.)
	#[serde(rename = "62")]
	N62,
	/// Technical trade
	#[serde(rename = "63")]
	N63,
	/// Benchmark
	#[serde(rename = "64")]
	N64,
	/// Credit event trade
	#[serde(rename = "59")]
	N59,
	/// Succession event trade
	#[serde(rename = "60")]
	N60,
	/// Package Trade (Identifies the pseudotrade of a stream or collection of trades to be cleared and be reported as an atomic unit.
	/// The subsequent actual trades reported should not have this value)
	#[serde(rename = "65")]
	N65,
	/// Roll trade
	#[serde(rename = "66")]
	N66,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TrdSubType {
	/// CMTA
	#[serde(rename = "0")]
	Cmta,
	/// Internal transfer or adjustment
	#[serde(rename = "1")]
	InternalTransferOrAdjustment,
	/// External transfer or transfer of account
	#[serde(rename = "2")]
	ExternalTransferOrTransferOfAccount,
	/// Reject for submitting side
	#[serde(rename = "3")]
	RejectForSubmittingSide,
	/// Advisory for contra side
	#[serde(rename = "4")]
	AdvisoryForContraSide,
	/// Offset due to an allocation
	#[serde(rename = "5")]
	OffsetDueToAnAllocation,
	/// Onset due to an allocation
	#[serde(rename = "6")]
	OnsetDueToAnAllocation,
	/// Differential Spread
	#[serde(rename = "7")]
	DifferentialSpread,
	/// Implied Spread leg executed against an outright
	#[serde(rename = "8")]
	ImpliedSpreadLegExecutedAgainstAnOutright,
	/// Transaction from exercise
	#[serde(rename = "9")]
	TransactionFromExercise,
	/// Transaction from assignment
	#[serde(rename = "10")]
	TransactionFromAssignment,
	/// ACATS
	#[serde(rename = "11")]
	Acats,
	/// AI (Automated input facility disabled in response to an exchange request.)
	#[serde(rename = "14")]
	Ai,
	/// B (Transaction between two member firms where neither member firm is registered as a market maker in the security in question
	/// and neither is a designated fund manager. Also used by broker dealers when dealing with another broker which is not a member
	/// firm. Non-order book securities only.)
	#[serde(rename = "15")]
	B,
	/// K (Transaction using block trade facility.)
	#[serde(rename = "16")]
	K,
	/// LC (Correction submitted more than three days after publication of the original trade report.)
	#[serde(rename = "17")]
	Lc,
	/// M (Transaction, other than a transaction resulting from a stock swap or stock switch, between two market makers registered
	/// in that security including IDB or a public display system trades. Non-order book securities only.)
	#[serde(rename = "18")]
	M,
	/// N (Non-protected portfolio transaction or a fully disclosed portfolio transaction)
	#[serde(rename = "19")]
	N,
	/// NM (i) transaction where Exchange has granted permission for non-publication (ii) IDB is reporting as seller (iii) submitting
	/// a transaction report to the Exchange, where the transaction report is not also a trade report.
	#[serde(rename = "20")]
	NmTransactionWhereExchangeHasGrantedPermissionForNonPublicationIdbIsReportingAsSellerSubmittingATransactionReportToTheExchangeWhereTheTransactionReportIsNotAlsoATradeReport,
	/// NR (Non-risk transaction in a SEATS security other than an AIM security)
	#[serde(rename = "21")]
	Nr,
	/// P (Protected portfolio transaction or a worked principal agreement to effect a portfolio transaction which includes order
	/// book securities)
	#[serde(rename = "22")]
	P,
	/// PA (Protected transaction notification)
	#[serde(rename = "23")]
	Pa,
	/// PC (Contra trade for transaction which took place on a previous day and which was automatically executed on the Exchange trading
	/// system)
	#[serde(rename = "24")]
	Pc,
	/// PN (Worked principal notification for a portfolio transaction which includes order book securities)
	#[serde(rename = "25")]
	Pn,
	/// R ((i) riskless principal transaction between non-members where the buying and selling transactions are executed at different
	/// prices or on different terms (requires a trade report with trade type indicator R for each transaction) (ii) market maker
	/// is reporting all the legs of a riskless principal transaction where the buying and selling transactions are executed at different
	/// prices (requires a trade report with trade type indicator R for each transaction)or (iii) market maker is reporting the onward
	/// leg of a riskless principal transaction where the legs are executed at different prices, and another market maker has submitted
	/// a trade report using trade type indicator M for the first leg (this requires a single trade report with trade type indicator
	/// R))
	#[serde(rename = "26")]
	RRisklessPrincipalTransactionBetweenNonMembersWhereTheBuyingAndSellingTransactionsAreExecutedAtDifferentPricesOrOnDifferentTermsMarketMakerIsReportingAllTheLegsOfARisklessPrincipalTransactionWhereTheBuyingAndSellingTransactionsAreExecutedAtDifferentPricesOrMarketMakerIsReportingTheOnwardLegOfARisklessPrincipalTransactionWhereTheLegsAreExecutedAtDifferentPricesAndAnotherMarketMakerHasSubmittedATradeReportUsingTradeTypeIndicatorMForTheFirstLeg,
	/// RO (Transaction which resulted from the exercise of a traditional option or a stock-settled covered warrant)
	#[serde(rename = "27")]
	Ro,
	/// RT (Risk transaction in a SEATS security, (excluding AIM security) reported by a market maker registered in that security)
	#[serde(rename = "28")]
	RtReportedByAMarketMakerRegisteredInThatSecurity,
	/// SW (Transactions resulting from stock swap or a stock switch (one report is required for each line of stock))
	#[serde(rename = "29")]
	Sw,
	/// T (If reporting a single protected transaction)
	#[serde(rename = "30")]
	T,
	/// WN (Worked principal notification for a single order book security)
	#[serde(rename = "31")]
	Wn,
	/// WT (Worked principal transaction (other than a portfolio transaction))
	#[serde(rename = "32")]
	Wt,
	/// Off Hours Trade
	#[serde(rename = "33")]
	OffHoursTrade,
	/// On Hours Trade
	#[serde(rename = "34")]
	OnHoursTrade,
	/// OTC Quote
	#[serde(rename = "35")]
	OtcQuote,
	/// Converted SWAP
	#[serde(rename = "36")]
	ConvertedSwap,
	/// Crossed Trade (X)
	#[serde(rename = "37")]
	CrossedTrade,
	/// Interim Protected Trade (I)
	#[serde(rename = "38")]
	InterimProtectedTrade,
	/// Large in Scale (L)
	#[serde(rename = "39")]
	LargeInScale,
	/// Wash Trade
	#[serde(rename = "40")]
	WashTrade,
	/// Trade at Settlement (TAS)
	#[serde(rename = "41")]
	TradeAtSettlement,
	/// Trade at Marker (TAM)
	#[serde(rename = "43")]
	TradeAtMarker,
	/// AuctionTrade
	#[serde(rename = "42")]
	AuctionTrade,
	/// Default (credit event)
	#[serde(rename = "44")]
	Default,
	/// Restructuring (credit event)
	#[serde(rename = "45")]
	Restructuring,
	/// Merger (succession event)
	#[serde(rename = "46")]
	Merger,
	/// Spin-off (succession event)
	#[serde(rename = "47")]
	SpinOff,
	/// Multilateral compression
	#[serde(rename = "48")]
	MultilateralCompression,
	/// Balancing
	#[serde(rename = "50")]
	Balancing,
	/// Basis Trade index Close (BTIC)
	#[serde(rename = "51")]
	BasisTradeIndexClose,
	/// Trade At Cash Open (TACO). The marketplace name given to trading futures based on an opening quote of the underlying cash
	/// market
	#[serde(rename = "52")]
	TradeAtCashOpenTheMarketplaceNameGivenToTradingFuturesBasedOnAnOpeningQuoteOfTheUnderlyingCashMarket,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CustOrderCapacity {
	/// Member trading for their own account
	#[serde(rename = "1")]
	MemberTradingForTheirOwnAccount,
	/// Clearing firm trading for its proprietary account
	#[serde(rename = "2")]
	ClearingFirmTradingForItsProprietaryAccount,
	/// Member trading for another member
	#[serde(rename = "3")]
	MemberTradingForAnotherMember,
	/// All other
	#[serde(rename = "4")]
	AllOther,
	/// Retail customer
	#[serde(rename = "5")]
	RetailCustomer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MultiLegReportingType {
	/// Single Security (default if not specified)
	#[serde(rename = "1")]
	SingleSecurity,
	/// Individual leg of a multi-leg security
	#[serde(rename = "2")]
	IndividualLegOfAMultiLegSecurity,
	/// Multi-leg security
	#[serde(rename = "3")]
	MultiLegSecurity,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CustOrderHandlingInst {
	/// Add-on Order
	#[serde(rename = "ADD")]
	AddOnOrder,
	/// All or None
	#[serde(rename = "AON")]
	AllOrNone,
	/// Cash Not Held
	#[serde(rename = "CNH")]
	CashNotHeld,
	/// Directed Order
	#[serde(rename = "DIR")]
	DirectedOrder,
	/// Exchange for Physical Transaction
	#[serde(rename = "E.W")]
	ExchangeForPhysicalTransaction,
	/// Fill or Kill
	#[serde(rename = "FOK")]
	FillOrKill,
	/// Imbalance Only
	#[serde(rename = "IO")]
	ImbalanceOnly,
	/// Immediate or Cancel
	#[serde(rename = "IOC")]
	ImmediateOrCancel,
	/// Limit On Open
	#[serde(rename = "LOO")]
	LimitOnOpen,
	/// Limit on Close
	#[serde(rename = "LOC")]
	LimitOnClose,
	/// Market at Open
	#[serde(rename = "MAO")]
	MarketAtOpen,
	/// Market at Close
	#[serde(rename = "MAC")]
	MarketAtClose,
	/// Market on Open
	#[serde(rename = "MOO")]
	MarketOnOpen,
	/// Market On Close
	#[serde(rename = "MOC")]
	MarketOnClose,
	/// Minimum Quantity
	#[serde(rename = "MQT")]
	MinimumQuantity,
	/// Not Held
	#[serde(rename = "NH")]
	NotHeld,
	/// Over the Day
	#[serde(rename = "OVD")]
	OverTheDay,
	/// Pegged
	#[serde(rename = "PEG")]
	Pegged,
	/// Reserve Size Order
	#[serde(rename = "RSV")]
	ReserveSizeOrder,
	/// Stop Stock Transaction
	#[serde(rename = "S.W")]
	StopStockTransaction,
	/// Scale
	#[serde(rename = "SCL")]
	Scale,
	/// Time Order
	#[serde(rename = "TMO")]
	TimeOrder,
	/// Trailing Stop
	#[serde(rename = "TS")]
	TrailingStop,
	/// Work
	#[serde(rename = "WRK")]
	Work,
	/// Phone simple
	#[serde(rename = "A")]
	PhoneSimple,
	/// Phone complex
	#[serde(rename = "B")]
	PhoneComplex,
	/// FCM provided screen
	#[serde(rename = "C")]
	FcmProvidedScreen,
	/// Other provided screen
	#[serde(rename = "D")]
	OtherProvidedScreen,
	/// Client provided platform controlled by FCM
	#[serde(rename = "E")]
	ClientProvidedPlatformControlledByFcm,
	/// Client provided platform direct to exchange
	#[serde(rename = "F")]
	ClientProvidedPlatformDirectToExchange,
	/// FCM API or FIX
	#[serde(rename = "G")]
	FcmApiOrFix,
	/// Algo engine
	#[serde(rename = "H")]
	AlgoEngine,
	/// Price at execution(price added at initial order entry, trading, middle office or time of give - up)
	#[serde(rename = "J")]
	PriceAtExecution,
	/// Desk - electronic
	#[serde(rename = "W")]
	DeskElectronic,
	/// Desk - pit
	#[serde(rename = "X")]
	DeskPit,
	/// Client - electronic
	#[serde(rename = "Y")]
	ClientElectronic,
	/// Client - pit
	#[serde(rename = "Z")]
	ClientPit,
	/// Conditional order
	#[serde(rename = "CND")]
	ConditionalOrder,
	/// Deliver instructions-cash
	#[serde(rename = "CSH")]
	DeliverInstructionsCash,
	/// Discretionary limit order
	#[serde(rename = "DLO")]
	DiscretionaryLimitOrder,
	/// Intra-day cross
	#[serde(rename = "IDX")]
	IntraDayCross,
	/// Intermarket sweep order
	#[serde(rename = "ISO")]
	IntermarketSweepOrder,
	/// Merger related transfer position
	#[serde(rename = "MPT")]
	MergerRelatedTransferPosition,
	/// Market to limit
	#[serde(rename = "MTL")]
	MarketToLimit,
	/// Delivery instructions - next day
	#[serde(rename = "ND")]
	DeliveryInstructionsNextDay,
	/// Options related transaction
	#[serde(rename = "OPT")]
	OptionsRelatedTransaction,
	/// Delivery instructions - seller's option
	#[serde(rename = "SLR")]
	DeliveryInstructionsSellerSOption,
	/// Stay on offerside
	#[serde(rename = "F0")]
	StayOnOfferside,
	/// Go along
	#[serde(rename = "F3")]
	GoAlong,
	/// Participate do not initiate
	#[serde(rename = "F6")]
	ParticipateDoNotInitiate,
	/// Strict scale
	#[serde(rename = "F7")]
	StrictScale,
	/// Try to scale
	#[serde(rename = "F8")]
	TryToScale,
	/// Stay on bidside
	#[serde(rename = "F9")]
	StayOnBidside,
	/// No Cross
	#[serde(rename = "FA")]
	NoCross,
	/// OK to Cross
	#[serde(rename = "FB")]
	OkToCross,
	/// Call first
	#[serde(rename = "FC")]
	CallFirst,
	/// Percent of volume
	#[serde(rename = "FD")]
	PercentOfVolume,
	/// Reinstate on system failure
	#[serde(rename = "FH")]
	ReinstateOnSystemFailure,
	/// Institution only
	#[serde(rename = "FI")]
	InstitutionOnly,
	/// Reinstate on trading halt
	#[serde(rename = "FJ")]
	ReinstateOnTradingHalt,
	/// Cancel on trading halt
	#[serde(rename = "FK")]
	CancelOnTradingHalt,
	/// Last peg
	#[serde(rename = "FL")]
	LastPeg,
	/// Mid-price peg
	#[serde(rename = "FM")]
	MidPricePeg,
	/// Non - negotiable
	#[serde(rename = "FN")]
	NonNegotiable,
	/// Opening peg
	#[serde(rename = "FO")]
	OpeningPeg,
	/// Market peg
	#[serde(rename = "FP")]
	MarketPeg,
	/// Cancel on system failure
	#[serde(rename = "FQ")]
	CancelOnSystemFailure,
	/// Primary peg
	#[serde(rename = "FR")]
	PrimaryPeg,
	/// Suspend
	#[serde(rename = "FS")]
	Suspend,
	/// Fixed peg to local best bid or offer at time of order
	#[serde(rename = "FT")]
	FixedPegToLocalBestBidOrOfferAtTimeOfOrder,
	/// Peg to VWAP
	#[serde(rename = "FW")]
	PegToVwap,
	/// Trade along
	#[serde(rename = "FX")]
	TradeAlong,
	/// Try to stop
	#[serde(rename = "FY")]
	TryToStop,
	/// Cancel if not best
	#[serde(rename = "FZ")]
	CancelIfNotBest,
	/// Strict limit
	#[serde(rename = "Fb")]
	StrictLimit,
	/// Ignore price validity checks
	#[serde(rename = "Fc")]
	IgnorePriceValidityChecks,
	/// Peg to limit price
	#[serde(rename = "Fd")]
	PegToLimitPrice,
	/// Work to target strategy
	#[serde(rename = "Fe")]
	WorkToTargetStrategy,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OrderHandlingInstSource {
	/// NASD OATS
	#[serde(rename = "1")]
	NasdOats,
	/// FIA Execution Source Code
	#[serde(rename = "2")]
	FiaExecutionSourceCode,
}
