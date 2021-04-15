
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExecutionReport {
	/// MsgType = 8
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'8', ' '>,
	/// For use in drop copy applications. NOT FOR USE in transactional applications.
	#[serde(flatten)]
	pub application_sequence_control: Option<super::super::application_sequence_control::ApplicationSequenceControl>,
	/// OrderID is required to be unique for each chain of orders.
	#[serde(rename = "37")]
	pub order_id: String,
	/// Can be used to provide order id used by exchange or executing system.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "198")]
	pub secondary_order_id: Option<String>,
	/// In the case of quotes can be mapped to: <a href="tag_117_QuoteID.html" target="bottom">QuoteID&nbsp;(117)</a> of a single Quote, <a href="tag_299_QuoteEntryID.html" target="bottom">QuoteEntryID&nbsp;(299)</a> of a Mass Quote.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "526")]
	pub secondary_cl_ord_id: Option<String>,
	/// SecondaryExecID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "527")]
	pub secondary_exec_id: Option<String>,
	/// Required when referring to orders that where electronically submitted over FIX or otherwise assigned a <a href="tag_11_ClOrdID.html" target="bottom">ClOrdID&nbsp;(11)</a> . In the case of quotes can be mapped to: <a href="tag_117_QuoteID.html" target="bottom">QuoteID&nbsp;(117)</a> of a single Quote, <a href="tag_299_QuoteEntryID.html" target="bottom">QuoteEntryID&nbsp;(299)</a> of a Mass Quote.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "11")]
	pub cl_ord_id: Option<String>,
	/// Conditionally required for response to an electronic Cancel or Cancel/Replace request (ExecType=PendingCancel, Replace, or
	/// Canceled). <a href="tag_11_ClOrdID.html" target="bottom">ClOrdID&nbsp;(11)</a> of the previous accepted order (NOT the initial order of the day) when canceling or replacing an order.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41")]
	pub orig_cl_ord_id: Option<String>,
	/// ClOrdLinkID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "583")]
	pub cl_ord_link_id: Option<String>,
	/// Required if responding to a QuoteResponse message. Echo back the Initiators value specified in the message.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "693")]
	pub quote_resp_id: Option<String>,
	/// Required if responding to and if provided on the <a href="message_Order_Status_Request_H.html" target="main">Order Status Request&nbsp;(H)</a> message. Echo back the value provided by the requester.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "790")]
	pub ord_status_req_id: Option<String>,
	/// Required if responding to a Order Mass Status Request. Echo back the value provided by the requester.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "584")]
	pub mass_status_req_id: Option<String>,
	/// Host assigned entity ID that can be used to reference all components of a cross; sides + strategy + legs.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "961")]
	pub host_cross_id: Option<String>,
	/// Can be used when responding to an <a href="message_Order_Mass_Status_Request_AF.html" target="main">Order Mass Status Request&nbsp;(AF)</a> to identify the total number of <a href="message_Execution_Report_8.html" target="main">Execution Reports&nbsp;(8)</a> which will be returned.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "911")]
	pub tot_num_reports: Option<i32>,
	/// Can be used when responding to an <a href="message_Order_Mass_Status_Request_AF.html" target="main">Order Mass Status Request&nbsp;(AF)</a> to indicate that this is the last <a href="message_Execution_Report_8.html" target="main">Execution Reports&nbsp;(8)</a> which will be returned as a result of the request.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "912")]
	pub last_rpt_requested: Option<LastRptRequested>,
	/// Insert here the set of "Parties" (firm identification) fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// TradeOriginationDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "229")]
	pub trade_origination_date: Option<fix_common::LocalMktDate>,
	/// Number of ContraBrokers repeating group instances.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "382")]
	pub contra_brokers: Option<fix_common::RepeatingValues<ContraBroker>>,
	/// Required for executions against orders which were submitted as part of a list.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "66")]
	pub list_id: Option<String>,
	/// CrossID for the replacement order
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "548")]
	pub cross_id: Option<String>,
	/// Must match original cross order. Same order chaining mechanism as ClOrdID/OrigClOrdID with single order Cancel/Replace.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "551")]
	pub orig_cross_id: Option<String>,
	/// CrossType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "549")]
	pub cross_type: Option<CrossType>,
	/// TrdMatchID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "880")]
	pub trd_match_id: Option<String>,
	/// Unique identifier of execution message as assigned by sell-side (broker, exchange, ECN) (will be 0 (zero) for ExecType=I (Order
	/// Status)).
	#[serde(rename = "17")]
	pub exec_id: String,
	/// Required for Trade Cancel and Trade Correct <a href="tag_150_ExecType.html" target="bottom">ExecType&nbsp;(150)</a> messages
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "19")]
	pub exec_ref_id: Option<String>,
	/// Describes the purpose of the execution report.
	#[serde(rename = "150")]
	pub exec_type: ExecType,
	/// Describes the current state of a CHAIN of orders, same scope as <a href="tag_38_OrderQty.html" target="bottom">OrderQty&nbsp;(38)</a> , <a href="tag_14_CumQty.html" target="bottom">CumQty&nbsp;(14)</a> , <a href="tag_151_LeavesQty.html" target="bottom">LeavesQty&nbsp;(151)</a> , and <a href="tag_6_AvgPx.html" target="bottom">AvgPx&nbsp;(6)</a>
	#[serde(rename = "39")]
	pub ord_status: OrdStatus,
	/// For optional use with <a href="tag_39_OrdStatus.html" target="bottom">OrdStatus&nbsp;(39)</a> = 0 (New)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "636")]
	pub working_indicator: Option<WorkingIndicator>,
	/// For optional use with <a href="tag_150_ExecType.html" target="bottom">ExecType&nbsp;(150)</a> = 8 (Rejected)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "103")]
	pub ord_rej_reason: Option<OrdRejReason>,
	/// Required for <a href="tag_150_ExecType.html" target="bottom">ExecType&nbsp;(150)</a> = D (Restated).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "378")]
	pub exec_restatement_reason: Option<ExecRestatementReason>,
	/// Required for executions against electronically submitted orders which were assigned an account by the institution or intermediary
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1")]
	pub account: Option<String>,
	/// AcctIDSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "660")]
	pub acct_id_source: Option<AcctIDSource>,
	/// Specifies type of account
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "581")]
	pub account_type: Option<AccountType>,
	/// DayBookingInst
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "589")]
	pub day_booking_inst: Option<DayBookingInst>,
	/// BookingUnit
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "590")]
	pub booking_unit: Option<BookingUnit>,
	/// PreallocMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "591")]
	pub prealloc_method: Option<PreallocMethod>,
	/// AllocID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "70")]
	pub alloc_id: Option<String>,
	/// Number of repeating groups for pre-trade allocation
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "78")]
	pub allocs: Option<fix_common::RepeatingValues<Alloc>>,
	/// SettlType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "63")]
	pub settl_type: Option<SettlType>,
	/// Takes precedence over <a href="tag_63_SettlType.html" target="bottom">SettlType&nbsp;(63)</a> value and conditionally required/omitted for specific <a href="tag_63_SettlType.html" target="bottom">SettlType&nbsp;(63)</a> values.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "64")]
	pub settl_date: Option<fix_common::LocalMktDate>,
	/// MatchType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "574")]
	pub match_type: Option<MatchType>,
	/// OrderCategory
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1115")]
	pub order_category: Option<OrderCategory>,
	/// CashMargin
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "544")]
	pub cash_margin: Option<CashMargin>,
	/// ClearingFeeIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "635")]
	pub clearing_fee_indicator: Option<ClearingFeeIndicator>,
	/// Insert here the set of "Instrument" fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub instrument: super::super::instrument::Instrument,
	/// Insert here the set of "FinancingDetails" fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub financing_details: Option<super::super::financing_details::FinancingDetails>,
	/// Number of underlyings
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "711")]
	pub underlyings: Option<fix_common::RepeatingValues<Underlying>>,
	/// Side
	#[serde(rename = "54")]
	pub side: Side,
	/// Insert here the set of "Stipulations" (repeating group of Fixed Income stipulations) fields defined in "Common Components
	/// of Application Messages".
	#[serde(flatten)]
	pub stipulations: Option<super::super::stipulations::Stipulations>,
	/// QtyType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "854")]
	pub qty_type: Option<QtyType>,
	/// Insert here the set of "OrderQtyData" fields defined in "Common Components of Application Messages". **IMPORTANT NOTE: <a href="tag_38_OrderQty.html" target="bottom">OrderQty&nbsp;(38)</a> field is required for Single <a href="block_Instrument.html" target="main">Instrument</a> Orders unless rejecting or acknowledging an order for a <a href="tag_152_CashOrderQty.html" target="bottom">CashOrderQty&nbsp;(152)</a> or PercentOrder. **
	#[serde(flatten)]
	pub order_qty_data: Option<super::super::order_qty_data::OrderQtyData>,
	/// LotType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1093")]
	pub lot_type: Option<LotType>,
	/// OrdType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40")]
	pub ord_type: Option<OrdType>,
	/// PriceType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "423")]
	pub price_type: Option<PriceType>,
	/// Required if specified on the order
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "44")]
	pub price: Option<f64>,
	/// PriceProtectionScope
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1092")]
	pub price_protection_scope: Option<PriceProtectionScope>,
	/// Required if specified on the order
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "99")]
	pub stop_px: Option<f64>,
	/// Insert here the set of "TriggeringInstruction" fields defined in "common components of application messages".
	#[serde(flatten)]
	pub triggering_instruction: Option<super::super::triggering_instruction::TriggeringInstruction>,
	/// Insert here the set of "PegInstruction" fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub peg_instructions: Option<super::super::peg_instructions::PegInstructions>,
	/// Insert here the set of "DiscretionInstruction" fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub discretion_instructions: Option<super::super::discretion_instructions::DiscretionInstructions>,
	/// The current price the order is pegged at
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "839")]
	pub pegged_price: Option<f64>,
	/// The reference price of a pegged order.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1095")]
	pub pegged_ref_price: Option<f64>,
	/// The current discretionary price of the order
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "845")]
	pub discretion_price: Option<f64>,
	/// The target strategy of the order
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "847")]
	pub target_strategy: Option<TargetStrategy>,
	/// Indicates number of strategy parameters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "957")]
	pub strategy_parameters: Option<fix_common::RepeatingValues<StrategyParameter>>,
	/// (Deprecated in FIX.5.0)For further specification of the TargetStrategy
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "848")]
	pub target_strategy_parameters: Option<String>,
	/// (Deprecated in FIX.5.0) Mandatory for a TargetStrategy=Participate order and specifies the target particpation rate. For other
	/// order types optionally specifies a volume limit (i.e. do not be more than this percent of the market volume)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "849")]
	pub participation_rate: Option<f32>,
	/// For communication of the performance of the order versus the target strategy
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "850")]
	pub target_strategy_performance: Option<f64>,
	/// Currency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "15")]
	pub currency: Option<Currency>,
	/// ComplianceID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "376")]
	pub compliance_id: Option<String>,
	/// SolicitedFlag
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "377")]
	pub solicited_flag: Option<SolicitedFlag>,
	/// Absence of this field indicates Day order
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "59")]
	pub time_in_force: Option<TimeInForce>,
	/// Time specified on the order at which the order should be considered valid
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "168")]
	pub effective_time: Option<fix_common::UTCTimestamp>,
	/// Conditionally required if <a href="tag_59_TimeInForce.html" target="bottom">TimeInForce&nbsp;(59)</a> = GTD and <a href="tag_126_ExpireTime.html" target="bottom">ExpireTime&nbsp;(126)</a> is not specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "432")]
	pub expire_date: Option<fix_common::LocalMktDate>,
	/// Conditionally required if <a href="tag_59_TimeInForce.html" target="bottom">TimeInForce&nbsp;(59)</a> = GTD and <a href="tag_432_ExpireDate.html" target="bottom">ExpireDate&nbsp;(432)</a> is not specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "126")]
	pub expire_time: Option<fix_common::UTCTimestamp>,
	/// Can contain multiple instructions, space delimited.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "18")]
	pub exec_inst: Option<fix_common::SeparatedValues<ExecInst>>,
	/// AggressorIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1057")]
	pub aggressor_indicator: Option<AggressorIndicator>,
	/// OrderCapacity
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "528")]
	pub order_capacity: Option<OrderCapacity>,
	/// OrderRestrictions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "529")]
	pub order_restrictions: Option<fix_common::SeparatedValues<OrderRestrictions>>,
	/// PreTradeAnonymity
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1091")]
	pub pre_trade_anonymity: Option<fix_common::Boolean>,
	/// CustOrderCapacity
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "582")]
	pub cust_order_capacity: Option<CustOrderCapacity>,
	/// Quantity (e.g. shares) bought/sold on this (last) fill. Required if <a href="tag_150_ExecType.html" target="bottom">ExecType&nbsp;(150)</a> = Trade or Trade Correct. If ExecType=Stopped, represents the quantity stopped/guaranteed/protected for.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "32")]
	pub last_qty: Option<f64>,
	/// Used for FX trades to express the quantity or amount of the other side of the currency. Conditionally required if <a href="tag_150_ExecType.html" target="bottom">ExecType&nbsp;(150)</a> = Trade or Trade Correct and is an FX trade.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1056")]
	pub calculated_ccy_last_qty: Option<f64>,
	/// Optionally used when <a href="tag_150_ExecType.html" target="bottom">ExecType&nbsp;(150)</a> = Trade or Trade Correct and is a FX Swap trade. Used to express the swap points for the swap trade event.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1071")]
	pub last_swap_points: Option<f64>,
	/// UnderlyingLastQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "652")]
	pub underlying_last_qty: Option<f64>,
	/// Price of this (last) fill. Required if <a href="tag_150_ExecType.html" target="bottom">ExecType&nbsp;(150)</a> = Trade or Trade Correct. Should represent the "all-in" (LastSpotRate + LastForwardPoints) rate for F/X orders. If ExecType=Stopped,
	/// represents the price stopped/guaranteed/protected at. Not required for FX Swap when <a href="tag_150_ExecType.html" target="bottom">ExecType&nbsp;(150)</a> = Trade or Trade Correct as there is no "all-in" rate that applies to both legs of the FX Swap.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "31")]
	pub last_px: Option<f64>,
	/// UnderlyingLastPx
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "651")]
	pub underlying_last_px: Option<f64>,
	/// Last price expressed in percent-of-par. Conditionally required for Fixed Income trades when <a href="tag_31_LastPx.html" target="bottom">LastPx&nbsp;(31)</a> is expressed in Yield, Spread, Discount or any other price type that is not percent-of-par.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "669")]
	pub last_par_px: Option<f64>,
	/// Applicable for F/X orders
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "194")]
	pub last_spot_rate: Option<f64>,
	/// Applicable for F/X orders
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "195")]
	pub last_forward_points: Option<f64>,
	/// If <a href="tag_150_ExecType.html" target="bottom">ExecType&nbsp;(150)</a> = Trade (F), indicates the market where the trade was executed. If <a href="tag_150_ExecType.html" target="bottom">ExecType&nbsp;(150)</a> = New (0), indicates the market where the order was routed.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "30")]
	pub last_mkt: Option<String>,
	/// TradingSessionID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "336")]
	pub trading_session_id: Option<TradingSessionID>,
	/// TradingSessionSubID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "625")]
	pub trading_session_sub_id: Option<TradingSessionSubID>,
	/// TimeBracket
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "943")]
	pub time_bracket: Option<String>,
	/// LastCapacity
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "29")]
	pub last_capacity: Option<LastCapacity>,
	/// Quantity open for further execution. If the <a href="tag_39_OrdStatus.html" target="bottom">OrdStatus&nbsp;(39)</a> is Canceled, DoneForTheDay, Expired, Calculated, or Rejected (in which case the order is no longer active) then <a href="tag_151_LeavesQty.html" target="bottom">LeavesQty&nbsp;(151)</a> could be 0, otherwise <a href="tag_151_LeavesQty.html" target="bottom">LeavesQty&nbsp;(151)</a> = <a href="tag_38_OrderQty.html" target="bottom">OrderQty&nbsp;(38)</a> - CumQty.
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "151")]
	pub leaves_qty: f64,
	/// Currently executed quantity for chain of orders.
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "14")]
	pub cum_qty: f64,
	/// Not required for markets where average price is not calculated by the market. Conditionally required otherwise.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "6")]
	pub avg_px: Option<f64>,
	/// For GT orders on days following the day of the first trade.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "424")]
	pub day_order_qty: Option<f64>,
	/// For GT orders on days following the day of the first trade.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "425")]
	pub day_cum_qty: Option<f64>,
	/// For GT orders on days following the day of the first trade.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "426")]
	pub day_avg_px: Option<f64>,
	/// Used to support fragmentation. Sum of NoFills across all messages with the same ExecID.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1361")]
	pub tot_no_fills: Option<i32>,
	/// Indicates whether this is the last fragment in a sequence of message fragments. Only required where message has been fragmented.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "893")]
	pub last_fragment: Option<LastFragment>,
	/// Specifies the number of partial fills included in this Execution Report
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1362")]
	pub fills: Option<fix_common::RepeatingValues<Fill>>,
	/// States whether executions are booked out or accumulated on a partially filled GT order
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "427")]
	pub gt_booking_inst: Option<GTBookingInst>,
	/// Used when reporting other than current day trades.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "75")]
	pub trade_date: Option<fix_common::LocalMktDate>,
	/// Time the transaction represented by this ExecutionReport occurred
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<fix_common::UTCTimestamp>,
	/// ReportToExch
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "113")]
	pub report_to_exch: Option<ReportToExch>,
	/// Insert here the set of "CommissionData" fields defined in "Common Components of Application Messages". Note: On a fill/partial
	/// fill messages, it represents value for that fill/partial fill. On ExecType=Calculated, it represents cumulative value for
	/// the order. Monetary commission values are expressed in the currency reflected by the <a href="tag_15_Currency.html" target="bottom">Currency&nbsp;(15)</a> field.
	#[serde(flatten)]
	pub commission_data: Option<super::super::commission_data::CommissionData>,
	/// Insert here the set of "SpreadOrBenchmarkCurveData" (Fixed Income spread or benchmark curve) fields defined in "Common Components
	/// of Application Messages".
	#[serde(flatten)]
	pub spread_or_benchmark_curve_data: Option<super::super::spread_or_benchmark_curve_data::SpreadOrBenchmarkCurveData>,
	/// Insert here the set of "YieldData" (yield-related) fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub yield_data: Option<super::super::yield_data::YieldData>,
	/// GrossTradeAmt
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "381")]
	pub gross_trade_amt: Option<f64>,
	/// NumDaysInterest
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "157")]
	pub num_days_interest: Option<i32>,
	/// ExDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "230")]
	pub ex_date: Option<fix_common::LocalMktDate>,
	/// AccruedInterestRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "158")]
	pub accrued_interest_rate: Option<f32>,
	/// AccruedInterestAmt
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "159")]
	pub accrued_interest_amt: Option<f64>,
	/// For fixed income products which pay lump-sum interest at maturity.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "738")]
	pub interest_at_maturity: Option<f64>,
	/// For repurchase agreements the accrued interest on termination.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "920")]
	pub end_accrued_interest_amt: Option<f64>,
	/// For repurchase agreements the start (dirty) cash consideration
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "921")]
	pub start_cash: Option<f64>,
	/// For repurchase agreements the end (dirty) cash consideration
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "922")]
	pub end_cash: Option<f64>,
	/// TradedFlatSwitch
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "258")]
	pub traded_flat_switch: Option<TradedFlatSwitch>,
	/// BasisFeatureDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "259")]
	pub basis_feature_date: Option<fix_common::LocalMktDate>,
	/// BasisFeaturePrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "260")]
	pub basis_feature_price: Option<f64>,
	/// Concession
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "238")]
	pub concession: Option<f64>,
	/// TotalTakedown
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "237")]
	pub total_takedown: Option<f64>,
	/// Note: On a fill/partial fill messages, it represents value for that fill/partial fill, on ExecType=Calculated, it represents
	/// cumulative value for the order. Value expressed in the currency reflected by the <a href="tag_15_Currency.html" target="bottom">Currency&nbsp;(15)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "118")]
	pub net_money: Option<f64>,
	/// Used to report results of forex accommodation trade
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "119")]
	pub settl_curr_amt: Option<f64>,
	/// Used to report results of forex accommodation trade
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "120")]
	pub settl_currency: Option<SettlCurrency>,
	/// Foreign exchange rate used to compute <a href="tag_119_SettlCurrAmt.html" target="bottom">SettlCurrAmt&nbsp;(119)</a> from <a href="tag_15_Currency.html" target="bottom">Currency&nbsp;(15)</a> to SettlCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "155")]
	pub settl_curr_fx_rate: Option<f64>,
	/// Specifies whether the <a href="tag_155_SettlCurrFxRate.html" target="bottom">SettlCurrFxRate&nbsp;(155)</a> should be multiplied or divided
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "156")]
	pub settl_curr_fx_rate_calc: Option<SettlCurrFxRateCalc>,
	/// HandlInst
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "21")]
	pub handl_inst: Option<HandlInst>,
	/// MinQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "110")]
	pub min_qty: Option<f64>,
	/// MatchIncrement
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1089")]
	pub match_increment: Option<f64>,
	/// MaxPriceLevels
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1090")]
	pub max_price_levels: Option<i32>,
	/// Insert here the set of "DisplayInstruction" fields defined in "common components of application messages".
	#[serde(flatten)]
	pub display_instruction: Option<super::super::display_instruction::DisplayInstruction>,
	/// MaxFloor
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "111")]
	pub max_floor: Option<f64>,
	/// For use in derivatives omnibus accounting
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "77")]
	pub position_effect: Option<PositionEffect>,
	/// (Deprecated in FIX.5.0)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "210")]
	pub max_show: Option<f64>,
	/// Method for booking out this order. Used when notifying a broker that an order to be settled by that broker is to be booked
	/// out as an OTC derivative (e.g. CFD or similar). Absence of this field implies regular booking.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "775")]
	pub booking_type: Option<BookingType>,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if <a href="tag_355_EncodedText.html" target="bottom">EncodedText&nbsp;(355)</a> field is specified and must immediately precede it.
	#[serde(rename = "354")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_58_Text.html" target="bottom">Text&nbsp;(58)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
	/// (Deprecated in FIX.5.0)Can be used with <a href="tag_40_OrdType.html" target="bottom">OrdType&nbsp;(40)</a> = "Forex - Swap" to specify the "value date" for the future portion of a F/X swap.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "193")]
	pub settl_date_2: Option<fix_common::LocalMktDate>,
	/// (Deprecated in FIX.5.0)Can be used with <a href="tag_40_OrdType.html" target="bottom">OrdType&nbsp;(40)</a> = "Forex - Swap" to specify the order quantity for the future portion of a F/X swap.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "192")]
	pub order_qty_2: Option<f64>,
	/// Can be used with <a href="tag_40_OrdType.html" target="bottom">OrdType&nbsp;(40)</a> = "Forex - Swap" to specify the forward points (added to LastSpotRate) for the future portion of a F/X swap.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "641")]
	pub last_forward_points_2: Option<f64>,
	/// Default is a single security if not specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "442")]
	pub multi_leg_reporting_type: Option<MultiLegReportingType>,
	/// For CIV - Optional
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "480")]
	pub cancellation_rights: Option<CancellationRights>,
	/// MoneyLaunderingStatus
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "481")]
	pub money_laundering_status: Option<MoneyLaunderingStatus>,
	/// Reference to <a href="message_Registration_Instructions_o.html" target="main">Registration Instructions&nbsp;(o)</a> message for this Order.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "513")]
	pub regist_id: Option<String>,
	/// Supplementary registration information for this Order
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "494")]
	pub designation: Option<String>,
	/// For CIV - Optional
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "483")]
	pub trans_bkd_time: Option<fix_common::UTCTimestamp>,
	/// For CIV - Optional
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "515")]
	pub exec_valuation_point: Option<fix_common::UTCTimestamp>,
	/// For CIV - Optional
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "484")]
	pub exec_price_type: Option<ExecPriceType>,
	/// For CIV - Optional
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "485")]
	pub exec_price_adjustment: Option<f64>,
	/// PriorityIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "638")]
	pub priority_indicator: Option<PriorityIndicator>,
	/// PriceImprovement
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "639")]
	pub price_improvement: Option<f64>,
	/// Applicable only on <a href="tag_39_OrdStatus.html" target="bottom">OrdStatus&nbsp;(39)</a> of Partial or Filled.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "851")]
	pub last_liquidity_ind: Option<LastLiquidityInd>,
	/// Number of contract details in this message (number of repeating groups to follow)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "518")]
	pub cont_amts: Option<fix_common::RepeatingValues<ContAmt>>,
	/// Number of legs. Identifies a Multi-leg Execution if present and non-zero.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "555")]
	pub legs: Option<fix_common::RepeatingValues<Leg>>,
	/// CopyMsgIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "797")]
	pub copy_msg_indicator: Option<fix_common::Boolean>,
	/// Required if any miscellaneous fees are reported. Indicates number of repeating entries. Repeating group. ** Nested Repeating
	/// Group follows **
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "136")]
	pub misc_fees: Option<fix_common::RepeatingValues<MiscFee>>,
	/// DividendYield
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1380")]
	pub dividend_yield: Option<f32>,
	/// ManualOrderIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1028")]
	pub manual_order_indicator: Option<fix_common::Boolean>,
	/// CustDirectedOrder
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1029")]
	pub cust_directed_order: Option<fix_common::Boolean>,
	/// ReceivedDeptID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1030")]
	pub received_dept_id: Option<String>,
	/// CustOrderHandlingInst
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1031")]
	pub cust_order_handling_inst: Option<fix_common::SeparatedValues<CustOrderHandlingInst>>,
	/// OrderHandlingInstSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1032")]
	pub order_handling_inst_source: Option<OrderHandlingInstSource>,
	/// TrdRegTimestamps
	#[serde(flatten)]
	pub trd_reg_timestamps: Option<super::super::trd_reg_timestamps::TrdRegTimestamps>,
	/// Volatility
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1188")]
	pub volatility: Option<f64>,
	/// TimeToExpiration
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1189")]
	pub time_to_expiration: Option<f64>,
	/// RiskFreeRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1190")]
	pub risk_free_rate: Option<f64>,
	/// PriceDelta
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "811")]
	pub price_delta: Option<f64>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ContraBroker {
	/// First field in repeating group. Required if <a href="tag_382_NoContraBrokers.html" target="bottom">NoContraBrokers&nbsp;(382)</a> &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "375")]
	pub contra_broker: Option<String>,
	/// ContraTrader
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "337")]
	pub contra_trader: Option<String>,
	/// ContraTradeQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "437")]
	pub contra_trade_qty: Option<f64>,
	/// ContraTradeTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "438")]
	pub contra_trade_time: Option<fix_common::UTCTimestamp>,
	/// ContraLegRefID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "655")]
	pub contra_leg_ref_id: Option<String>,
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
	/// AllocSettlCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "736")]
	pub alloc_settl_currency: Option<AllocSettlCurrency>,
	/// IndividualAllocID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "467")]
	pub individual_alloc_id: Option<String>,
	/// AllocQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "80")]
	pub alloc_qty: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Underlying {
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StrategyParameter {
	/// Name of parameter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "958")]
	pub strategy_parameter_name: Option<String>,
	/// Datatype of the parameter.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "959")]
	pub strategy_parameter_type: Option<StrategyParameterType>,
	/// Value of the parameter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "960")]
	pub strategy_parameter_value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Fill {
	/// Required if <a href="tag_1362_NoFills.html" target="bottom">NoFills&nbsp;(1362)</a> &gt; 0. Must be first field in repeating group. Unique identifier of execution as assigned by sell-side (broker, exchange, ECN).
	/// Must not overlap ExecID(17).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1363")]
	pub fill_exec_id: Option<String>,
	/// Price of this partial fill. Conditionally required if <a href="tag_1362_NoFills.html" target="bottom">NoFills&nbsp;(1362)</a> &gt; 0. Refer to LastPx(31).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1364")]
	pub fill_px: Option<f64>,
	/// Quantity (e.g. shares) bought/sold on this partial fill. Required if <a href="tag_1362_NoFills.html" target="bottom">NoFills&nbsp;(1362)</a> &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1365")]
	pub fill_qty: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ContAmt {
	/// Must be first field in the repeating group.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "519")]
	pub cont_amt_type: Option<ContAmtType>,
	/// ContAmtValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "520")]
	pub cont_amt_value: Option<f64>,
	/// ContAmtCurr
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "521")]
	pub cont_amt_curr: Option<ContAmtCurr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Leg {
	/// Deprecated in FIX.5.0 SP1.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "687")]
	pub leg_qty: Option<f64>,
	/// When reporting an Execution, <a href="tag_685_LegOrderQty.html" target="bottom">LegOrderQty&nbsp;(685)</a> may be used on Execution Report to echo back original <a href="tag_685_LegOrderQty.html" target="bottom">LegOrderQty&nbsp;(685)</a> submission. This field should be used to specify <a href="tag_38_OrderQty.html" target="bottom">OrderQty&nbsp;(38)</a> at the leg level rather than <a href="tag_687_LegQty.html" target="bottom">LegQty&nbsp;(687)</a> (deprecated).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "685")]
	pub leg_order_qty: Option<f64>,
	/// Instead of <a href="tag_687_LegQty.html" target="bottom">LegQty&nbsp;(687)</a> requests that the sellside calculate <a href="tag_687_LegQty.html" target="bottom">LegQty&nbsp;(687)</a> based on opposite Leg
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "690")]
	pub leg_swap_type: Option<LegSwapType>,
	/// LegAllocID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1366")]
	pub leg_alloc_id: Option<String>,
	/// NoLegAllocs
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "670")]
	pub no_leg_allocs: Option<usize>,
	/// LegAllocAccount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "671")]
	pub leg_alloc_account: Option<String>,
	/// LegIndividualAllocID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "672")]
	pub leg_individual_alloc_id: Option<String>,
	/// LegAllocQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "673")]
	pub leg_alloc_qty: Option<f64>,
	/// LegAllocAcctIDSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "674")]
	pub leg_alloc_acct_id_source: Option<LegAllocAcctIDSource>,
	/// LegAllocSettlCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1367")]
	pub leg_alloc_settl_currency: Option<LegAllocSettlCurrency>,
	/// Provide if the <a href="tag_77_PositionEffect.html" target="bottom">PositionEffect&nbsp;(77)</a> for the leg is different from that specified for the overall multileg security
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "564")]
	pub leg_position_effect: Option<LegPositionEffect>,
	/// Provide if the <a href="tag_203_CoveredOrUncovered.html" target="bottom">CoveredOrUncovered&nbsp;(203)</a> for the leg is different from that specified for the overall multileg security.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "565")]
	pub leg_covered_or_uncovered: Option<LegCoveredOrUncovered>,
	/// Used to identify a specific leg.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "654")]
	pub leg_ref_id: Option<String>,
	/// LegSettlType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "587")]
	pub leg_settl_type: Option<LegSettlType>,
	/// Takes precedence over <a href="tag_587_LegSettlType.html" target="bottom">LegSettlType&nbsp;(587)</a> value and conditionally required/omitted for specific <a href="tag_587_LegSettlType.html" target="bottom">LegSettlType&nbsp;(587)</a> values.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "588")]
	pub leg_settl_date: Option<fix_common::LocalMktDate>,
	/// Used to report the execution price assigned to the leg of the multileg instrument
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "637")]
	pub leg_last_px: Option<f64>,
	/// LegSettlCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "675")]
	pub leg_settl_currency: Option<LegSettlCurrency>,
	/// LegLastForwardPoints
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1073")]
	pub leg_last_forward_points: Option<f64>,
	/// LegCalculatedCcyLastQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1074")]
	pub leg_calculated_ccy_last_qty: Option<f64>,
	/// For FX Futures can be used to express the notional value of a trade when LegLastQty and other quantity fields are expressed
	/// in terms of number of contracts - <a href="tag_614_LegContractMultiplier.html" target="bottom">LegContractMultiplier&nbsp;(614)</a> (231) is required in this case.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1075")]
	pub leg_gross_trade_amt: Option<f64>,
	/// LegVolatility
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1379")]
	pub leg_volatility: Option<f64>,
	/// LegDividendYield
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1381")]
	pub leg_dividend_yield: Option<f32>,
	/// LegCurrencyRatio
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1383")]
	pub leg_currency_ratio: Option<f64>,
	/// LegExecInst
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1384")]
	pub leg_exec_inst: Option<fix_common::SeparatedValues<LegExecInst>>,
	/// LegLastQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1418")]
	pub leg_last_qty: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MiscFee {
	/// Required if <a href="tag_136_NoMiscFees.html" target="bottom">NoMiscFees&nbsp;(136)</a> &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "137")]
	pub misc_fee_amt: Option<f64>,
	/// MiscFeeCurr
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "138")]
	pub misc_fee_curr: Option<MiscFeeCurr>,
	/// Required if <a href="tag_136_NoMiscFees.html" target="bottom">NoMiscFees&nbsp;(136)</a> &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "139")]
	pub misc_fee_type: Option<MiscFeeType>,
	/// MiscFeeBasis
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "891")]
	pub misc_fee_basis: Option<MiscFeeBasis>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LastRptRequested {
	/// Not last message
	#[serde(rename = "N")]
	NotLastMessage,
	/// Last message
	#[serde(rename = "Y")]
	LastMessage,
}

impl Default for LastRptRequested {
	fn default() -> Self {
		LastRptRequested::NotLastMessage
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum CrossType {
	/// Cross AON - cross tade which is executed complete or not. Both sides are treated in the same manner. This is equivalent to
	/// an "All or None".
	#[serde(rename = "1")]
	CrossAonCrossTadeWhichIsExecutedCompleteOrNotBothSidesAreTreatedInTheSameMannerThisIsEquivalentToAnAllOrNone,
	/// Cross IOC - cross trade which is executed partially and the rest is cancelled. One side is fully executed, the other side
	/// is partially executed with the remainder being cancelled. This is equivalent to an IOC on the other side. Note: <a href="tag_550_CrossPrioritization.html" target="bottom">CrossPrioritization&nbsp;(550)</a> field may be used to indicate which side should fully execute in this scenario.
	#[serde(rename = "2")]
	CrossIocCrossTradeWhichIsExecutedPartiallyAndTheRestIsCancelledOneSideIsFullyExecutedTheOtherSideIsPartiallyExecutedWithTheRemainderBeingCancelledThisIsEquivalentToAnIocOnTheOtherSideNoteAHrefTag550CrossPrioritizationHtmlTargetBottomCrossPrioritizationNbspAFieldMayBeUsedToIndicateWhichSideShouldFullyExecuteInThisScenario,
	/// Cross One Side - cross trade which is partially executed with the unfilled portions remaining active. One side of the corss
	/// is fully executed (as denoted by the <a href="tag_550_CrossPrioritization.html" target="bottom">CrossPrioritization&nbsp;(550)</a> field), but the unfilled portion remains active.
	#[serde(rename = "3")]
	CrossOneSideCrossTradeWhichIsPartiallyExecutedWithTheUnfilledPortionsRemainingActiveOneSideOfTheCorssIsFullyExecutedAFieldButTheUnfilledPortionRemainsActive,
	/// Cross Same Price - cross trade is executed with existing orders with the same price. In this case other orders exist with
	/// the same price, the quantity of the Cross is executed against the existing orders and quotes, the remainder of the corss is
	/// executed against the other side of the cross. The two sides potentially have different quantities.
	#[serde(rename = "4")]
	CrossSamePriceCrossTradeIsExecutedWithExistingOrdersWithTheSamePriceInThisCaseOtherOrdersExistWithTheSamePriceTheQuantityOfTheCrossIsExecutedAgainstTheExistingOrdersAndQuotesTheRemainderOfTheCorssIsExecutedAgainstTheOtherSideOfTheCrossTheTwoSidesPotentiallyHaveDifferentQuantities,
}

impl Default for CrossType {
	fn default() -> Self {
		CrossType::CrossAonCrossTadeWhichIsExecutedCompleteOrNotBothSidesAreTreatedInTheSameMannerThisIsEquivalentToAnAllOrNone
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ExecType {
	/// New
	#[serde(rename = "0")]
	New,
	/// Done for day
	#[serde(rename = "3")]
	DoneForDay,
	/// Canceled
	#[serde(rename = "4")]
	Canceled,
	/// Replaced
	#[serde(rename = "5")]
	Replaced,
	/// Pending Cancel (e.g. result of Order Cancel Request)
	#[serde(rename = "6")]
	PendingCancel,
	/// Stopped
	#[serde(rename = "7")]
	Stopped,
	/// Rejected
	#[serde(rename = "8")]
	Rejected,
	/// Suspended
	#[serde(rename = "9")]
	Suspended,
	/// Pending New
	#[serde(rename = "A")]
	PendingNew,
	/// Calculated
	#[serde(rename = "B")]
	Calculated,
	/// Expired
	#[serde(rename = "C")]
	Expired,
	/// Restated (Execution Report sent unsolicited by sellside, with <a href="tag_378_ExecRestatementReason.html" target="bottom">ExecRestatementReason&nbsp;(378)</a> set)
	#[serde(rename = "D")]
	RestatedASet,
	/// Pending Replace (e.g. result of <a href="message_Order_Cancel_Replace_Request_G.html" target="main">Order Cancel/Replace Request&nbsp;(G)</a> )
	#[serde(rename = "E")]
	PendingReplaceA,
	/// Trade (partial fill or fill)
	#[serde(rename = "F")]
	Trade,
	/// Trade Correct (formerly an ExecTransType)
	#[serde(rename = "G")]
	TradeCorrect,
	/// Trade Cancel (formerly an ExecTransType)
	#[serde(rename = "H")]
	TradeCancel,
	/// Order Status (formerly an ExecTransType)
	#[serde(rename = "I")]
	OrderStatus,
	/// Trade in a Clearing Hold
	#[serde(rename = "J")]
	TradeInAClearingHold,
	/// Trade has been released to Clearing
	#[serde(rename = "K")]
	TradeHasBeenReleasedToClearing,
	/// Triggered or Activated by System
	#[serde(rename = "L")]
	TriggeredOrActivatedBySystem,
}

impl Default for ExecType {
	fn default() -> Self {
		ExecType::New
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum OrdStatus {
	/// New
	#[serde(rename = "0")]
	New,
	/// Partially filled
	#[serde(rename = "1")]
	PartiallyFilled,
	/// Filled
	#[serde(rename = "2")]
	Filled,
	/// Done for day
	#[serde(rename = "3")]
	DoneForDay,
	/// Canceled
	#[serde(rename = "4")]
	Canceled,
	/// Replaced (No longer used)
	#[serde(rename = "5")]
	Replaced,
	/// Pending Cancel (i.e. result of Order Cancel Request)
	#[serde(rename = "6")]
	PendingCancel,
	/// Stopped
	#[serde(rename = "7")]
	Stopped,
	/// Rejected
	#[serde(rename = "8")]
	Rejected,
	/// Suspended
	#[serde(rename = "9")]
	Suspended,
	/// Pending New
	#[serde(rename = "A")]
	PendingNew,
	/// Calculated
	#[serde(rename = "B")]
	Calculated,
	/// Expired
	#[serde(rename = "C")]
	Expired,
	/// Accepted for Bidding
	#[serde(rename = "D")]
	AcceptedForBidding,
	/// Pending Replace (i.e. result of Order Cancel/Replace Request)
	#[serde(rename = "E")]
	PendingReplace,
}

impl Default for OrdStatus {
	fn default() -> Self {
		OrdStatus::New
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum WorkingIndicator {
	/// Order has been accepted but not yet in a working state
	#[serde(rename = "N")]
	OrderHasBeenAcceptedButNotYetInAWorkingState,
	/// Order is currently being worked
	#[serde(rename = "Y")]
	OrderIsCurrentlyBeingWorked,
}

impl Default for WorkingIndicator {
	fn default() -> Self {
		WorkingIndicator::OrderHasBeenAcceptedButNotYetInAWorkingState
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum OrdRejReason {
	/// Broker / Exchange option
	#[serde(rename = "0")]
	BrokerExchangeOption,
	/// Unknown symbol
	#[serde(rename = "1")]
	UnknownSymbol,
	/// Exchange closed
	#[serde(rename = "2")]
	ExchangeClosed,
	/// Order exceeds limit
	#[serde(rename = "3")]
	OrderExceedsLimit,
	/// Too late to enter
	#[serde(rename = "4")]
	TooLateToEnter,
	/// Unknown order
	#[serde(rename = "5")]
	UnknownOrder,
	/// Duplicate Order (e.g. dupe ClOrdID)
	#[serde(rename = "6")]
	DuplicateOrder,
	/// Duplicate of a verbally communicated order
	#[serde(rename = "7")]
	DuplicateOfAVerballyCommunicatedOrder,
	/// Stale order
	#[serde(rename = "8")]
	StaleOrder,
	/// Trade along required
	#[serde(rename = "9")]
	TradeAlongRequired,
	/// Invalid Investor ID
	#[serde(rename = "10")]
	InvalidInvestorId,
	/// Unsupported order characteristic
	#[serde(rename = "11")]
	UnsupportedOrderCharacteristic,
	/// Surveillence Option
	#[serde(rename = "12")]
	SurveillenceOption,
	/// Incorrect quantity
	#[serde(rename = "13")]
	IncorrectQuantity,
	/// Incorrect allocated quantity
	#[serde(rename = "14")]
	IncorrectAllocatedQuantity,
	/// Unknown account(s)
	#[serde(rename = "15")]
	UnknownAccount,
	/// Price exceeds current price band
	#[serde(rename = "16")]
	PriceExceedsCurrentPriceBand,
	/// Invalid price increment
	#[serde(rename = "18")]
	InvalidPriceIncrement,
	/// Other
	#[serde(rename = "99")]
	Other,
}

impl Default for OrdRejReason {
	fn default() -> Self {
		OrdRejReason::BrokerExchangeOption
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ExecRestatementReason {
	/// GT Corporate action
	#[serde(rename = "0")]
	GtCorporateAction,
	/// GT renewal / restatement (no corporate action)
	#[serde(rename = "1")]
	GtRenewalRestatement,
	/// Verbal change
	#[serde(rename = "2")]
	VerbalChange,
	/// Repricing of order
	#[serde(rename = "3")]
	RepricingOfOrder,
	/// Broker option
	#[serde(rename = "4")]
	BrokerOption,
	/// Partial decline of <a href="tag_38_OrderQty.html" target="bottom">OrderQty&nbsp;(38)</a> (e.g. exchange-initiated partial cancel)
	#[serde(rename = "5")]
	PartialDeclineOfAHrefTag38OrderQtyHtmlTargetBottomOrderQtyNbspA,
	/// Cancel on Trading Halt
	#[serde(rename = "6")]
	CancelOnTradingHalt,
	/// Cancel on System Failure
	#[serde(rename = "7")]
	CancelOnSystemFailure,
	/// Market (Exchange) Option
	#[serde(rename = "8")]
	MarketOption,
	/// Canceled, Not Best
	#[serde(rename = "9")]
	CanceledNotBest,
	/// Warehouse recap
	#[serde(rename = "10")]
	WarehouseRecap,
	/// Peg Refresh
	#[serde(rename = "11")]
	PegRefresh,
	/// Other
	#[serde(rename = "99")]
	Other,
}

impl Default for ExecRestatementReason {
	fn default() -> Self {
		ExecRestatementReason::GtCorporateAction
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
pub enum AccountType {
	/// Account is carried on customer Side of Books
	#[serde(rename = "1")]
	AccountIsCarriedOnCustomerSideOfBooks,
	/// Account is carried on non-Customer Side of books
	#[serde(rename = "2")]
	AccountIsCarriedOnNonCustomerSideOfBooks,
	/// House Trader
	#[serde(rename = "3")]
	HouseTrader,
	/// Floor Trader
	#[serde(rename = "4")]
	FloorTrader,
	/// Account is carried on non-customer side of books and is cross margined
	#[serde(rename = "6")]
	AccountIsCarriedOnNonCustomerSideOfBooksAndIsCrossMargined,
	/// Account is house trader and is cross margined
	#[serde(rename = "7")]
	AccountIsHouseTraderAndIsCrossMargined,
	/// Joint Backoffice Account (JBO)
	#[serde(rename = "8")]
	JointBackofficeAccount,
}

impl Default for AccountType {
	fn default() -> Self {
		AccountType::AccountIsCarriedOnCustomerSideOfBooks
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum DayBookingInst {
	/// Can trigger booking without reference to the order initiator ("auto")
	#[serde(rename = "0")]
	CanTriggerBookingWithoutReferenceToTheOrderInitiator,
	/// Speak with order initiator before booking ("speak first")
	#[serde(rename = "1")]
	SpeakWithOrderInitiatorBeforeBooking,
	/// Accumulate
	#[serde(rename = "2")]
	Accumulate,
}

impl Default for DayBookingInst {
	fn default() -> Self {
		DayBookingInst::CanTriggerBookingWithoutReferenceToTheOrderInitiator
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum BookingUnit {
	/// Each partial execution is a bookable unit
	#[serde(rename = "0")]
	EachPartialExecutionIsABookableUnit,
	/// Aggregate partial executions on this order, and book one trade per order
	#[serde(rename = "1")]
	AggregatePartialExecutionsOnThisOrderAndBookOneTradePerOrder,
	/// Aggregate executions for this symbol, side, and settlement date
	#[serde(rename = "2")]
	AggregateExecutionsForThisSymbolSideAndSettlementDate,
}

impl Default for BookingUnit {
	fn default() -> Self {
		BookingUnit::EachPartialExecutionIsABookableUnit
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PreallocMethod {
	/// Pro-rata
	#[serde(rename = "0")]
	ProRata,
	/// Do not pro-rata - discuss first
	#[serde(rename = "1")]
	DoNotProRataDiscussFirst,
}

impl Default for PreallocMethod {
	fn default() -> Self {
		PreallocMethod::ProRata
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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
}

impl Default for MatchType {
	fn default() -> Self {
		MatchType::ActAcceptedTrade
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum OrderCategory {
	/// Order
	#[serde(rename = "1")]
	Order,
	/// Quote
	#[serde(rename = "2")]
	Quote,
	/// Privately Negotiated Trade
	#[serde(rename = "3")]
	PrivatelyNegotiatedTrade,
	/// Multileg order
	#[serde(rename = "4")]
	MultilegOrder,
	/// Linked order
	#[serde(rename = "5")]
	LinkedOrder,
	/// Quote Request
	#[serde(rename = "6")]
	QuoteRequest,
	/// Implied Order
	#[serde(rename = "7")]
	ImpliedOrder,
	/// Cross Order
	#[serde(rename = "8")]
	CrossOrder,
	/// Streaming price (quote)
	#[serde(rename = "9")]
	StreamingPrice,
}

impl Default for OrderCategory {
	fn default() -> Self {
		OrderCategory::Order
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum CashMargin {
	/// Cash
	#[serde(rename = "1")]
	Cash,
	/// Margin Open
	#[serde(rename = "2")]
	MarginOpen,
	/// Margin Close
	#[serde(rename = "3")]
	MarginClose,
}

impl Default for CashMargin {
	fn default() -> Self {
		CashMargin::Cash
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ClearingFeeIndicator {
	/// 1st year delegate trading for own account
	#[serde(rename = "1")]
	N1StYearDelegateTradingForOwnAccount,
	/// 2nd year delegate trading for own account
	#[serde(rename = "2")]
	N2NdYearDelegateTradingForOwnAccount,
	/// 3rd year delegate trading for own account
	#[serde(rename = "3")]
	N3RdYearDelegateTradingForOwnAccount,
	/// 4th year delegate trading for own account
	#[serde(rename = "4")]
	N4ThYearDelegateTradingForOwnAccount,
	/// 5th year delegate trading for own account
	#[serde(rename = "5")]
	N5ThYearDelegateTradingForOwnAccount,
	/// 6th year delegate trading for own account
	#[serde(rename = "9")]
	N6ThYearDelegateTradingForOwnAccount,
	/// CBOE Member
	#[serde(rename = "B")]
	CboeMember,
	/// Non-member and Customer
	#[serde(rename = "C")]
	NonMemberAndCustomer,
	/// Equity Member and Clearing Member
	#[serde(rename = "E")]
	EquityMemberAndClearingMember,
	/// Full and Associate Member trading for own account and as floor brokers
	#[serde(rename = "F")]
	FullAndAssociateMemberTradingForOwnAccountAndAsFloorBrokers,
	/// 106.H and 106.J firms
	#[serde(rename = "H")]
	N106HAnd106JFirms,
	/// GIM, IDEM and COM Membership Interest Holders
	#[serde(rename = "I")]
	GimIdemAndComMembershipInterestHolders,
	/// Lessee 106.F Employees
	#[serde(rename = "L")]
	Lessee106FEmployees,
	/// All other ownership types
	#[serde(rename = "M")]
	AllOtherOwnershipTypes,
}

impl Default for ClearingFeeIndicator {
	fn default() -> Self {
		ClearingFeeIndicator::N1StYearDelegateTradingForOwnAccount
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

impl Default for QtyType {
	fn default() -> Self {
		QtyType::Units
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LotType {
	/// Odd Lot
	#[serde(rename = "1")]
	OddLot,
	/// Round Lot
	#[serde(rename = "2")]
	RoundLot,
	/// Block Lot
	#[serde(rename = "3")]
	BlockLot,
}

impl Default for LotType {
	fn default() -> Self {
		LotType::OddLot
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum OrdType {
	/// Market
	#[serde(rename = "1")]
	Market,
	/// Limit
	#[serde(rename = "2")]
	Limit,
	/// Stop / Stop Loss
	#[serde(rename = "3")]
	StopStopLoss,
	/// Stop Limit
	#[serde(rename = "4")]
	StopLimit,
	/// Market On Close (No longer used)
	#[serde(rename = "5")]
	MarketOnClose,
	/// With Or Without
	#[serde(rename = "6")]
	WithOrWithout,
	/// Limit Or Better
	#[serde(rename = "7")]
	LimitOrBetter,
	/// Limit With Or Without
	#[serde(rename = "8")]
	LimitWithOrWithout,
	/// On Basis
	#[serde(rename = "9")]
	OnBasis,
	/// On Close (No longer used)
	#[serde(rename = "A")]
	OnClose,
	/// Limit On Close (No longer used)
	#[serde(rename = "B")]
	LimitOnClose,
	/// Forex Market (No longer used)
	#[serde(rename = "C")]
	ForexMarket,
	/// Previously Quoted
	#[serde(rename = "D")]
	PreviouslyQuoted,
	/// Previously Indicated
	#[serde(rename = "E")]
	PreviouslyIndicated,
	/// Forex Limit (No longer used)
	#[serde(rename = "F")]
	ForexLimit,
	/// Forex Swap
	#[serde(rename = "G")]
	ForexSwap,
	/// Forex Previously Quoted (No longer used)
	#[serde(rename = "H")]
	ForexPreviouslyQuoted,
	/// Funari (Limit day order with unexecuted portion handles as Market On Close. E.g. Japan)
	#[serde(rename = "I")]
	Funari,
	/// Market If Touched (MIT)
	#[serde(rename = "J")]
	MarketIfTouched,
	/// Market With Left Over as Limit (market order with unexecuted quantity becoming limit order at last price)
	#[serde(rename = "K")]
	MarketWithLeftOverAsLimit,
	/// Previous Fund Valuation Point (Historic pricing; for CIV)"
	#[serde(rename = "L")]
	PreviousFundValuationPoint,
	/// Next Fund Valuation Point (Forward pricing; for CIV)"
	#[serde(rename = "M")]
	NextFundValuationPoint,
	/// Pegged
	#[serde(rename = "P")]
	Pegged,
	/// Counter-order selection
	#[serde(rename = "Q")]
	CounterOrderSelection,
}

impl Default for OrdType {
	fn default() -> Self {
		OrdType::Market
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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
	/// Product ticks in halfs
	#[serde(rename = "13")]
	ProductTicksInHalfs,
	/// Product ticks in fourths
	#[serde(rename = "14")]
	ProductTicksInFourths,
	/// Product ticks in eights
	#[serde(rename = "15")]
	ProductTicksInEights,
	/// Product ticks in sixteenths
	#[serde(rename = "16")]
	ProductTicksInSixteenths,
	/// Product ticks in thirty-seconds
	#[serde(rename = "17")]
	ProductTicksInThirtySeconds,
	/// Product ticks in sixty-forths
	#[serde(rename = "18")]
	ProductTicksInSixtyForths,
	/// Product ticks in one-twenty-eights
	#[serde(rename = "19")]
	ProductTicksInOneTwentyEights,
}

impl Default for PriceType {
	fn default() -> Self {
		PriceType::Percentage
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PriceProtectionScope {
	/// None
	#[serde(rename = "0")]
	None,
	/// Local (Exchange, ECN, ATS)
	#[serde(rename = "1")]
	Local,
	/// National (Across all national markets)
	#[serde(rename = "2")]
	National,
	/// Global (Across all markets)
	#[serde(rename = "3")]
	Global,
}

impl Default for PriceProtectionScope {
	fn default() -> Self {
		PriceProtectionScope::None
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TargetStrategy {
	/// VWAP
	#[serde(rename = "1")]
	Vwap,
	/// Participate (i.e. aim to be x percent of the market volume)
	#[serde(rename = "2")]
	Participate,
	/// Mininize market impact
	#[serde(rename = "3")]
	MininizeMarketImpact,
}

impl Default for TargetStrategy {
	fn default() -> Self {
		TargetStrategy::Vwap
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

impl Default for Currency {
	fn default() -> Self {
		Currency::Afa
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SolicitedFlag {
	/// Was not solicited
	#[serde(rename = "N")]
	WasNotSolicited,
	/// Was solicited
	#[serde(rename = "Y")]
	WasSolicited,
}

impl Default for SolicitedFlag {
	fn default() -> Self {
		SolicitedFlag::WasNotSolicited
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TimeInForce {
	/// Day (or session)
	#[serde(rename = "0")]
	Day,
	/// Good Till Cancel (GTC)
	#[serde(rename = "1")]
	GoodTillCancel,
	/// At the Opening (OPG)
	#[serde(rename = "2")]
	AtTheOpening,
	/// Immediate Or Cancel (IOC)
	#[serde(rename = "3")]
	ImmediateOrCancel,
	/// Fill Or Kill (FOK)
	#[serde(rename = "4")]
	FillOrKill,
	/// Good Till Crossing (GTX)
	#[serde(rename = "5")]
	GoodTillCrossing,
	/// Good Till Date (GTD)
	#[serde(rename = "6")]
	GoodTillDate,
	/// At the Close
	#[serde(rename = "7")]
	AtTheClose,
	/// Good Through Crossing
	#[serde(rename = "8")]
	GoodThroughCrossing,
	/// At Crossing
	#[serde(rename = "9")]
	AtCrossing,
}

impl Default for TimeInForce {
	fn default() -> Self {
		TimeInForce::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ExecInst {
	/// Stay on offerside
	#[serde(rename = "0")]
	StayOnOfferside,
	/// Not held
	#[serde(rename = "1")]
	NotHeld,
	/// Work
	#[serde(rename = "2")]
	Work,
	/// Go along
	#[serde(rename = "3")]
	GoAlong,
	/// Over the day
	#[serde(rename = "4")]
	OverTheDay,
	/// Held
	#[serde(rename = "5")]
	Held,
	/// Participate don't initiate
	#[serde(rename = "6")]
	ParticipateDonTInitiate,
	/// Strict scale
	#[serde(rename = "7")]
	StrictScale,
	/// Try to scale
	#[serde(rename = "8")]
	TryToScale,
	/// Stay on bidside
	#[serde(rename = "9")]
	StayOnBidside,
	/// No cross (cross is forbidden)
	#[serde(rename = "A")]
	NoCross,
	/// OK to cross
	#[serde(rename = "B")]
	OkToCross,
	/// Call first
	#[serde(rename = "C")]
	CallFirst,
	/// Percent of volume (indicates that the sender does not want to be all of the volume on the floor vs. a specific percentage)
	#[serde(rename = "D")]
	PercentOfVolume,
	/// Do not increase - DNI
	#[serde(rename = "E")]
	DoNotIncreaseDni,
	/// Do not reduce - DNR
	#[serde(rename = "F")]
	DoNotReduceDnr,
	/// All or none - AON
	#[serde(rename = "G")]
	AllOrNoneAon,
	/// Reinstate on System Failure (mutually exclusive with Q)
	#[serde(rename = "H")]
	ReinstateOnSystemFailure,
	/// Institutions only
	#[serde(rename = "I")]
	InstitutionsOnly,
	/// Reinstate on Trading Halt (mutually exclusive with K)
	#[serde(rename = "J")]
	ReinstateOnTradingHalt,
	/// Cancel on Trading Halt (mutually exclusive with J)
	#[serde(rename = "K")]
	CancelOnTradingHalt,
	/// Last peg (last sale)
	#[serde(rename = "L")]
	LastPeg,
	/// Mid-price peg (midprice of inside quote)
	#[serde(rename = "M")]
	MidPricePeg,
	/// Non-negotiable
	#[serde(rename = "N")]
	NonNegotiable,
	/// Opening peg
	#[serde(rename = "O")]
	OpeningPeg,
	/// Market peg
	#[serde(rename = "P")]
	MarketPeg,
	/// Cancel on System Failure (mutually exclusive with H)
	#[serde(rename = "Q")]
	CancelOnSystemFailure,
	/// Primary peg (primary market - buy at bid/sell at offer)
	#[serde(rename = "R")]
	PrimaryPeg,
	/// Suspend
	#[serde(rename = "S")]
	Suspend,
	/// Fixed Peg to Local best bid or offer at time of order
	#[serde(rename = "T")]
	FixedPegToLocalBestBidOrOfferAtTimeOfOrder,
	/// Customer Display Instruction (Rule11Ac1-1/4)
	#[serde(rename = "U")]
	CustomerDisplayInstruction,
	/// Netting (for Forex)
	#[serde(rename = "V")]
	Netting,
	/// Peg to VWAP
	#[serde(rename = "W")]
	PegToVwap,
	/// Trade Along
	#[serde(rename = "X")]
	TradeAlong,
	/// Try to Stop
	#[serde(rename = "Y")]
	TryToStop,
	/// Cancel if Not Best
	#[serde(rename = "Z")]
	CancelIfNotBest,
	/// Trailing Stop Peg
	#[serde(rename = "a")]
	TrailingStopPeg,
	/// Strict Limit (No Price Improvement)
	#[serde(rename = "b")]
	StrictLimit,
	/// Ignore Price Validity Checks
	#[serde(rename = "c")]
	IgnorePriceValidityChecks,
	/// Peg to Limit Price
	#[serde(rename = "d")]
	PegToLimitPrice,
	/// Work to Target Strategy
	#[serde(rename = "e")]
	WorkToTargetStrategy,
	/// Intermarket Sweep
	#[serde(rename = "f")]
	IntermarketSweep,
	/// External Routing Allowed
	#[serde(rename = "g")]
	ExternalRoutingAllowed,
	/// External Routing Not Allowed
	#[serde(rename = "h")]
	ExternalRoutingNotAllowed,
	/// Imbalance Only
	#[serde(rename = "i")]
	ImbalanceOnly,
	/// Single execution requested for block trade
	#[serde(rename = "j")]
	SingleExecutionRequestedForBlockTrade,
	/// Best Execution
	#[serde(rename = "k")]
	BestExecution,
	/// Suspend on system failure (mutually exclusive with H and Q)
	#[serde(rename = "l")]
	SuspendOnSystemFailure,
	/// Suspend on Trading Halt (mutually exclusive with J and K)
	#[serde(rename = "m")]
	SuspendOnTradingHalt,
	/// Reinstate on connection loss (mutually exclusive with o and p)
	#[serde(rename = "n")]
	ReinstateOnConnectionLoss,
	/// Cancel on connection loss (mutually exclusive with n and p)
	#[serde(rename = "o")]
	CancelOnConnectionLoss,
	/// Suspend on connection loss (mutually exclusive with n and o)
	#[serde(rename = "p")]
	SuspendOnConnectionLoss,
	/// Release from suspension (mutually exclusive with S)
	#[serde(rename = "q")]
	ReleaseFromSuspension,
	/// Execute as delta neutral using volatility provided
	#[serde(rename = "r")]
	ExecuteAsDeltaNeutralUsingVolatilityProvided,
	/// Execute as duration neutral
	#[serde(rename = "s")]
	ExecuteAsDurationNeutral,
	/// Execute as FX neutral
	#[serde(rename = "t")]
	ExecuteAsFxNeutral,
}

impl Default for ExecInst {
	fn default() -> Self {
		ExecInst::StayOnOfferside
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum AggressorIndicator {
	/// Order initiator is aggressor
	#[serde(rename = "Y")]
	OrderInitiatorIsAggressor,
	/// Order initiator is passive
	#[serde(rename = "N")]
	OrderInitiatorIsPassive,
}

impl Default for AggressorIndicator {
	fn default() -> Self {
		AggressorIndicator::OrderInitiatorIsAggressor
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum OrderCapacity {
	/// Agency
	#[serde(rename = "A")]
	Agency,
	/// Proprietary
	#[serde(rename = "G")]
	Proprietary,
	/// Individual
	#[serde(rename = "I")]
	Individual,
	/// Principal (Note for CMS purposes, "Principal" includes "Proprietary")"
	#[serde(rename = "P")]
	Principal,
	/// Riskless Principal
	#[serde(rename = "R")]
	RisklessPrincipal,
	/// Agent for Other Member
	#[serde(rename = "W")]
	AgentForOtherMember,
}

impl Default for OrderCapacity {
	fn default() -> Self {
		OrderCapacity::Agency
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum OrderRestrictions {
	/// Program Trade
	#[serde(rename = "1")]
	ProgramTrade,
	/// Index Arbitrage
	#[serde(rename = "2")]
	IndexArbitrage,
	/// Non-Index Arbitrage
	#[serde(rename = "3")]
	NonIndexArbitrage,
	/// Competing Market Maker
	#[serde(rename = "4")]
	CompetingMarketMaker,
	/// Acting as Market Maker or Specialist in the security
	#[serde(rename = "5")]
	ActingAsMarketMakerOrSpecialistInTheSecurity,
	/// Acting as Market Maker or Specialist in the underlying security of a derivative security
	#[serde(rename = "6")]
	ActingAsMarketMakerOrSpecialistInTheUnderlyingSecurityOfADerivativeSecurity,
	/// Foreign Entity (of foreign governmnet or regulatory jurisdiction)
	#[serde(rename = "7")]
	ForeignEntity,
	/// External Market Participant
	#[serde(rename = "8")]
	ExternalMarketParticipant,
	/// External Inter-connected Market Linkage
	#[serde(rename = "9")]
	ExternalInterConnectedMarketLinkage,
	/// Riskless Arbitrage
	#[serde(rename = "A")]
	RisklessArbitrage,
	/// Issuer Holding
	#[serde(rename = "B")]
	IssuerHolding,
	/// Issue Price Stabilization
	#[serde(rename = "C")]
	IssuePriceStabilization,
	/// Non-algorithmic
	#[serde(rename = "D")]
	NonAlgorithmic,
	/// Algorithmic
	#[serde(rename = "E")]
	Algorithmic,
}

impl Default for OrderRestrictions {
	fn default() -> Self {
		OrderRestrictions::ProgramTrade
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum CustOrderCapacity {
	/// Member trading for their own account
	#[serde(rename = "1")]
	MemberTradingForTheirOwnAccount,
	/// Clearing Firm trading for its proprietary account
	#[serde(rename = "2")]
	ClearingFirmTradingForItsProprietaryAccount,
	/// Member trading for another member
	#[serde(rename = "3")]
	MemberTradingForAnotherMember,
	/// All other
	#[serde(rename = "4")]
	AllOther,
}

impl Default for CustOrderCapacity {
	fn default() -> Self {
		CustOrderCapacity::MemberTradingForTheirOwnAccount
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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
}

impl Default for TradingSessionID {
	fn default() -> Self {
		TradingSessionID::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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
}

impl Default for TradingSessionSubID {
	fn default() -> Self {
		TradingSessionSubID::PreTrading
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LastCapacity {
	/// Agent
	#[serde(rename = "1")]
	Agent,
	/// Cross as agent
	#[serde(rename = "2")]
	CrossAsAgent,
	/// Cross as principal
	#[serde(rename = "3")]
	CrossAsPrincipal,
	/// Principal
	#[serde(rename = "4")]
	Principal,
}

impl Default for LastCapacity {
	fn default() -> Self {
		LastCapacity::Agent
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LastFragment {
	/// Not Last Message
	#[serde(rename = "N")]
	NotLastMessage,
	/// Last Message
	#[serde(rename = "Y")]
	LastMessage,
}

impl Default for LastFragment {
	fn default() -> Self {
		LastFragment::NotLastMessage
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum GTBookingInst {
	/// Book out all trades on day of execution
	#[serde(rename = "0")]
	BookOutAllTradesOnDayOfExecution,
	/// Accumulate executions until order is filled or expires
	#[serde(rename = "1")]
	AccumulateExecutionsUntilOrderIsFilledOrExpires,
	/// Accumulate until verbally notified otherwise
	#[serde(rename = "2")]
	AccumulateUntilVerballyNotifiedOtherwise,
}

impl Default for GTBookingInst {
	fn default() -> Self {
		GTBookingInst::BookOutAllTradesOnDayOfExecution
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ReportToExch {
	/// Indicates the party sending message will report trade
	#[serde(rename = "N")]
	IndicatesThePartySendingMessageWillReportTrade,
	/// Indicates the party receiving message must report trade
	#[serde(rename = "Y")]
	IndicatesThePartyReceivingMessageMustReportTrade,
}

impl Default for ReportToExch {
	fn default() -> Self {
		ReportToExch::IndicatesThePartySendingMessageWillReportTrade
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TradedFlatSwitch {
	/// Not Traded Flat
	#[serde(rename = "N")]
	NotTradedFlat,
	/// Traded Flat
	#[serde(rename = "Y")]
	TradedFlat,
}

impl Default for TradedFlatSwitch {
	fn default() -> Self {
		TradedFlatSwitch::NotTradedFlat
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SettlCurrency {
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

impl Default for SettlCurrency {
	fn default() -> Self {
		SettlCurrency::Afa
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SettlCurrFxRateCalc {
	/// Multiply
	#[serde(rename = "M")]
	Multiply,
	/// Divide
	#[serde(rename = "D")]
	Divide,
}

impl Default for SettlCurrFxRateCalc {
	fn default() -> Self {
		SettlCurrFxRateCalc::Multiply
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum HandlInst {
	/// Automated execution order, private, no Broker intervention
	#[serde(rename = "1")]
	AutomatedExecutionOrderPrivateNoBrokerIntervention,
	/// Automated execution order, public, Broker intervention OK
	#[serde(rename = "2")]
	AutomatedExecutionOrderPublicBrokerInterventionOk,
	/// Manual order, best execution
	#[serde(rename = "3")]
	ManualOrderBestExecution,
}

impl Default for HandlInst {
	fn default() -> Self {
		HandlInst::AutomatedExecutionOrderPrivateNoBrokerIntervention
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

impl Default for PositionEffect {
	fn default() -> Self {
		PositionEffect::Close
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

impl Default for BookingType {
	fn default() -> Self {
		BookingType::RegularBooking
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

impl Default for MultiLegReportingType {
	fn default() -> Self {
		MultiLegReportingType::SingleSecurity
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum CancellationRights {
	/// Yes
	#[serde(rename = "Y")]
	Yes,
	/// No - execution only
	#[serde(rename = "N")]
	NoExecutionOnly,
	/// No - waiver agreement
	#[serde(rename = "M")]
	NoWaiverAgreement,
	/// No - institutional
	#[serde(rename = "O")]
	NoInstitutional,
}

impl Default for CancellationRights {
	fn default() -> Self {
		CancellationRights::Yes
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MoneyLaunderingStatus {
	/// Passed
	#[serde(rename = "Y")]
	Passed,
	/// Not Checked
	#[serde(rename = "N")]
	NotChecked,
	/// Exempt - Below The Limit
	#[serde(rename = "1")]
	ExemptBelowTheLimit,
	/// Exempt - Client Money Type Exemption
	#[serde(rename = "2")]
	ExemptClientMoneyTypeExemption,
	/// Exempt - Authorised Credit or Financial Institution
	#[serde(rename = "3")]
	ExemptAuthorisedCreditOrFinancialInstitution,
}

impl Default for MoneyLaunderingStatus {
	fn default() -> Self {
		MoneyLaunderingStatus::Passed
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ExecPriceType {
	/// Bid price
	#[serde(rename = "B")]
	BidPrice,
	/// Creation price
	#[serde(rename = "C")]
	CreationPrice,
	/// Creation price plus adjustment %
	#[serde(rename = "D")]
	CreationPricePlusAdjustment,
	/// Creation price plus adjustment amount
	#[serde(rename = "E")]
	CreationPricePlusAdjustmentAmount,
	/// Offer price
	#[serde(rename = "O")]
	OfferPrice,
	/// Offer price minus adjustment %
	#[serde(rename = "P")]
	OfferPriceMinusAdjustment,
	/// Offer price minus adjustment amount
	#[serde(rename = "Q")]
	OfferPriceMinusAdjustmentAmount,
	/// Single price
	#[serde(rename = "S")]
	SinglePrice,
}

impl Default for ExecPriceType {
	fn default() -> Self {
		ExecPriceType::BidPrice
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PriorityIndicator {
	/// Priority unchanged
	#[serde(rename = "0")]
	PriorityUnchanged,
	/// Lost Priority as result of order change
	#[serde(rename = "1")]
	LostPriorityAsResultOfOrderChange,
}

impl Default for PriorityIndicator {
	fn default() -> Self {
		PriorityIndicator::PriorityUnchanged
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LastLiquidityInd {
	/// Added Liquidity
	#[serde(rename = "1")]
	AddedLiquidity,
	/// Removed Liquidity
	#[serde(rename = "2")]
	RemovedLiquidity,
	/// Liquidity Routed Out
	#[serde(rename = "3")]
	LiquidityRoutedOut,
	/// Auction
	#[serde(rename = "4")]
	Auction,
}

impl Default for LastLiquidityInd {
	fn default() -> Self {
		LastLiquidityInd::AddedLiquidity
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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
}

impl Default for CustOrderHandlingInst {
	fn default() -> Self {
		CustOrderHandlingInst::AddOnOrder
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum OrderHandlingInstSource {
	/// NASD OATS
	#[serde(rename = "1")]
	NasdOats,
}

impl Default for OrderHandlingInstSource {
	fn default() -> Self {
		OrderHandlingInstSource::NasdOats
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum AllocSettlCurrency {
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

impl Default for AllocSettlCurrency {
	fn default() -> Self {
		AllocSettlCurrency::Afa
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum StrategyParameterType {
	/// Int
	#[serde(rename = "1")]
	Int,
	/// Length
	#[serde(rename = "2")]
	Length,
	/// NumInGroup
	#[serde(rename = "3")]
	NumInGroup,
	/// SeqNum
	#[serde(rename = "4")]
	SeqNum,
	/// TagNum
	#[serde(rename = "5")]
	TagNum,
	/// Float
	#[serde(rename = "6")]
	Float,
	/// Qty
	#[serde(rename = "7")]
	Qty,
	/// Price
	#[serde(rename = "8")]
	Price,
	/// PriceOffset
	#[serde(rename = "9")]
	PriceOffset,
	/// Amt
	#[serde(rename = "10")]
	Amt,
	/// Percentage
	#[serde(rename = "11")]
	Percentage,
	/// Char
	#[serde(rename = "12")]
	Char,
	/// Boolean
	#[serde(rename = "13")]
	Boolean,
	/// String
	#[serde(rename = "14")]
	String,
	/// MultipleCharValue
	#[serde(rename = "15")]
	MultipleCharValue,
	/// Currency
	#[serde(rename = "16")]
	Currency,
	/// Exchange
	#[serde(rename = "17")]
	Exchange,
	/// Month-Year
	#[serde(rename = "18")]
	MonthYear,
	/// UTCTimeStamp
	#[serde(rename = "19")]
	UtcTimeStamp,
	/// UTCTimeOnly
	#[serde(rename = "20")]
	UtcTimeOnly,
	/// LocalMktDate
	#[serde(rename = "21")]
	LocalMktDate,
	/// UTCDateOnly
	#[serde(rename = "22")]
	UtcDateOnly,
	/// Data
	#[serde(rename = "23")]
	Data,
	/// MultipleStringValue
	#[serde(rename = "24")]
	MultipleStringValue,
}

impl Default for StrategyParameterType {
	fn default() -> Self {
		StrategyParameterType::Int
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ContAmtType {
	/// Commission Amount (actual)
	#[serde(rename = "1")]
	N1,
	/// Commission % (actual)
	#[serde(rename = "2")]
	N2,
	/// Initial Charge Amount
	#[serde(rename = "3")]
	N3,
	/// Initial Charge %
	#[serde(rename = "4")]
	N4,
	/// Discount Amount
	#[serde(rename = "5")]
	N5,
	/// Discount %
	#[serde(rename = "6")]
	N6,
	/// Dilution Levy Amount
	#[serde(rename = "7")]
	N7,
	/// Dilution Levy %
	#[serde(rename = "8")]
	N8,
	/// Exit Charge Amount
	#[serde(rename = "9")]
	N9,
	/// Exit Charge %
	#[serde(rename = "10")]
	N10,
	/// Fund-based Renewal Commission % (a.k.a. Trail commission)
	#[serde(rename = "11")]
	N11,
	/// Projected Fund Value (i.e. for investments intended to realise or exceed a specific future value)
	#[serde(rename = "12")]
	N12,
	/// Fund-based Renewal Commission Amount (based on Order value)
	#[serde(rename = "13")]
	N13,
	/// Fund-based Renewal Commission Amount (based on Projected Fund value)
	#[serde(rename = "14")]
	N14,
	/// Net Settlement Amount
	#[serde(rename = "15")]
	N15,
}

impl Default for ContAmtType {
	fn default() -> Self {
		ContAmtType::N1
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ContAmtCurr {
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

impl Default for ContAmtCurr {
	fn default() -> Self {
		ContAmtCurr::Afa
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegSwapType {
	/// Par For Par
	#[serde(rename = "1")]
	ParForPar,
	/// Modified Duration
	#[serde(rename = "2")]
	ModifiedDuration,
	/// Risk
	#[serde(rename = "4")]
	Risk,
	/// Proceeds
	#[serde(rename = "5")]
	Proceeds,
}

impl Default for LegSwapType {
	fn default() -> Self {
		LegSwapType::ParForPar
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegAllocAcctIDSource {
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

impl Default for LegAllocAcctIDSource {
	fn default() -> Self {
		LegAllocAcctIDSource::Bic
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegAllocSettlCurrency {
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

impl Default for LegAllocSettlCurrency {
	fn default() -> Self {
		LegAllocSettlCurrency::Afa
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegPositionEffect {
	/// Close
	#[serde(rename = "C")]
	Close,
	/// FIFO
	#[serde(rename = "F")]
	Fifo,
	/// Open
	#[serde(rename = "O")]
	Open,
	/// Rolled
	#[serde(rename = "R")]
	Rolled,
	/// Close but notify on open
	#[serde(rename = "N")]
	CloseButNotifyOnOpen,
	/// Default
	#[serde(rename = "D")]
	Default,
}

impl Default for LegPositionEffect {
	fn default() -> Self {
		LegPositionEffect::Close
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegCoveredOrUncovered {
	/// Covered
	#[serde(rename = "0")]
	Covered,
	/// Uncovered
	#[serde(rename = "1")]
	Uncovered,
}

impl Default for LegCoveredOrUncovered {
	fn default() -> Self {
		LegCoveredOrUncovered::Covered
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegSettlType {
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

impl Default for LegSettlType {
	fn default() -> Self {
		LegSettlType::RegularFxSpotSettlement
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegSettlCurrency {
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

impl Default for LegSettlCurrency {
	fn default() -> Self {
		LegSettlCurrency::Afa
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegExecInst {
	/// Stay on offerside
	#[serde(rename = "0")]
	StayOnOfferside,
	/// Not held
	#[serde(rename = "1")]
	NotHeld,
	/// Work
	#[serde(rename = "2")]
	Work,
	/// Go along
	#[serde(rename = "3")]
	GoAlong,
	/// Over the day
	#[serde(rename = "4")]
	OverTheDay,
	/// Held
	#[serde(rename = "5")]
	Held,
	/// Participate don't initiate
	#[serde(rename = "6")]
	ParticipateDonTInitiate,
	/// Strict scale
	#[serde(rename = "7")]
	StrictScale,
	/// Try to scale
	#[serde(rename = "8")]
	TryToScale,
	/// Stay on bidside
	#[serde(rename = "9")]
	StayOnBidside,
	/// No cross (cross is forbidden)
	#[serde(rename = "A")]
	NoCross,
	/// OK to cross
	#[serde(rename = "B")]
	OkToCross,
	/// Call first
	#[serde(rename = "C")]
	CallFirst,
	/// Percent of volume (indicates that the sender does not want to be all of the volume on the floor vs. a specific percentage)
	#[serde(rename = "D")]
	PercentOfVolume,
	/// Do not increase - DNI
	#[serde(rename = "E")]
	DoNotIncreaseDni,
	/// Do not reduce - DNR
	#[serde(rename = "F")]
	DoNotReduceDnr,
	/// All or none - AON
	#[serde(rename = "G")]
	AllOrNoneAon,
	/// Reinstate on System Failure (mutually exclusive with Q)
	#[serde(rename = "H")]
	ReinstateOnSystemFailure,
	/// Institutions only
	#[serde(rename = "I")]
	InstitutionsOnly,
	/// Reinstate on Trading Halt (mutually exclusive with K)
	#[serde(rename = "J")]
	ReinstateOnTradingHalt,
	/// Cancel on Trading Halt (mutually exclusive with J)
	#[serde(rename = "K")]
	CancelOnTradingHalt,
	/// Last peg (last sale)
	#[serde(rename = "L")]
	LastPeg,
	/// Mid-price peg (midprice of inside quote)
	#[serde(rename = "M")]
	MidPricePeg,
	/// Non-negotiable
	#[serde(rename = "N")]
	NonNegotiable,
	/// Opening peg
	#[serde(rename = "O")]
	OpeningPeg,
	/// Market peg
	#[serde(rename = "P")]
	MarketPeg,
	/// Cancel on System Failure (mutually exclusive with H)
	#[serde(rename = "Q")]
	CancelOnSystemFailure,
	/// Primary peg (primary market - buy at bid/sell at offer)
	#[serde(rename = "R")]
	PrimaryPeg,
	/// Suspend
	#[serde(rename = "S")]
	Suspend,
	/// Fixed Peg to Local best bid or offer at time of order
	#[serde(rename = "T")]
	FixedPegToLocalBestBidOrOfferAtTimeOfOrder,
	/// Customer Display Instruction (Rule11Ac1-1/4)
	#[serde(rename = "U")]
	CustomerDisplayInstruction,
	/// Netting (for Forex)
	#[serde(rename = "V")]
	Netting,
	/// Peg to VWAP
	#[serde(rename = "W")]
	PegToVwap,
	/// Trade Along
	#[serde(rename = "X")]
	TradeAlong,
	/// Try to Stop
	#[serde(rename = "Y")]
	TryToStop,
	/// Cancel if Not Best
	#[serde(rename = "Z")]
	CancelIfNotBest,
	/// Trailing Stop Peg
	#[serde(rename = "a")]
	TrailingStopPeg,
	/// Strict Limit (No Price Improvement)
	#[serde(rename = "b")]
	StrictLimit,
	/// Ignore Price Validity Checks
	#[serde(rename = "c")]
	IgnorePriceValidityChecks,
	/// Peg to Limit Price
	#[serde(rename = "d")]
	PegToLimitPrice,
	/// Work to Target Strategy
	#[serde(rename = "e")]
	WorkToTargetStrategy,
	/// Intermarket Sweep
	#[serde(rename = "f")]
	IntermarketSweep,
	/// External Routing Allowed
	#[serde(rename = "g")]
	ExternalRoutingAllowed,
	/// External Routing Not Allowed
	#[serde(rename = "h")]
	ExternalRoutingNotAllowed,
	/// Imbalance Only
	#[serde(rename = "i")]
	ImbalanceOnly,
	/// Single execution requested for block trade
	#[serde(rename = "j")]
	SingleExecutionRequestedForBlockTrade,
	/// Best Execution
	#[serde(rename = "k")]
	BestExecution,
	/// Suspend on system failure (mutually exclusive with H and Q)
	#[serde(rename = "l")]
	SuspendOnSystemFailure,
	/// Suspend on Trading Halt (mutually exclusive with J and K)
	#[serde(rename = "m")]
	SuspendOnTradingHalt,
	/// Reinstate on connection loss (mutually exclusive with o and p)
	#[serde(rename = "n")]
	ReinstateOnConnectionLoss,
	/// Cancel on connection loss (mutually exclusive with n and p)
	#[serde(rename = "o")]
	CancelOnConnectionLoss,
	/// Suspend on connection loss (mutually exclusive with n and o)
	#[serde(rename = "p")]
	SuspendOnConnectionLoss,
	/// Release from suspension (mutually exclusive with S)
	#[serde(rename = "q")]
	ReleaseFromSuspension,
	/// Execute as delta neutral using volatility provided
	#[serde(rename = "r")]
	ExecuteAsDeltaNeutralUsingVolatilityProvided,
	/// Execute as duration neutral
	#[serde(rename = "s")]
	ExecuteAsDurationNeutral,
	/// Execute as FX neutral
	#[serde(rename = "t")]
	ExecuteAsFxNeutral,
}

impl Default for LegExecInst {
	fn default() -> Self {
		LegExecInst::StayOnOfferside
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MiscFeeCurr {
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

impl Default for MiscFeeCurr {
	fn default() -> Self {
		MiscFeeCurr::Afa
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MiscFeeType {
	/// Regulatory (e.g. SEC)
	#[serde(rename = "1")]
	Regulatory,
	/// Tax
	#[serde(rename = "2")]
	Tax,
	/// Local Commission
	#[serde(rename = "3")]
	LocalCommission,
	/// Exchange Fees
	#[serde(rename = "4")]
	ExchangeFees,
	/// Stamp
	#[serde(rename = "5")]
	Stamp,
	/// Levy
	#[serde(rename = "6")]
	Levy,
	/// Other
	#[serde(rename = "7")]
	Other,
	/// Markup
	#[serde(rename = "8")]
	Markup,
	/// Consumption Tax
	#[serde(rename = "9")]
	ConsumptionTax,
	/// Per transaction
	#[serde(rename = "10")]
	PerTransaction,
	/// Conversion
	#[serde(rename = "11")]
	Conversion,
	/// Agent
	#[serde(rename = "12")]
	Agent,
	/// Transfer Fee
	#[serde(rename = "13")]
	TransferFee,
	/// Security Lending
	#[serde(rename = "14")]
	SecurityLending,
}

impl Default for MiscFeeType {
	fn default() -> Self {
		MiscFeeType::Regulatory
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MiscFeeBasis {
	/// Absolute
	#[serde(rename = "0")]
	Absolute,
	/// Per Unit
	#[serde(rename = "1")]
	PerUnit,
	/// Percentage
	#[serde(rename = "2")]
	Percentage,
}

impl Default for MiscFeeBasis {
	fn default() -> Self {
		MiscFeeBasis::Absolute
	}
}
