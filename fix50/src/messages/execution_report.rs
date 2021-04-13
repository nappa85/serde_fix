
use serde::{Deserialize, Serialize};

use fix_common::{ApplVerID, Boolean, Currency, EncodedText, LocalMktDate, RepeatingValues, SeparatedValues, UTCTimestamp, FixVersion};
use crate::underlying::Underlying;
use fix50sp2::{commission_data::CommissionData, discretion_instructions::DiscretionInstructions, display_instruction::DisplayInstruction, financing_details::FinancingDetails, instrument::Instrument, order_qty_data::OrderQtyData, parties::Parties, peg_instructions::PegInstructions, spread_or_benchmark_curve_data::SpreadOrBenchmarkCurveData, stipulations::Stipulations, trd_reg_timestamps::TrdRegTimestamps, triggering_instruction::TriggeringInstruction, yield_data::YieldData};
use fixt11::{Trailer, header::{HasHeader, Header, MsgType}};

//// MsgType = 8
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExecutionReport {
	/// Standard Message Header
	#[serde(flatten)]
	pub header: Header,
	/// OrderID is required to be unique for each chain of orders.
	#[serde(rename = "37")]
	pub order_id: String,
	/// Can be used to provide order id used by exchange or executing system.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "198")]
	pub secondary_order_id: Option<String>,
	/// SecondaryClOrdID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "526")]
	pub secondary_cl_ord_id: Option<String>,
	/// SecondaryExecID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "527")]
	pub secondary_exec_id: Option<String>,
	/// Required for executions against electronically submitted orders which were assigned an ID by the institution or intermediary.
	/// Not required for orders manually entered by the broker or fund manager (for CIV orders).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "11")]
	pub cl_ord_id: Option<String>,
	/// Conditionally required for response to an electronic Cancel or Cancel/Replace request (ExecType=PendingCancel, Replace, or
	/// Canceled). <a href="tag_11_ClOrdID_.html">ClOrdID&nbsp;(11)</a> of the previous accepted order (NOT the initial order of the day) when canceling or replacing an order.
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
	/// Required if responding to and if provided on the <a href="message_Order_Status_Request_H_.html">Order Status Request&nbsp;(H)</a> message. Echo back the value provided by the requester.
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
	/// Can be used when responding to an <a href="message_Order_Mass_Status_Request_AF_.html">Order Mass Status Request&nbsp;(AF)</a> to identify the total number of <a href="message_Execution_Report_8_.html">Execution Reports&nbsp;(8)</a> which will be returned.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "911")]
	pub tot_num_reports: Option<i32>,
	/// Can be used when responding to an <a href="message_Order_Mass_Status_Request_AF_.html">Order Mass Status Request&nbsp;(AF)</a> to indicate that this is the last <a href="message_Execution_Report_8_.html">Execution Reports&nbsp;(8)</a> which will be returned as a result of the request.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "912")]
	pub last_rpt_requested: Option<LastRptRequested>,
	/// Insert here the set of "Parties" (firm identification) fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub parties: Option<Parties>,
	/// TradeOriginationDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "229")]
	pub trade_origination_date: Option<LocalMktDate>,
	/// Number of ContraBrokers repeating group instances.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "382")]
	pub contra_brokers: Option<RepeatingValues<ContraBroker>>,
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
	/// Unique identifier of execution message as assigned by sell-side (broker, exchange, ECN) (will be 0 (zero) forExecType=I (Order
	/// Status)).
	#[serde(rename = "17")]
	pub exec_id: String,
	/// Required for Trade Cancel and Trade Correct <a href="tag_150_ExecType_.html">ExecType&nbsp;(150)</a> messages
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "19")]
	pub exec_ref_id: Option<String>,
	/// Describes the purpose of the execution report.
	#[serde(rename = "150")]
	pub exec_type: ExecType,
	/// Describes the current state of a CHAIN of orders, same scope as OrderQty, CumQty, LeavesQty, and AvgPx
	#[serde(rename = "39")]
	pub ord_status: OrdStatus,
	/// For optional use with <a href="tag_39_OrdStatus_.html">OrdStatus&nbsp;(39)</a> = 0 (New)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "636")]
	pub working_indicator: Option<WorkingIndicator>,
	/// For optional use with <a href="tag_150_ExecType_.html">ExecType&nbsp;(150)</a> = 8 (Rejected)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "103")]
	pub ord_rej_reason: Option<OrdRejReason>,
	/// Required for <a href="tag_150_ExecType_.html">ExecType&nbsp;(150)</a> = D (Restated).
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
	/// SettlType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "63")]
	pub settl_type: Option<SettlType>,
	/// Takes precedence over <a href="tag_63_SettlType_.html">SettlType&nbsp;(63)</a> value and conditionally required/omitted for specific <a href="tag_63_SettlType_.html">SettlType&nbsp;(63)</a> values.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "64")]
	pub settl_date: Option<LocalMktDate>,
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
	pub instrument: Instrument,
	/// Insert here the set of "FinancingDetails" fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub financing_details: Option<FinancingDetails>,
	/// Number of underlyings
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "711")]
	pub underlyings: Option<RepeatingValues<Underlying>>,
	/// Side
	#[serde(rename = "54")]
	pub side: Side,
	/// Insert here the set of "Stipulations" (repeating group of Fixed Income stipulations) fields defined in "Common Components
	/// of Application Messages".
	#[serde(flatten)]
	pub stipulations: Option<Stipulations>,
	/// QtyType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "854")]
	pub qty_type: Option<QtyType>,
	/// Insert here the set of "OrderQtyData" fields defined in "Common Components of Application Messages". **IMPORTANT NOTE: <a href="tag_38_OrderQty_.html">OrderQty&nbsp;(38)</a> field is required for Single <a href="block_Instrument_.html">Instrument</a> Orders unless rejecting or acknowledging an order for a <a href="tag_152_CashOrderQty_.html">CashOrderQty&nbsp;(152)</a> or PercentOrder. **
	#[serde(flatten)]
	pub order_qty_data: Option<OrderQtyData>,
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
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "44")]
	pub price: Option<f64>,
	/// PriceProtectionScope
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1092")]
	pub price_protection_scope: Option<PriceProtectionScope>,
	/// Required if specified on the order
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "99")]
	pub stop_px: Option<f64>,
	/// Insert here the set of "TriggeringInstruction" fields defined in "common components of application messages".
	#[serde(flatten)]
	pub triggering_instruction: Option<TriggeringInstruction>,
	/// Insert here the set of "PegInstruction" fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub peg_instructions: Option<PegInstructions>,
	/// Insert here the set of "DiscretionInstruction" fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub discretion_instructions: Option<DiscretionInstructions>,
	/// The current price the order is pegged at
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "839")]
	pub pegged_price: Option<f64>,
	/// The reference price of a pegged order.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1095")]
	pub pegged_ref_price: Option<f64>,
	/// The current discretionary price of the order
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
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
	pub strategy_parameters: Option<RepeatingValues<StrategyParameter>>,
	/// (Deprecated in FIX.5.0)For further specification of the TargetStrategy
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "848")]
	pub target_strategy_parameters: Option<String>,
	/// (Deprecated in FIX.5.0) Mandatory for a TargetStrategy=Participate order and specifies the target particpation rate. For other
	/// order types optionally specifies a volume limit (i.e. do not be more than this percent of the market volume)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "849")]
	pub participation_rate: Option<f32>,
	/// For communication of the performance of the order versus the target strategy
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
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
	pub effective_time: Option<UTCTimestamp>,
	/// Conditionally required if <a href="tag_59_TimeInForce_.html">TimeInForce&nbsp;(59)</a> = GTD and <a href="tag_126_ExpireTime_.html">ExpireTime&nbsp;(126)</a> is not specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "432")]
	pub expire_date: Option<LocalMktDate>,
	/// Conditionally required if <a href="tag_59_TimeInForce_.html">TimeInForce&nbsp;(59)</a> = GTD and <a href="tag_432_ExpireDate_.html">ExpireDate&nbsp;(432)</a> is not specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "126")]
	pub expire_time: Option<UTCTimestamp>,
	/// Can contain multiple instructions, space delimited.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "18")]
	pub exec_inst: Option<SeparatedValues<ExecInst>>,
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
	pub order_restrictions: Option<SeparatedValues<OrderRestrictions>>,
	/// PreTradeAnonymity
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1091")]
	pub pre_trade_anonymity: Option<Boolean>,
	/// CustOrderCapacity
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "582")]
	pub cust_order_capacity: Option<CustOrderCapacity>,
	/// Quantity (e.g. shares) bought/sold on this (last) fill. Required if <a href="tag_150_ExecType_.html">ExecType&nbsp;(150)</a> = Trade or Trade Correct. If ExecType=Stopped, represents the quantity stopped/guaranteed/protected for.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "32")]
	pub last_qty: Option<f64>,
	/// Used for FX trades to express the quantity or amount of the other side of the currency. Conditionally required if <a href="tag_150_ExecType_.html">ExecType&nbsp;(150)</a> = Trade or Trade Correct and is an FX trade.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1056")]
	pub calculated_ccy_last_qty: Option<f64>,
	/// Optionally used when <a href="tag_150_ExecType_.html">ExecType&nbsp;(150)</a> = Trade or Trade Correct and is a FX Swap trade. Used to express the swap points for the swap trade event.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1071")]
	pub last_swap_points: Option<f64>,
	/// UnderlyingLastQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "652")]
	pub underlying_last_qty: Option<f64>,
	/// Price of this (last) fill. Required if <a href="tag_150_ExecType_.html">ExecType&nbsp;(150)</a> = Trade or Trade Correct. Should represent the "all-in" (LastSpotRate + LastForwardPoints) rate for F/X orders. If ExecType=Stopped,
	/// represents the price stopped/guaranteed/protected at. Not required for FX Swap when <a href="tag_150_ExecType_.html">ExecType&nbsp;(150)</a> = Trade or Trade Correct as there is no "all-in" rate that applies to both legs of the FX Swap.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "31")]
	pub last_px: Option<f64>,
	/// UnderlyingLastPx
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "651")]
	pub underlying_last_px: Option<f64>,
	/// Last price expressed in percent-of-par. Conditionally required for Fixed Income trades when <a href="tag_31_LastPx_.html">LastPx&nbsp;(31)</a> is expressed in Yield, Spread, Discount or any other price type that is not percent-of-par.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "669")]
	pub last_par_px: Option<f64>,
	/// Applicable for F/X orders
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "194")]
	pub last_spot_rate: Option<f64>,
	/// Applicable for F/X orders
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "195")]
	pub last_forward_points: Option<f64>,
	/// If <a href="tag_150_ExecType_.html">ExecType&nbsp;(150)</a> = Trade (F), indicates the market where the trade was executed. If <a href="tag_150_ExecType_.html">ExecType&nbsp;(150)</a> = New (0), indicates the market where the order was routed.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "30")]
	pub last_mkt: Option<String>,
	/// TradingSessionID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "336")]
	pub trading_session_id: Option<String>,
	/// TradingSessionSubID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "625")]
	pub trading_session_sub_id: Option<String>,
	/// TimeBracket
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "943")]
	pub time_bracket: Option<String>,
	/// LastCapacity
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "29")]
	pub last_capacity: Option<LastCapacity>,
	/// Quantity open for further execution. If the <a href="tag_39_OrdStatus_.html">OrdStatus&nbsp;(39)</a> is Canceled, DoneForTheDay, Expired, Calculated, or Rejected (in which case the order is no longer active) then <a href="tag_151_LeavesQty_.html">LeavesQty&nbsp;(151)</a> could be 0, otherwise <a href="tag_151_LeavesQty_.html">LeavesQty&nbsp;(151)</a> = <a href="tag_38_OrderQty_.html">OrderQty&nbsp;(38)</a> - CumQty.
	#[serde(deserialize_with = "crate::entities::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "151")]
	pub leaves_qty: f64,
	/// Currently executed quantity for chain of orders.
	#[serde(deserialize_with = "crate::entities::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "14")]
	pub cum_qty: f64,
	/// Not required for markets where average price is not calculated by the market. Conditionally required otherwise.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "6")]
	pub avg_px: Option<f64>,
	/// For GT orders on days following the day of the first trade.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "424")]
	pub day_order_qty: Option<f64>,
	/// For GT orders on days following the day of the first trade.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "425")]
	pub day_cum_qty: Option<f64>,
	/// For GT orders on days following the day of the first trade.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "426")]
	pub day_avg_px: Option<f64>,
	/// States whether executions are booked out or accumulated on a partially filled GT order
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "427")]
	pub gt_booking_inst: Option<GTBookingInst>,
	/// Used when reporting other than current day trades.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "75")]
	pub trade_date: Option<LocalMktDate>,
	/// Time the transaction represented by this ExecutionReport occurred
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<UTCTimestamp>,
	/// ReportToExch
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "113")]
	pub report_to_exch: Option<ReportToExch>,
	/// Insert here the set of "CommissionData" fields defined in "Common Components of Application Messages". Note: On a fill/partial
	/// fill messages, it represents value for that fill/partial fill. On ExecType=Calculated, it represents cumulative value for
	/// the order. Monetary commission values are expressed in the currency reflected by the <a href="tag_15_Currency_.html">Currency&nbsp;(15)</a> field.
	#[serde(flatten)]
	pub commission_data: Option<CommissionData>,
	/// Insert here the set of "SpreadOrBenchmarkCurveData" (Fixed Income spread or benchmark curve) fields defined in "Common Components
	/// of Application Messages".
	#[serde(flatten)]
	pub spread_or_benchmark_curve_data: Option<SpreadOrBenchmarkCurveData>,
	/// Insert here the set of "YieldData" (yield-related) fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub yield_data: Option<YieldData>,
	/// GrossTradeAmt
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "381")]
	pub gross_trade_amt: Option<f64>,
	/// NumDaysInterest
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "157")]
	pub num_days_interest: Option<i32>,
	/// ExDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "230")]
	pub ex_date: Option<LocalMktDate>,
	/// AccruedInterestRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "158")]
	pub accrued_interest_rate: Option<f32>,
	/// AccruedInterestAmt
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "159")]
	pub accrued_interest_amt: Option<f64>,
	/// For fixed income products which pay lump-sum interest at maturity.
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
	/// TradedFlatSwitch
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "258")]
	pub traded_flat_switch: Option<TradedFlatSwitch>,
	/// BasisFeatureDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "259")]
	pub basis_feature_date: Option<LocalMktDate>,
	/// BasisFeaturePrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "260")]
	pub basis_feature_price: Option<f64>,
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
	/// Note: On a fill/partial fill messages, it represents value for that fill/partial fill, on ExecType=Calculated, it represents
	/// cumulative value for the order. Value expressed in the currency reflected by the <a href="tag_15_Currency_.html">Currency&nbsp;(15)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "118")]
	pub net_money: Option<f64>,
	/// Used to report results of forex accommodation trade
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "119")]
	pub settl_curr_amt: Option<f64>,
	/// Used to report results of forex accommodation trade
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "120")]
	pub settl_currency: Option<Currency>,
	/// Foreign exchange rate used to compute <a href="tag_119_SettlCurrAmt_.html">SettlCurrAmt&nbsp;(119)</a> from <a href="tag_15_Currency_.html">Currency&nbsp;(15)</a> to SettlCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "155")]
	pub settl_curr_fx_rate: Option<f64>,
	/// Specifies whether the <a href="tag_155_SettlCurrFxRate_.html">SettlCurrFxRate&nbsp;(155)</a> should be multiplied or divided
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "156")]
	pub settl_curr_fx_rate_calc: Option<SettlCurrFxRateCalc>,
	/// HandlInst
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "21")]
	pub handl_inst: Option<HandlInst>,
	/// MinQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "110")]
	pub min_qty: Option<f64>,
	/// MatchIncrement
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1089")]
	pub match_increment: Option<f64>,
	/// MaxPriceLevels
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1090")]
	pub max_price_levels: Option<i32>,
	/// Insert here the set of "DisplayInstruction" fields defined in "common components of application messages".
	#[serde(flatten)]
	pub display_instruction: Option<DisplayInstruction>,
	/// MaxFloor
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "111")]
	pub max_floor: Option<f64>,
	/// For use in derivatives omnibus accounting
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "77")]
	pub position_effect: Option<PositionEffect>,
	/// (Deprecated in FIX.5.0)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
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
	/// Must be set if <a href="tag_355_EncodedText_.html">EncodedText&nbsp;(355)</a> field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	#[serde(rename = "354")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_58_Text_.html">Text&nbsp;(58)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding_.html">MessageEncoding&nbsp;(347)</a> field.
	#[serde(alias = "355")]
	pub encoded_text: Option<EncodedText<355>>,
	/// (Deprecated in FIX.5.0)Can be used with <a href="tag_40_OrdType_.html">OrdType&nbsp;(40)</a> = "Forex - Swap" to specify the "value date" for the future portion of a F/X swap.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "193")]
	pub settl_date_2: Option<LocalMktDate>,
	/// (Deprecated in FIX.5.0)Can be used with <a href="tag_40_OrdType_.html">OrdType&nbsp;(40)</a> = "Forex - Swap" to specify the order quantity for the future portion of a F/X swap.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "192")]
	pub order_qty_2: Option<f64>,
	/// Can be used with <a href="tag_40_OrdType_.html">OrdType&nbsp;(40)</a> = "Forex - Swap" to specify the forward points (added to LastSpotRate) for the future portion of a F/X swap.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
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
	/// Reference to <a href="message_Registration_Instructions_o_.html">Registration Instructions&nbsp;(o)</a> message for this Order.
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
	pub trans_bkd_time: Option<UTCTimestamp>,
	/// For CIV - Optional
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "515")]
	pub exec_valuation_point: Option<UTCTimestamp>,
	/// For CIV - Optional
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "484")]
	pub exec_price_type: Option<ExecPriceType>,
	/// For CIV - Optional
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "485")]
	pub exec_price_adjustment: Option<f64>,
	/// PriorityIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "638")]
	pub priority_indicator: Option<PriorityIndicator>,
	/// PriceImprovement
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "639")]
	pub price_improvement: Option<f64>,
	/// Applicable only on <a href="tag_39_OrdStatus_.html">OrdStatus&nbsp;(39)</a> of Partial or Filled.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "851")]
	pub last_liquidity_ind: Option<LastLiquidityInd>,
	/// Number of contract details in this message (number of repeating groups to follow)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "518")]
	pub cont_amts: Option<RepeatingValues<ContAmt>>,
	/// Number of legs Identifies a Multi-leg Execution if present and non-zero.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "555")]
	pub legs: Option<RepeatingValues<Leg>>,
	/// CopyMsgIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "797")]
	pub copy_msg_indicator: Option<Boolean>,
	/// Required if any miscellaneous fees are reported. Indicates number of repeating entries. Repeating group. ** Nested Repeating
	/// Group follows **
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "136")]
	pub misc_fees: Option<RepeatingValues<MiscFee>>,
	/// ManualOrderIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1028")]
	pub manual_order_indicator: Option<Boolean>,
	/// CustDirectedOrder
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1029")]
	pub cust_directed_order: Option<Boolean>,
	/// ReceivedDeptID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1030")]
	pub received_dept_id: Option<String>,
	/// CustOrderHandlingInst
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1031")]
	pub cust_order_handling_inst: Option<SeparatedValues<CustOrderHandlingInst>>,
	/// OrderHandlingInstSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1032")]
	pub order_handling_inst_source: Option<OrderHandlingInstSource>,
	/// TrdRegTimestamps
	#[serde(flatten)]
	pub trd_reg_timestamps: Option<TrdRegTimestamps>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub trailer: Trailer,
}

impl ExecutionReport {
    pub fn new() -> Self {
        ExecutionReport {
            header: Header {
                begin_string: Some(FixVersion::FIXT11),
                msg_type: Some(MsgType::ExecutionReport),
                appl_ver_id: Some(ApplVerID::FIX50),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl HasHeader for ExecutionReport {
    fn get_header(&self) -> &Header {
        &self.header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.header
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ContraBroker {
	/// First field in repeating group. Required if <a href="tag_382_NoContraBrokers_.html">NoContraBrokers&nbsp;(382)</a> &gt; 0.
	#[serde(rename = "375")]
	pub contra_broker: Option<String>,
	/// ContraTrader
	#[serde(rename = "337")]
	pub contra_trader: Option<String>,
	/// ContraTradeQty
	#[serde(rename = "437")]
	pub contra_trade_qty: Option<f64>,
	/// ContraTradeTime
	#[serde(rename = "438")]
	pub contra_trade_time: Option<UTCTimestamp>,
	/// ContraLegRefID
	#[serde(rename = "655")]
	pub contra_leg_ref_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StrategyParameter {
	/// Name of parameter
	#[serde(rename = "958")]
	pub strategy_parameter_name: Option<String>,
	/// Datatype of the parameter.
	#[serde(rename = "959")]
	pub strategy_parameter_type: Option<StrategyParameterType>,
	/// Value of the parameter
	#[serde(rename = "960")]
	pub strategy_parameter_value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ContAmt {
	/// Must be first field in the repeating group.
	#[serde(rename = "519")]
	pub cont_amt_type: Option<ContAmtType>,
	/// ContAmtValue
	#[serde(rename = "520")]
	pub cont_amt_value: Option<f64>,
	/// ContAmtCurr
	#[serde(rename = "521")]
	pub cont_amt_curr: Option<Currency>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Leg {
	/// LegQty
	#[serde(rename = "687")]
	pub leg_qty: Option<f64>,
	/// When reporting an Execution, <a href="tag_685_LegOrderQty_.html">LegOrderQty&nbsp;(685)</a> may be used on Execution Report to echo back original <a href="tag_685_LegOrderQty_.html">LegOrderQty&nbsp;(685)</a> submission. This field should be used to specify <a href="tag_38_OrderQty_.html">OrderQty&nbsp;(38)</a> at the leg level rather than <a href="tag_687_LegQty_.html">LegQty&nbsp;(687)</a> (deprecated).
	#[serde(rename = "685")]
	pub leg_order_qty: Option<f64>,
	/// Instead of <a href="tag_687_LegQty_.html">LegQty&nbsp;(687)</a> requests that the sellside calculate <a href="tag_687_LegQty_.html">LegQty&nbsp;(687)</a> based on opposite Leg
	#[serde(rename = "690")]
	pub leg_swap_type: Option<LegSwapType>,
	/// Provide if the <a href="tag_77_PositionEffect_.html">PositionEffect&nbsp;(77)</a> for the leg is different from that specified for the overall multileg security
	#[serde(rename = "564")]
	pub leg_position_effect: Option<LegPositionEffect>,
	/// Provide if the <a href="tag_203_CoveredOrUncovered_.html">CoveredOrUncovered&nbsp;(203)</a> for the leg is different from that specified for the overall multileg security.
	#[serde(rename = "565")]
	pub leg_covered_or_uncovered: Option<LegCoveredOrUncovered>,
	/// Used to identify a specific leg.
	#[serde(rename = "654")]
	pub leg_ref_id: Option<String>,
	/// Provide only if a <a href="tag_44_Price_.html">Price&nbsp;(44)</a> is required for a specific leg. Used for anchoring the overall multileg security price to a specific leg Price.
	#[serde(rename = "566")]
	pub leg_price: Option<f64>,
	/// LegSettlType
	#[serde(rename = "587")]
	pub leg_settl_type: Option<LegSettlType>,
	/// Takes precedence over <a href="tag_587_LegSettlType_.html">LegSettlType&nbsp;(587)</a> value and conditionally required/omitted for specific <a href="tag_587_LegSettlType_.html">LegSettlType&nbsp;(587)</a> values.
	#[serde(rename = "588")]
	pub leg_settl_date: Option<LocalMktDate>,
	/// Used to report the execution price assigned to the leg of the multileg instrument
	#[serde(rename = "637")]
	pub leg_last_px: Option<f64>,
	/// LegSettlCurrency
	#[serde(rename = "675")]
	pub leg_settl_currency: Option<Currency>,
	/// LegLastForwardPoints
	#[serde(rename = "1073")]
	pub leg_last_forward_points: Option<f64>,
	/// LegCalculatedCcyLastQty
	#[serde(rename = "1074")]
	pub leg_calculated_ccy_last_qty: Option<f64>,
	/// For FX Futures can be used to express the notional value of a trade when LegLastQty and other quantity fields are expressed
	/// in terms of number of contracts - <a href="tag_614_LegContractMultiplier_.html">LegContractMultiplier&nbsp;(614)</a> (231) is required in this case.
	#[serde(rename = "1075")]
	pub leg_gross_trade_amt: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MiscFee {
	/// Required if <a href="tag_136_NoMiscFees_.html">NoMiscFees&nbsp;(136)</a> &gt; 0
	#[serde(rename = "137")]
	pub misc_fee_amt: Option<f64>,
	/// MiscFeeCurr
	#[serde(rename = "138")]
	pub misc_fee_curr: Option<Currency>,
	/// Required if <a href="tag_136_NoMiscFees_.html">NoMiscFees&nbsp;(136)</a> &gt; 0
	#[serde(rename = "139")]
	pub misc_fee_type: Option<MiscFeeType>,
	/// MiscFeeBasis
	#[serde(rename = "891")]
	pub misc_fee_basis: Option<MiscFeeBasis>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LastRptRequested {
	/// Not last message
	#[serde(rename = "N")]
	NotLastMessage,
	/// Last message
	#[serde(rename = "Y")]
	LastMessage,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CrossType {
	/// Cross AON - cross tade which is executed complete or not. Both sides are treated in the same manner. This is equivalent to
	/// an "All or None".
	#[serde(rename = "1")]
	CrossAon,
	/// Cross IOC - cross trade which is executed partially and the rest is cancelled. One side is fully executed, the other side
	/// is partially executed with the remainder being cancelled. This is equivalent to an IOC on the other side. Note: <a href="tag_550_CrossPrioritization_.html">CrossPrioritization&nbsp;(550)</a> field may be used to indicate which side should fully execute in this scenario.
	#[serde(rename = "2")]
	CrossIoc,
	/// Cross One Side - cross trade which is partially executed with the unfilled portions remaining active. One side of the corss
	/// is fully executed (as denoted by the <a href="tag_550_CrossPrioritization_.html">CrossPrioritization&nbsp;(550)</a> field), but the unfilled portion remains active.
	#[serde(rename = "3")]
	CrossOneSide,
	/// Cross Same Price - cross trade is executed with existing orders with the same price. In this case other orders exist with
	/// the same price, the quantity of the Cross is executed against the existing orders and quotes, the remainder of the corss is
	/// executed against the other side of the cross. The two sides potentially have different quantities.
	#[serde(rename = "4")]
	CrossSamePrice,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
	/// Restated (Execution Report sent unsolicited by sellside, with <a href="tag_378_ExecRestatementReason_.html">ExecRestatementReason&nbsp;(378)</a> set)
	#[serde(rename = "D")]
	Restated,
	/// Pending Replace (e.g. result of <a href="message_Order_Cancel_Replace_Request_G_.html">Order Cancel/Replace Request&nbsp;(G)</a> )
	#[serde(rename = "E")]
	PendingReplace,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum WorkingIndicator {
	/// Order has been accepted but not yet in a working state
	#[serde(rename = "N")]
	OrderHasBeenAcceptedButNotYetInAWorkingState,
	/// Order is currently being worked
	#[serde(rename = "Y")]
	OrderIsCurrentlyBeingWorked,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OrdRejReason {
	/// Broker / Exchange option
	#[serde(rename = "0")]
	Broker,
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
	UnknownAccountS,
	/// Invalid price increment
	#[serde(rename = "18")]
	InvalidPriceIncrement,
	/// Other
	#[serde(rename = "99")]
	Other,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExecRestatementReason {
	/// GT Corporate action
	#[serde(rename = "0")]
	GtCorporateAction,
	/// GT renewal / restatement (no corporate action)
	#[serde(rename = "1")]
	GtRenewal,
	/// Verbal change
	#[serde(rename = "2")]
	VerbalChange,
	/// Repricing of order
	#[serde(rename = "3")]
	RepricingOfOrder,
	/// Broker option
	#[serde(rename = "4")]
	BrokerOption,
	/// Partial decline of <a href="tag_38_OrderQty_.html">OrderQty&nbsp;(38)</a> (e.g. exchange-initiated partial cancel)
	#[serde(rename = "5")]
	PartialDeclineOf,
	/// Cancel on Trading Halt
	#[serde(rename = "6")]
	CancelOnTradingHalt,
	/// Cancel on System Failure
	#[serde(rename = "7")]
	CancelOnSystemFailure,
	/// Market (Exchange) Option
	#[serde(rename = "8")]
	Market,
	/// Canceled, Not Best
	#[serde(rename = "9")]
	Canceled,
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
	/// Other (custom or proprietary)
	#[serde(rename = "99")]
	Other,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BookingUnit {
	/// Each partial execution is a bookable unit
	#[serde(rename = "0")]
	EachPartialExecutionIsABookableUnit,
	/// Aggregate partial executions on this order, and book one trade per order
	#[serde(rename = "1")]
	AggregatePartialExecutionsOnThisOrder,
	/// Aggregate executions for this symbol, side, and settlement date
	#[serde(rename = "2")]
	AggregateExecutionsForThisSymbol,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PreallocMethod {
	/// Pro-rata
	#[serde(rename = "0")]
	ProRata,
	/// Do not pro-rata - discuss first
	#[serde(rename = "1")]
	DoNotProRata,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SettlType {
	/// Regular / FX Spot settlement (T+1 or T+2 depending on currency)
	#[serde(rename = "0")]
	Regular,
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
	/// Broken date - for FX expressing non-standard tenor, <a href="tag_64_SettlDate_.html">SettlDate&nbsp;(64)</a> must be specified
	#[serde(rename = "B")]
	BrokenDate,
	/// FX Spot Next settlement (Spot+1, aka next day)
	#[serde(rename = "C")]
	FxSpotNextSettlement,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MatchType {
	/// One-Party Privately Negotiated Trade Report
	#[serde(rename = "60")]
	OnePartyPrivatelyNegotiatedTradeReport,
	/// Two-Party Privately Negotiated Trade Report
	#[serde(rename = "61")]
	TwoPartyPrivatelyNegotiatedTradeReport,
	/// Continuous Auto-match
	#[serde(rename = "62")]
	ContinuousAutoMatch,
	/// Cross Auction
	#[serde(rename = "63")]
	CrossAuction,
	/// Counter-Order Selection
	#[serde(rename = "64")]
	CounterOrderSelection,
	/// Call Auction
	#[serde(rename = "65")]
	CallAuction,
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
	ExactMatchOnTradeDate1,
	/// Exact match on Trade Date, Stock Symbol, Quantity, Price, Trade Type, and Special Trade Indicator plus four badges
	#[serde(rename = "A2")]
	ExactMatchOnTradeDate2,
	/// Exact match on Trade Date, Stock Symbol, Quantity, Price, Trade Type, and Special Trade Indicator plus two badges and execution
	/// time (within two-minute window)
	#[serde(rename = "A3")]
	ExactMatchOnTradeDate3,
	/// Exact match on Trade Date, Stock Symbol, Quantity, Price, Trade Type, and Special Trade Indicator plus two badges
	#[serde(rename = "A4")]
	ExactMatchOnTradeDate4,
	/// Exact match on Trade Date, Stock Symbol, Quantity, Price, Trade Type, and Special Trade Indicator plus execution time (within
	/// two-minute window)
	#[serde(rename = "A5")]
	ExactMatchOnTradeDate5,
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
	ExactMatchOnTradeDate,
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
	CrossAuction5,
	/// Counter-Order Selection
	#[serde(rename = "6")]
	CounterOrderSelection6,
	/// Call Auction
	#[serde(rename = "7")]
	CallAuction7,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
	Gim,
	/// Lessee 106.F Employees
	#[serde(rename = "L")]
	Lessee106FEmployees,
	/// All other ownership types
	#[serde(rename = "M")]
	AllOtherOwnershipTypes,
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
pub enum QtyType {
	/// Units (shares, par, currency)
	#[serde(rename = "0")]
	Units,
	/// Contracts (if used - must specify <a href="tag_231_ContractMultiplier_.html">ContractMultiplier&nbsp;(231)</a> )
	#[serde(rename = "1")]
	Contracts,
	/// Units of Measure per Time Unit (if used - must specify <a href="tag_996_UnitofMeasure_.html">UnitofMeasure&nbsp;(996)</a> and <a href="tag_997_TimeUnit_.html">TimeUnit&nbsp;(997)</a> )
	#[serde(rename = "2")]
	UnitsOfMeasurePerTimeUnit,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OrdType {
	/// Market
	#[serde(rename = "1")]
	Market,
	/// Limit
	#[serde(rename = "2")]
	Limit,
	/// Stop / Stop Loss
	#[serde(rename = "3")]
	Stop,
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
	Discount,
	/// Premium - percentage points over par
	#[serde(rename = "5")]
	Premium,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SolicitedFlag {
	/// Was not solicited
	#[serde(rename = "N")]
	WasNotSolicited,
	/// Was solicited
	#[serde(rename = "Y")]
	WasSolicited,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
	DoNotIncrease,
	/// Do not reduce - DNR
	#[serde(rename = "F")]
	DoNotReduce,
	/// All or none - AON
	#[serde(rename = "G")]
	AllOrNone,
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
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AggressorIndicator {
	/// Order initiator is aggressor
	#[serde(rename = "Y")]
	OrderInitiatorIsAggressor,
	/// Order initiator is passive
	#[serde(rename = "N")]
	OrderInitiatorIsPassive,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ReportToExch {
	/// Indicates the party sending message will report trade
	#[serde(rename = "N")]
	IndicatesThePartySendingMessageWillReportTrade,
	/// Indicates the party receiving message must report trade
	#[serde(rename = "Y")]
	IndicatesThePartyReceivingMessageMustReportTrade,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TradedFlatSwitch {
	/// Not Traded Flat
	#[serde(rename = "N")]
	NotTradedFlat,
	/// Traded Flat
	#[serde(rename = "Y")]
	TradedFlat,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SettlCurrFxRateCalc {
	/// Multiply
	#[serde(rename = "M")]
	Multiply,
	/// Divide
	#[serde(rename = "D")]
	Divide,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum HandlInst {
	/// Automated execution order, private, no Broker intervention
	#[serde(rename = "1")]
	AutomatedExecutionOrderPriovate,
	/// Automated execution order, public, Broker intervention OK
	#[serde(rename = "2")]
	AutomatedExecutionOrderPublic,
	/// Manual order, best execution
	#[serde(rename = "3")]
	ManualOrder,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PositionEffect {
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MoneyLaunderingStatus {
	/// Passed
	#[serde(rename = "Y")]
	Passed,
	/// Not Checked
	#[serde(rename = "N")]
	NotChecked,
	/// Exempt - Below The Limit
	#[serde(rename = "1")]
	ExemptBelowLimit,
	/// Exempt - Client Money Type Exemption
	#[serde(rename = "2")]
	ExemptClientMoney,
	/// Exempt - Authorised Credit or Financial Institution
	#[serde(rename = "3")]
	ExemptAuthorizedCredit,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PriorityIndicator {
	/// Priority unchanged
	#[serde(rename = "0")]
	PriorityUnchanged,
	/// Lost Priority as result of order change
	#[serde(rename = "1")]
	LostPriorityAsResultOfOrderChange,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ContAmtType {
	/// Commission Amount (actual)
	#[serde(rename = "1")]
	CommissionAmount,
	/// Commission % (actual)
	#[serde(rename = "2")]
	Commission,
	/// Initial Charge Amount
	#[serde(rename = "3")]
	InitialChargeAmount,
	/// Initial Charge %
	#[serde(rename = "4")]
	InitialCharge,
	/// Discount Amount
	#[serde(rename = "5")]
	DiscountAmount,
	/// Discount %
	#[serde(rename = "6")]
	Discount,
	/// Dilution Levy Amount
	#[serde(rename = "7")]
	DilutionLevyAmount,
	/// Dilution Levy %
	#[serde(rename = "8")]
	DilutionLevy,
	/// Exit Charge Amount
	#[serde(rename = "9")]
	ExitChargeAmount,
	/// Exit Charge %
	#[serde(rename = "10")]
	ExitCharge,
	/// Fund-based Renewal Commission % (a.k.a. Trail commission)
	#[serde(rename = "11")]
	FundBasedRenewalCommission,
	/// Projected Fund Value (i.e. for investments intended to realise or exceed a specific future value)
	#[serde(rename = "12")]
	ProjectedFundValue,
	/// Fund-based Renewal Commission Amount (based on Order value)
	#[serde(rename = "13")]
	FundBasedRenewalCommissionAmount,
	/// Fund-based Renewal Commission Amount (based on Projected Fund value)
	#[serde(rename = "14")]
	FundBasedRenewalCommissionAmountProjected,
	/// Net Settlement Amount
	#[serde(rename = "15")]
	NetSettlementAmount,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegCoveredOrUncovered {
	/// Covered
	#[serde(rename = "0")]
	Covered,
	/// Uncovered
	#[serde(rename = "1")]
	Uncovered,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegSettlType {
	/// Regular / FX Spot settlement (T+1 or T+2 depending on currency)
	#[serde(rename = "0")]
	Regular,
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
	/// Broken date - for FX expressing non-standard tenor, <a href="tag_64_SettlDate_.html">SettlDate&nbsp;(64)</a> must be specified
	#[serde(rename = "B")]
	BrokenDate,
	/// FX Spot Next settlement (Spot+1, aka next day)
	#[serde(rename = "C")]
	FxSpotNextSettlement,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OrderHandlingInstSource {
	/// NASD OATS
	#[serde(rename = "1")]
	NasdOats,
}
