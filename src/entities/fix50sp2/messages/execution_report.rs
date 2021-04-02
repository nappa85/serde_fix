
use serde::{Serialize, Deserialize};

use crate::entities::{ApplVerID, Boolean, EncodedText, LocalMktDate, UTCTimestamp, fix50sp2::{application_sequence_control::ApplicationSequenceControl, commission_data::CommissionData, commission_data_grp::CommissionDataGrp, cont_amt_grp::ContAmtGrp, contra_grp::ContraGrp, currency::Currency, disclosure_instruction_grp::DisclosureInstructionGrp, discretion_instructions::DiscretionInstructions, display_instruction::DisplayInstruction, fills_grp::FillsGrp, financing_details::FinancingDetails, instrmt_leg_exec_grp::InstrmtLegExecGrp, instrument::Instrument, limit_amts::LimitAmts, matching_instructions::MatchingInstructions, misc_fees_grp::MiscFeesGrp, order_attribute_grp::OrderAttributeGrp, order_event_grp::OrderEventGrp, order_qty_data::OrderQtyData, parties::Parties, payment_grp::PaymentGrp, peg_instructions::PegInstructions, pre_alloc_grp::PreAllocGrp, price_qualifier_grp::PriceQualifierGrp, rate_source::RateSource, regulatory_trade_id_grp::RegulatoryTradeIDGrp, related_order_grp::RelatedOrderGrp, relative_value_grp::RelativeValueGrp, spread_or_benchmark_curve_data::SpreadOrBenchmarkCurveData, stipulations::Stipulations, strategy_parameters_grp::StrategyParametersGrp, target_parties::TargetParties, throttle_response::ThrottleResponse, trade_price_condition_grp::TradePriceConditionGrp, trd_reg_publication_grp::TrdRegPublicationGrp, trd_reg_timestamps::TrdRegTimestamps, triggering_instruction::TriggeringInstruction, und_instrmt_grp::UndInstrmtGrp, value_checks_grp::ValueChecksGrp, yield_data::YieldData}, fixt11::{Trailer, header::{HasHeader, Header, MsgType}}, version::FixVersion};

/// MsgType = 8
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExecutionReport {
	/// Standard Message Header
	#[serde(flatten)]
	pub header: Header,
	/// For use in drop copy applications. NOT FOR USE in transactional applications.
	#[serde(flatten)]
	pub application_sequence_control: Option<ApplicationSequenceControl>,
	/// OrderID is required to be unique for each chain of orders.
	#[serde(rename = "37")]
	pub order_id: String,
	/// Can be used to provide order id used by exchange or executing system. Can alternatively be used to convey implicit order priority.
	#[serde(rename = "198")]
	pub secondary_order_id: Option<String>,
	/// SecondaryClOrdID
	#[serde(rename = "526")]
	pub secondary_cl_ord_id: Option<String>,
	/// SecondaryExecID
	#[serde(rename = "527")]
	pub secondary_exec_id: Option<String>,
	/// <p>Required when referring to orders that where electronically submitted over FIX or otherwise assigned a ClOrdID(11).<br>In the case of quotes can be mapped to:
	/// </p>
	/// <ul>
	/// <li>QuoteMsgID(1166) of a single Quote(35=S)</li>
	/// <li>QuoteID(117) of a MassQuote(35=i)</li>
	/// <li>MassOrderReportID(2424) of a MassOrderAck(35=DK)</li>
	/// </ul>
	#[serde(rename = "11")]
	pub cl_ord_id: Option<String>,
	/// Conditionally required for response to a Cancel or Cancel/Replace request (ExecType(150) = 6 (Pending Cancel), 5 (Replaced),
	/// or 4 (Canceled)) when referring to orders that where electronically submitted over FIX or otherwise assigned a ClOrdID(11).
	/// ClOrdID(11) of the previous accepted order (NOT the initial order of the day) when canceling or replacing an order.
	#[serde(rename = "41")]
	pub orig_cl_ord_id: Option<String>,
	/// ClOrdLinkID
	#[serde(rename = "583")]
	pub cl_ord_link_id: Option<String>,
	/// Required if responding to a QuoteResponse(35=AJ) message. Echo back the Initiator's value specified in the message.
	#[serde(rename = "693")]
	pub quote_resp_id: Option<String>,
	/// Required if responding to and if provided on the OrderStatusRequest(35=H) message. Echo back the value provided by the requester.
	#[serde(rename = "790")]
	pub ord_status_req_id: Option<String>,
	/// Required if responding to a OrderMassStatusRequest(35=AF). Echo back the value provided by the requester.
	#[serde(rename = "584")]
	pub mass_status_req_id: Option<String>,
	/// Host assigned entity ID that can be used to reference all components of a cross; sides + strategy + legs.
	#[serde(rename = "961")]
	pub host_cross_id: Option<String>,
	/// Can be used when responding to an OrderMassStatusRequest(35=AF) to identify the total number of ExecutionReport(35=8) messages
	/// which will be returned.
	#[serde(rename = "911")]
	pub tot_num_reports: Option<i32>,
	/// Can be used when responding to an OrderMassStatusRequest(35=AF) to indicate that this is the last ExecutionReport(35=8) messages
	/// which will be returned as a result of the request.
	#[serde(rename = "912")]
	pub last_rpt_requested: Option<LastRptRequested>,
	/// <p>Specifies party information related to the submitter</p>
	#[serde(flatten)]
	pub parties: Option<Parties>,
	/// TradeOriginationDate
	#[serde(rename = "229")]
	pub trade_origination_date: Option<LocalMktDate>,
	/// ContraGrp
	#[serde(flatten)]
	pub contra_grp: Option<ContraGrp>,
	/// Required for executions against orders which were submitted as part of a list.
	#[serde(rename = "66")]
	pub list_id: Option<String>,
	/// CrossID for the replacement order
	#[serde(rename = "548")]
	pub cross_id: Option<String>,
	/// Must match original cross order. Same order chaining mechanism as ClOrdID(11)/OrigClOrdID(41) with OrderCancelReplaceRequest(35=G).
	#[serde(rename = "551")]
	pub orig_cross_id: Option<String>,
	/// CrossType
	#[serde(rename = "549")]
	pub cross_type: Option<CrossType>,
	/// TrdMatchID
	#[serde(rename = "880")]
	pub trd_match_id: Option<String>,
	/// Unique identifier of execution message as assigned by sell-side (broker, exchange, ECN) (will be 0 (zero) for ExecType(150)
	/// = I (Order Status)).
	#[serde(rename = "17")]
	pub exec_id: String,
	/// Required for ExecType(150) = H (Trade Cancel) and ExecType(150) = G (Trade Correct).
	#[serde(rename = "19")]
	pub exec_ref_id: Option<String>,
	/// Describes the purpose of the execution report.
	#[serde(rename = "150")]
	pub exec_type: ExecType,
	/// Describes the current state of a CHAIN of orders, same scope as <a href="tag_38_OrderQty_.html">OrderQty&nbsp;(38)</a> , <a href="tag_14_CumQty_.html">CumQty&nbsp;(14)</a> , <a href="tag_151_LeavesQty_.html">LeavesQty&nbsp;(151)</a> , and <a href="tag_6_AvgPx_.html">AvgPx&nbsp;(6)</a>
	#[serde(rename = "39")]
	pub ord_status: OrdStatus,
	/// For optional use with <a href="tag_39_OrdStatus_.html">OrdStatus&nbsp;(39)</a> = 0 (New)
	#[serde(rename = "636")]
	pub working_indicator: Option<WorkingIndicator>,
	/// For optional use with <a href="tag_150_ExecType_.html">ExecType&nbsp;(150)</a> = 8 (Rejected)
	#[serde(rename = "103")]
	pub ord_rej_reason: Option<OrdRejReason>,
	/// Required for <a href="tag_150_ExecType_.html">ExecType&nbsp;(150)</a> = D (Restated).
	#[serde(rename = "378")]
	pub exec_restatement_reason: Option<ExecRestatementReason>,
	/// Required for executions against electronically submitted orders which were assigned an account by the institution or intermediary
	#[serde(rename = "1")]
	pub account: Option<String>,
	/// AcctIDSource
	#[serde(rename = "660")]
	pub acct_id_source: Option<AcctIDSource>,
	/// Specifies type of account
	#[serde(rename = "581")]
	pub account_type: Option<AccountType>,
	/// DayBookingInst
	#[serde(rename = "589")]
	pub day_booking_inst: Option<DayBookingInst>,
	/// BookingUnit
	#[serde(rename = "590")]
	pub booking_unit: Option<BookingUnit>,
	/// PreallocMethod
	#[serde(rename = "591")]
	pub prealloc_method: Option<PreallocMethod>,
	/// AllocID
	#[serde(rename = "70")]
	pub alloc_id: Option<String>,
	/// Pre-trade allocation instructions.
	#[serde(flatten)]
	pub pre_alloc_grp: Option<PreAllocGrp>,
	/// SettlType
	#[serde(rename = "63")]
	pub settl_type: Option<SettlType>,
	/// Takes precedence over <a href="tag_63_SettlType_.html">SettlType&nbsp;(63)</a> value and conditionally required/omitted for specific <a href="tag_63_SettlType_.html">SettlType&nbsp;(63)</a> values.
	#[serde(rename = "64")]
	pub settl_date: Option<LocalMktDate>,
	/// MatchType
	#[serde(rename = "574")]
	pub match_type: Option<MatchType>,
	/// OrderCategory
	#[serde(rename = "1115")]
	pub order_category: Option<OrderCategory>,
	/// CashMargin
	#[serde(rename = "544")]
	pub cash_margin: Option<CashMargin>,
	/// ClearingFeeIndicator
	#[serde(rename = "635")]
	pub clearing_fee_indicator: Option<ClearingFeeIndicator>,
	/// Instrument
	#[serde(flatten)]
	pub instrument: Instrument,
	/// FinancingDetails
	#[serde(flatten)]
	pub financing_details: Option<FinancingDetails>,
	/// Number of underlyings.
	#[serde(flatten)]
	pub und_instrmt_grp: Option<UndInstrmtGrp>,
	/// Side
	#[serde(rename = "54")]
	pub side: Side,
	/// Stipulations
	#[serde(flatten)]
	pub stipulations: Option<Stipulations>,
	/// QtyType
	#[serde(rename = "854")]
	pub qty_type: Option<QtyType>,
	/// Conditionally required when the OrderQtyData component is required or specified in a prior, related message. For example,
	/// when used in a work flow including a NewOrderSingle(35=D) or NewOrderCross(35=s) message, the OrderQtyData component is a
	/// required component in these messages and thus the component is required here. When the OrderQtyData component is optional
	/// in a related message, such as the NewOrderMultileg(35=AB), the component is required here when specified in the prior, related
	/// NewOrderMultileg(35=AB) message.
	#[serde(flatten)]
	pub order_qty_data: Option<OrderQtyData>,
	/// LotType
	#[serde(rename = "1093")]
	pub lot_type: Option<LotType>,
	/// OrdType
	#[serde(rename = "40")]
	pub ord_type: Option<OrdType>,
	/// PriceType
	#[serde(rename = "423")]
	pub price_type: Option<PriceType>,
	/// Required if specified on the order
	#[serde(rename = "44")]
	pub price: Option<f64>,
	/// PriceProtectionScope
	#[serde(rename = "1092")]
	pub price_protection_scope: Option<PriceProtectionScope>,
	/// Required if specified on the order
	#[serde(rename = "99")]
	pub stop_px: Option<f64>,
	/// TriggeringInstruction
	#[serde(flatten)]
	pub triggering_instruction: Option<TriggeringInstruction>,
	/// PegInstructions
	#[serde(flatten)]
	pub peg_instructions: Option<PegInstructions>,
	/// DiscretionInstructions
	#[serde(flatten)]
	pub discretion_instructions: Option<DiscretionInstructions>,
	/// The current price the order is pegged at.
	#[serde(rename = "839")]
	pub pegged_price: Option<f64>,
	/// The reference price of a pegged order.
	#[serde(rename = "1095")]
	pub pegged_ref_price: Option<f64>,
	/// The current discretionary price of the order.
	#[serde(rename = "845")]
	pub discretion_price: Option<f64>,
	/// The target strategy of the order.
	#[serde(rename = "847")]
	pub target_strategy: Option<TargetStrategy>,
	/// Strategy parameter block
	#[serde(flatten)]
	pub strategy_parameters_grp: Option<StrategyParametersGrp>,
	/// (Deprecated in FIX.5.0)For further specification of the <a href="tag_847_TargetStrategy_.html">TargetStrategy&nbsp;(847)</a> .
	#[serde(rename = "848")]
	pub target_strategy_parameters: Option<String>,
	/// (Deprecated in FIX.5.0) Mandatory for a TargetStrategy=Participate order and specifies the target particpation rate. For other
	/// order types optionally specifies a volume limit (i.e. do not be more than this percent of the market volume)
	#[serde(rename = "849")]
	pub participation_rate: Option<f32>,
	/// For communication of the performance of the order versus the target strategy
	#[serde(rename = "850")]
	pub target_strategy_performance: Option<f64>,
	/// Currency
	#[serde(rename = "15")]
	pub currency: Option<Currency>,
	/// ComplianceID
	#[serde(rename = "376")]
	pub compliance_id: Option<String>,
	/// SolicitedFlag
	#[serde(rename = "377")]
	pub solicited_flag: Option<SolicitedFlag>,
	/// Absence of this field indicates Day order
	#[serde(rename = "59")]
	pub time_in_force: Option<TimeInForce>,
	/// Time specified on the order at which the order should be considered valid
	#[serde(rename = "168")]
	pub effective_time: Option<UTCTimestamp>,
	/// Conditionally required if TimeInForce(59) = 6 (GTD) and ExpireTime(126) is not specified.
	#[serde(rename = "432")]
	pub expire_date: Option<LocalMktDate>,
	/// Conditionally required if TimeInForce(59) = 6 (GTD) and ExpireDate(432) is not specified.
	#[serde(rename = "126")]
	pub expire_time: Option<UTCTimestamp>,
	/// Can contain multiple instructions, space delimited.
	#[serde(rename = "18")]
	pub exec_inst: Option<ExecInst>,
	/// AggressorIndicator
	#[serde(rename = "1057")]
	pub aggressor_indicator: Option<AggressorIndicator>,
	/// OrderCapacity
	#[serde(rename = "528")]
	pub order_capacity: Option<OrderCapacity>,
	/// OrderRestrictions
	#[serde(rename = "529")]
	pub order_restrictions: Option<OrderRestrictions>,
	/// PreTradeAnonymity
	#[serde(rename = "1091")]
	pub pre_trade_anonymity: Option<Boolean>,
	/// CustOrderCapacity
	#[serde(rename = "582")]
	pub cust_order_capacity: Option<CustOrderCapacity>,
	/// Quantity (e.g. shares) bought/sold on this (last) fill. Required if ExecType(150) = F (Trade) or ExecType(150) = G (Trade
	/// Correct) unless FillsGrp or OrderEventGrp is used. If ExecType(150) = 7 (Stopped), represents the quantity stopped/guaranteed/protected
	/// for.
	#[serde(rename = "32")]
	pub last_qty: Option<f64>,
	/// Used for FX trades to express the quantity or amount of the other side of the currency. Conditionally required if ExecType(150)
	/// = F (Trade) or G (Trade Correct) and is an FX trade.
	#[serde(rename = "1056")]
	pub calculated_ccy_last_qty: Option<f64>,
	/// Optionally used when ExecType(150) = F (Trade) or G (Trade Correct) and is a FX Swap trade. Used to express the swap points
	/// for the swap trade event.
	#[serde(rename = "1071")]
	pub last_swap_points: Option<f64>,
	/// UnderlyingLastQty
	#[serde(rename = "652")]
	pub underlying_last_qty: Option<f64>,
	/// Price of this (last) fill. Required if ExecType(150) = ExecType = F (Trade) or G (Trade Correct) unless FillsGrp or OrderEventGrp
	/// or TradePriceCondition(1839)=17 (Price is pending) or 18 (Price is not applicable) is used. Should represent the "all-in"
	/// (LastSpotRate(194) + LastForwardPoints(195)) rate for F/X orders.). If ExecType(150) = 7 (Stopped), represents the price stopped/guaranteed/protected
	/// at. Not required for FX Swap when ExecType(150) = F (Trade) or G (Trade Correct) as there is no "all-in" rate that applies
	/// to both legs of the FX Swap.
	#[serde(rename = "31")]
	pub last_px: Option<f64>,
	/// UnderlyingLastPx
	#[serde(rename = "651")]
	pub underlying_last_px: Option<f64>,
	/// Last price expressed in percent-of-par. Conditionally required for Fixed Income trades when LastPx(31) is expressed in Yield,
	/// Spread, Discount or any other price type that is not percent-of-par.
	#[serde(rename = "669")]
	pub last_par_px: Option<f64>,
	/// Applicable for F/X orders
	#[serde(rename = "194")]
	pub last_spot_rate: Option<f64>,
	/// Applicable for F/X orders
	#[serde(rename = "195")]
	pub last_forward_points: Option<f64>,
	/// If ExecType(150) = F (Trade), indicates the market where the trade was executed. If ExecType(150) = 0 (New), indicates the
	/// market where the order was routed.
	#[serde(rename = "30")]
	pub last_mkt: Option<String>,
	/// TradingSessionID
	#[serde(rename = "336")]
	pub trading_session_id: Option<TradingSessionID>,
	/// TradingSessionSubID
	#[serde(rename = "625")]
	pub trading_session_sub_id: Option<TradingSessionSubID>,
	/// TimeBracket
	#[serde(rename = "943")]
	pub time_bracket: Option<String>,
	/// LastCapacity
	#[serde(rename = "29")]
	pub last_capacity: Option<LastCapacity>,
	/// Quantity open for further execution. If the OrdStatus(39) is = 4 (Canceled), 3 (Done For Day), C (Expired), B (Calculated),
	/// or 8 (Rejected) (in which case the order is no longer active) then LeavesQty(151) could be 0, otherwise LeavesQty(151) = OrderQty(38)
	/// - CumQty(14).
	#[serde(rename = "151")]
	pub leaves_qty: f64,
	/// Currently executed quantity for chain of orders.
	#[serde(rename = "14")]
	pub cum_qty: f64,
	/// Not required for markets where average price is not calculated by the market. Conditionally required otherwise.
	#[serde(rename = "6")]
	pub avg_px: Option<f64>,
	/// For GT orders on days following the day of the first trade.
	#[serde(rename = "424")]
	pub day_order_qty: Option<f64>,
	/// For GT orders on days following the day of the first trade.
	#[serde(rename = "425")]
	pub day_cum_qty: Option<f64>,
	/// For GT orders on days following the day of the first trade.
	#[serde(rename = "426")]
	pub day_avg_px: Option<f64>,
	/// Used to support fragmentation. Sum of NoFills(1362) across all messages with the same ExecID(17).
	#[serde(rename = "1361")]
	pub tot_no_fills: Option<i32>,
	/// Indicates whether this is the last fragment in a sequence of message fragments. Only required where message has been fragmented.
	#[serde(rename = "893")]
	pub last_fragment: Option<LastFragment>,
	/// Specifies the partial fills included in this Execution Report(35=8), mutually exclusive with OrderEventGrp component
	#[serde(flatten)]
	pub fills_grp: Option<FillsGrp>,
	/// States whether executions are booked out or accumulated on a partially filled GT order
	#[serde(rename = "427")]
	pub gt_booking_inst: Option<GTBookingInst>,
	/// Used when reporting other than current day trades.
	#[serde(rename = "75")]
	pub trade_date: Option<LocalMktDate>,
	/// Time the transaction represented by this ExecutionReport occurred
	#[serde(rename = "60")]
	pub transact_time: Option<UTCTimestamp>,
	/// ReportToExch
	#[serde(rename = "113")]
	pub report_to_exch: Option<ReportToExch>,
	/// Note: On a fill/partial-fill message, it represents value for that fill/partial fill. On ExecType(150) = B (Calculated), it
	/// represents cumulative value for the order.
	#[serde(flatten)]
	pub commission_data: Option<CommissionData>,
	/// SpreadOrBenchmarkCurveData
	#[serde(flatten)]
	pub spread_or_benchmark_curve_data: Option<SpreadOrBenchmarkCurveData>,
	/// YieldData
	#[serde(flatten)]
	pub yield_data: Option<YieldData>,
	/// GrossTradeAmt
	#[serde(rename = "381")]
	pub gross_trade_amt: Option<f64>,
	/// NumDaysInterest
	#[serde(rename = "157")]
	pub num_days_interest: Option<i32>,
	/// ExDate
	#[serde(rename = "230")]
	pub ex_date: Option<LocalMktDate>,
	/// AccruedInterestRate
	#[serde(rename = "158")]
	pub accrued_interest_rate: Option<f32>,
	/// AccruedInterestAmt
	#[serde(rename = "159")]
	pub accrued_interest_amt: Option<f64>,
	/// For fixed income products which pay lump-sum interest at maturity.
	#[serde(rename = "738")]
	pub interest_at_maturity: Option<f64>,
	/// For repurchase agreements the accrued interest on termination.
	#[serde(rename = "920")]
	pub end_accrued_interest_amt: Option<f64>,
	/// For repurchase agreements the start (dirty) cash consideration.
	#[serde(rename = "921")]
	pub start_cash: Option<f64>,
	/// For repurchase agreements the end (dirty) cash consideration.
	#[serde(rename = "922")]
	pub end_cash: Option<f64>,
	/// TradedFlatSwitch
	#[serde(rename = "258")]
	pub traded_flat_switch: Option<TradedFlatSwitch>,
	/// BasisFeatureDate
	#[serde(rename = "259")]
	pub basis_feature_date: Option<LocalMktDate>,
	/// BasisFeaturePrice
	#[serde(rename = "260")]
	pub basis_feature_price: Option<f64>,
	/// Concession
	#[serde(rename = "238")]
	pub concession: Option<f64>,
	/// TotalTakedown
	#[serde(rename = "237")]
	pub total_takedown: Option<f64>,
	/// On a fill/partial fill message, it represents value for that fill/partial fill. On a ExecType(150) = B (Calculated) message,
	/// it represents cumulative value for the order. Value expressed in the currency reflected by the Currency(15) field.
	#[serde(rename = "118")]
	pub net_money: Option<f64>,
	/// Used to report results of forex accommodation trade.
	#[serde(rename = "119")]
	pub settl_curr_amt: Option<f64>,
	/// Used to report results of forex accommodation trade. Required for Non-Deliverable Forwards.
	#[serde(rename = "120")]
	pub settl_currency: Option<Currency>,
	/// RateSource
	#[serde(flatten)]
	pub rate_source: Option<RateSource>,
	/// OffshoreIndicator
	#[serde(rename = "2795")]
	pub offshore_indicator: Option<OffshoreIndicator>,
	/// Foreign exchange rate used to compute SettlCurrAmt(119) from Currency(15) to SettlCurrency(120).
	#[serde(rename = "155")]
	pub settl_curr_fx_rate: Option<f64>,
	/// Specifies whether the SettlCurrFxRate(155) should be multiplied or divided.
	#[serde(rename = "156")]
	pub settl_curr_fx_rate_calc: Option<SettlCurrFxRateCalc>,
	/// HandlInst
	#[serde(rename = "21")]
	pub handl_inst: Option<HandlInst>,
	/// MinQty
	#[serde(rename = "110")]
	pub min_qty: Option<f64>,
	/// MatchIncrement
	#[serde(rename = "1089")]
	pub match_increment: Option<f64>,
	/// MaxPriceLevels
	#[serde(rename = "1090")]
	pub max_price_levels: Option<i32>,
	/// DisplayInstruction
	#[serde(flatten)]
	pub display_instruction: Option<DisplayInstruction>,
	/// MaxFloor
	#[serde(rename = "111")]
	pub max_floor: Option<f64>,
	/// For use in derivatives omnibus accounting
	#[serde(rename = "77")]
	pub position_effect: Option<PositionEffect>,
	/// (Deprecated in FIX.5.0)
	#[serde(rename = "210")]
	pub max_show: Option<f64>,
	/// Method for booking out this order. Used when notifying a broker that an order to be settled by that broker is to be booked
	/// out as an OTC derivative (e.g. CFD or similar). Absence of this field implies regular booking.
	#[serde(rename = "775")]
	pub booking_type: Option<BookingType>,
	/// Text
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if <a href="tag_355_EncodedText_.html">EncodedText&nbsp;(355)</a> field is specified and must immediately precede it.
	#[serde(rename = "354")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_58_Text_.html">Text&nbsp;(58)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding_.html">MessageEncoding&nbsp;(347)</a> field.
	#[serde(alias = "355")]
	pub encoded_text: Option<EncodedText<355>>,
	/// (Deprecated in FIX.5.0)Can be used with <a href="tag_40_OrdType_.html">OrdType&nbsp;(40)</a> = "Forex - Swap" to specify the "value date" for the future portion of a F/X swap.
	#[serde(rename = "193")]
	pub settl_date_2: Option<LocalMktDate>,
	/// (Deprecated in FIX.5.0)Can be used with <a href="tag_40_OrdType_.html">OrdType&nbsp;(40)</a> = "Forex - Swap" to specify the order quantity for the future portion of a F/X swap.
	#[serde(rename = "192")]
	pub order_qty_2: Option<f64>,
	/// Can be used with <a href="tag_40_OrdType_.html">OrdType&nbsp;(40)</a> = "Forex - Swap" to specify the forward points (added to LastSpotRate) for the future portion of a F/X swap.
	#[serde(rename = "641")]
	pub last_forward_points_2: Option<f64>,
	/// Default is a single security if not specified.
	#[serde(rename = "442")]
	pub multi_leg_reporting_type: Option<MultiLegReportingType>,
	/// For CIV - Optional
	#[serde(rename = "480")]
	pub cancellation_rights: Option<CancellationRights>,
	/// MoneyLaunderingStatus
	#[serde(rename = "481")]
	pub money_laundering_status: Option<MoneyLaunderingStatus>,
	/// Reference to <a href="message_Registration_Instructions_o_.html">Registration Instructions&nbsp;(o)</a> message for this Order.
	#[serde(rename = "513")]
	pub regist_id: Option<String>,
	/// Supplementary registration information for this Order
	#[serde(rename = "494")]
	pub designation: Option<String>,
	/// For CIV - Optional
	#[serde(rename = "483")]
	pub trans_bkd_time: Option<UTCTimestamp>,
	/// For CIV - Optional
	#[serde(rename = "515")]
	pub exec_valuation_point: Option<UTCTimestamp>,
	/// For CIV - Optional
	#[serde(rename = "484")]
	pub exec_price_type: Option<ExecPriceType>,
	/// For CIV - Optional
	#[serde(rename = "485")]
	pub exec_price_adjustment: Option<f64>,
	/// PriorityIndicator
	#[serde(rename = "638")]
	pub priority_indicator: Option<PriorityIndicator>,
	/// PriceImprovement
	#[serde(rename = "639")]
	pub price_improvement: Option<f64>,
	/// Applicable only on OrdStatus(39) = 1 of (Partially filled) or 2(Filled).
	#[serde(rename = "851")]
	pub last_liquidity_ind: Option<LastLiquidityInd>,
	/// ContAmtGrp
	#[serde(flatten)]
	pub cont_amt_grp: Option<ContAmtGrp>,
	/// InstrmtLegExecGrp
	#[serde(flatten)]
	pub instrmt_leg_exec_grp: Option<InstrmtLegExecGrp>,
	/// CopyMsgIndicator
	#[serde(rename = "797")]
	pub copy_msg_indicator: Option<Boolean>,
	/// Required if any miscellaneous fees are reported.
	#[serde(flatten)]
	pub misc_fees_grp: Option<MiscFeesGrp>,
	/// DividendYield
	#[serde(rename = "1380")]
	pub dividend_yield: Option<f32>,
	/// ManualOrderIndicator
	#[serde(rename = "1028")]
	pub manual_order_indicator: Option<Boolean>,
	/// CustDirectedOrder
	#[serde(rename = "1029")]
	pub cust_directed_order: Option<Boolean>,
	/// ReceivedDeptID
	#[serde(rename = "1030")]
	pub received_dept_id: Option<String>,
	/// CustOrderHandlingInst
	#[serde(rename = "1031")]
	pub cust_order_handling_inst: Option<CustOrderHandlingInst>,
	/// OrderHandlingInstSource
	#[serde(rename = "1032")]
	pub order_handling_inst_source: Option<OrderHandlingInstSource>,
	/// TrdRegTimestamps
	#[serde(flatten)]
	pub trd_reg_timestamps: Option<TrdRegTimestamps>,
	/// Volatility
	#[serde(rename = "1188")]
	pub volatility: Option<f64>,
	/// TimeToExpiration
	#[serde(rename = "1189")]
	pub time_to_expiration: Option<f64>,
	/// RiskFreeRate
	#[serde(rename = "1190")]
	pub risk_free_rate: Option<f64>,
	/// PriceDelta
	#[serde(rename = "811")]
	pub price_delta: Option<f64>,
	/// MatchingInstructions
	#[serde(flatten)]
	pub matching_instructions: Option<MatchingInstructions>,
	/// Conditionally required when TimeInForce(59)=10 (Good for Time)
	#[serde(rename = "1629")]
	pub exposure_duration: Option<i32>,
	/// For contingency orders, the type of contingency as specified in the order
	#[serde(rename = "1385")]
	pub contingency_type: Option<ContingencyType>,
	/// LimitAmts
	#[serde(flatten)]
	pub limit_amts: Option<LimitAmts>,
	/// ExDestination
	#[serde(rename = "100")]
	pub ex_destination: Option<String>,
	/// ExDestinationIDSource
	#[serde(rename = "1133")]
	pub ex_destination_id_source: Option<ExDestinationIDSource>,
	/// ThrottleResponse
	#[serde(flatten)]
	pub throttle_response: Option<ThrottleResponse>,
	/// Upfront Price for CDS transactions. Conditionally required if TradePriceNegotiationMethod(1740) = 4(Percent of par and upfront
	/// amount), 5(Deal spread and upfront amount) or 6(Upfront points and upfront amount)
	#[serde(rename = "1743")]
	pub last_upfront_price: Option<f64>,
	/// TradePriceNegotiationMethod
	#[serde(rename = "1740")]
	pub trade_price_negotiation_method: Option<TradePriceNegotiationMethod>,
	/// UpfrontPriceType
	#[serde(rename = "1741")]
	pub upfront_price_type: Option<UpfrontPriceType>,
	/// Required if specified on the order.
	#[serde(rename = "1742")]
	pub upfront_price: Option<f64>,
	/// Available for optional use when Side(54) = 6(Sell short exempt)
	#[serde(rename = "1688")]
	pub short_sale_exemption_reason: Option<ShortSaleExemptionReason>,
	/// Reference to the MDEntryID(278) of this order or quote in the market data.
	#[serde(rename = "278")]
	pub md_entry_id: Option<String>,
	/// RefOrderID
	#[serde(rename = "1080")]
	pub ref_order_id: Option<String>,
	/// RefOrderIDSource
	#[serde(rename = "1081")]
	pub ref_order_id_source: Option<RefOrderIDSource>,
	/// Specifies parties not directly associated with or owning the order, who are to be informed to effect processing of the order
	#[serde(flatten)]
	pub target_parties: Option<TargetParties>,
	/// Triggered
	#[serde(rename = "1823")]
	pub triggered: Option<Triggered>,
	/// AuctionInstruction
	#[serde(rename = "1805")]
	pub auction_instruction: Option<AuctionInstruction>,
	/// TradingCapacity
	#[serde(rename = "1815")]
	pub trading_capacity: Option<TradingCapacity>,
	/// Applies to trades resulting from the order
	#[serde(rename = "1390")]
	pub trade_publish_indicator: Option<TradePublishIndicator>,
	/// MarketSegmentID
	#[serde(rename = "1300")]
	pub market_segment_id: Option<String>,
	/// Specifies the order events included in this ExecutionReport(35=8), mutually exclusive with FillsGrp component.
	#[serde(flatten)]
	pub order_event_grp: Option<OrderEventGrp>,
	/// MinQtyMethod
	#[serde(rename = "1822")]
	pub min_qty_method: Option<MinQtyMethod>,
	/// DisclosureInstructionGrp
	#[serde(flatten)]
	pub disclosure_instruction_grp: Option<DisclosureInstructionGrp>,
	/// ClearingAccountType
	#[serde(rename = "1816")]
	pub clearing_account_type: Option<ClearingAccountType>,
	/// RefClOrdID
	#[serde(rename = "1806")]
	pub ref_cl_ord_id: Option<String>,
	/// AuctionType
	#[serde(rename = "1803")]
	pub auction_type: Option<AuctionType>,
	/// AuctionAllocationPct
	#[serde(rename = "1804")]
	pub auction_allocation_pct: Option<f32>,
	/// LockedQty
	#[serde(rename = "1808")]
	pub locked_qty: Option<f64>,
	/// SecondaryLockedQty
	#[serde(rename = "1809")]
	pub secondary_locked_qty: Option<f64>,
	/// LockType
	#[serde(rename = "1807")]
	pub lock_type: Option<LockType>,
	/// ReleaseInstruction
	#[serde(rename = "1810")]
	pub release_instruction: Option<ReleaseInstruction>,
	/// ReleaseQty
	#[serde(rename = "1811")]
	pub release_qty: Option<f64>,
	/// RelatedHighPrice
	#[serde(rename = "1819")]
	pub related_high_price: Option<f64>,
	/// RelatedLowPrice
	#[serde(rename = "1820")]
	pub related_low_price: Option<f64>,
	/// RelatedPriceSource
	#[serde(rename = "1821")]
	pub related_price_source: Option<RelatedPriceSource>,
	/// LastQtyVariance
	#[serde(rename = "1828")]
	pub last_qty_variance: Option<f64>,
	/// Reason description for rejecting the transaction
	#[serde(rename = "1328")]
	pub reject_text: Option<String>,
	/// Must be set if <a href="tag_1665_EncodedRejectText_.html">EncodedRejectTextLen(1665)&nbsp;(1665)</a> field is specified and must immediately precede it.
	#[serde(rename = "1664")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_1328_RejectText_.html">RejectedText(1328)&nbsp;(1328)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding_.html">MessageEncoding(347)&nbsp;(347)</a> field.
	#[serde(alias = "1665")]
	pub encoded_reject_text: Option<EncodedText<1665>>,
	/// OrderOrigination
	#[serde(rename = "1724")]
	pub order_origination: Option<OrderOrigination>,
	/// OriginatingDeptID
	#[serde(rename = "1725")]
	pub originating_dept_id: Option<String>,
	/// ReceivingDeptID
	#[serde(rename = "1726")]
	pub receiving_dept_id: Option<String>,
	/// OwnerType
	#[serde(rename = "522")]
	pub owner_type: Option<OwnerType>,
	/// In the case of quotes can be mapped to: QuoteMsgID(1166) of a single Quote(35=S) or QuoteID(117) of a Mass Quote(35=i).
	#[serde(rename = "1166")]
	pub quote_msg_id: Option<String>,
	/// CoverPrice
	#[serde(rename = "1917")]
	pub cover_price: Option<f64>,
	/// ShortMarkingExemptIndicator
	#[serde(rename = "2102")]
	pub short_marking_exempt_indicator: Option<ShortMarkingExemptIndicator>,
	/// RefRiskLimitCheckID
	#[serde(rename = "2334")]
	pub ref_risk_limit_check_id: Option<String>,
	/// Conditionally required when RefRiskLimitCheckID(2334) is specified.
	#[serde(rename = "2335")]
	pub ref_risk_limit_check_id_type: Option<RefRiskLimitCheckIDType>,
	/// RegulatoryTradeIDGrp
	#[serde(flatten)]
	pub regulatory_trade_id_grp: Option<RegulatoryTradeIDGrp>,
	/// MidPx
	#[serde(rename = "631")]
	pub mid_px: Option<f64>,
	/// TrdType
	#[serde(rename = "828")]
	pub trd_type: Option<TrdType>,
	/// RegulatoryTransactionType
	#[serde(rename = "2347")]
	pub regulatory_transaction_type: Option<RegulatoryTransactionType>,
	/// ComplianceText
	#[serde(rename = "2404")]
	pub compliance_text: Option<String>,
	/// Must be set if <a href="tag_2352_EncodedComplianceText_.html">EncodedComplianceTextLen(2352)&nbsp;(2352)</a> field is specified and must immediately precede it.
	#[serde(rename = "2351")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_2404_ComplianceText_.html">ComplianceText(2404)&nbsp;(2404)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding_.html">MessageEncoding(347)&nbsp;(347)</a> field.
	#[serde(alias = "2352")]
	pub encoded_compliance_text: Option<EncodedText<2352>>,
	/// Required if provided on the order message. Echo back the value provided in the order message.
	#[serde(rename = "2422")]
	pub order_request_id: Option<i32>,
	/// Can be used to link execution to the MassOrder(35=DJ) message.
	#[serde(rename = "2423")]
	pub mass_order_request_id: Option<String>,
	/// Can be used to provide further detail for ExecType(150) field.
	#[serde(rename = "2431")]
	pub exec_type_reason: Option<ExecTypeReason>,
	/// Can be used to specify the remaining quantity that was cancelled prior to order reaching terminal state (i.e. when LeavesQty(151)=0).
	/// If specified, OrderQty(38) = CumQty(14) + CxlQty(84).
	#[serde(rename = "84")]
	pub cxl_qty: Option<f64>,
	/// ExposureDurationUnit
	#[serde(rename = "1916")]
	pub exposure_duration_unit: Option<ExposureDurationUnit>,
	/// RelativeValueGrp
	#[serde(flatten)]
	pub relative_value_grp: Option<RelativeValueGrp>,
	/// Use as an alternative to CommissionData component if multiple commissions or enhanced attributes are needed.
	#[serde(flatten)]
	pub commission_data_grp: Option<CommissionDataGrp>,
	/// VenueType
	#[serde(rename = "1430")]
	pub venue_type: Option<VenueType>,
	/// May be used as an alternative to MatchingInstructions when the identifier does not appear in another field.
	#[serde(rename = "2362")]
	pub self_match_prevention_id: Option<String>,
	/// CrossedIndicator
	#[serde(rename = "2523")]
	pub crossed_indicator: Option<CrossedIndicator>,
	/// AlgorithmicTradeIndicator
	#[serde(rename = "2667")]
	pub algorithmic_trade_indicator: Option<AlgorithmicTradeIndicator>,
	/// TrdSubType
	#[serde(rename = "829")]
	pub trd_sub_type: Option<TrdSubType>,
	/// SecondaryTrdType
	#[serde(rename = "855")]
	pub secondary_trd_type: Option<TrdType>,
	/// PreviouslyReported
	#[serde(rename = "570")]
	pub previously_reported: Option<PreviouslyReported>,
	/// May be used to bilaterally inform counterparty of trade reporting status.
	#[serde(rename = "2524")]
	pub trade_reporting_indicator: Option<TradeReportingIndicator>,
	/// OrderAttributeGrp
	#[serde(flatten)]
	pub order_attribute_grp: Option<OrderAttributeGrp>,
	/// TrdRegPublicationGrp
	#[serde(flatten)]
	pub trd_reg_publication_grp: Option<TrdRegPublicationGrp>,
	/// TradePriceConditionGrp
	#[serde(flatten)]
	pub trade_price_condition_grp: Option<TradePriceConditionGrp>,
	/// TrdMatchSubID
	#[serde(rename = "1891")]
	pub trd_match_sub_id: Option<String>,
	/// MaximumPriceDeviation
	#[serde(rename = "2676")]
	pub maximum_price_deviation: Option<f32>,
	/// ValueChecksGrp
	#[serde(flatten)]
	pub value_checks_grp: Option<ValueChecksGrp>,
	/// Can be used to highlight change of order ownership.
	#[serde(rename = "2679")]
	pub order_ownership_indicator: Option<OrderOwnershipIndicator>,
	/// PaymentGrp
	#[serde(flatten)]
	pub payment_grp: Option<PaymentGrp>,
	/// ExDestinationType
	#[serde(rename = "2704")]
	pub ex_destination_type: Option<ExDestinationType>,
	/// PriceQualifierGrp
	#[serde(flatten)]
	pub price_qualifier_grp: Option<PriceQualifierGrp>,
	/// ReportingPx
	#[serde(rename = "2750")]
	pub reporting_px: Option<f64>,
	/// ReportingQty
	#[serde(rename = "2751")]
	pub reporting_qty: Option<f64>,
	/// CurrentWorkingPrice
	#[serde(rename = "2838")]
	pub current_working_price: Option<f64>,
	/// EventInitiatorType
	#[serde(rename = "2830")]
	pub event_initiator_type: Option<EventInitiatorType>,
	/// RoutingArrangmentIndicator
	#[serde(rename = "2883")]
	pub routing_arrangment_indicator: Option<RoutingArrangmentIndicator>,
	/// May be used for cross orders submitted with single order messages.
	#[serde(rename = "2884")]
	pub contra_routing_arrangment_indicator: Option<i32>,
	/// May be used for cross orders submitted with single order messages.
	#[serde(rename = "2882")]
	pub contra_order_origination: Option<i32>,
	/// May be used to indicate the post-execution trade continuation or lifecycle event. This should echo the value in the message
	/// that resulted in this report.
	#[serde(rename = "1937")]
	pub trade_continuation: Option<TradeContinuation>,
	/// TradeContinuationText
	#[serde(rename = "2374")]
	pub trade_continuation_text: Option<String>,
	/// Must be set if EncodedTradeContinuationText(2371) field is specified and must immediately precede it.
	#[serde(rename = "2372")]
	/// Encoded (non-ASCII characters) representation of the TradeContinuationText(2374) field in the encoded format specified via
	/// the MessageEncoding(347) field.
	#[serde(alias = "2371")]
	pub encoded_trade_continuation_text: Option<EncodedText<2371>>,
	/// May be used to provide a list of orders and their relationship to the order identified in this message.
	#[serde(flatten)]
	pub related_order_grp: Option<RelatedOrderGrp>,
	/// RegulatoryReportType
	#[serde(rename = "1934")]
	pub regulatory_report_type: Option<RegulatoryReportType>,
	/// AffiliatedFirmsTradeIndicator
	#[serde(rename = "2525")]
	pub affiliated_firms_trade_indicator: Option<AffiliatedFirmsTradeIndicator>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub trailer: Trailer,
}

impl ExecutionReport {
	pub fn new() -> Self {
		ExecutionReport {
			header: Header {
                begin_string: Some(FixVersion::FIXT11),
                msg_type: Some(MsgType::Heartbeat),
                appl_ver_id: Some(ApplVerID::FIX50SP2),
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
	/// an "All or None"."
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
	/// Basis Cross (A trade where a basket of securities or an index participation unit is transacted at prices achieved through
	/// the execution of related exchange-traded derivative instruments in an amount that will correspond to an equivalent market
	/// exposure)
	#[serde(rename = "5")]
	BasisCross,
	/// Contingent Cross (A cross resulting from a paired order placed by a Participant to execute an order on a security that is
	/// contingent on the execution of a second order for an offsetting volume of a related security)
	#[serde(rename = "6")]
	ContingentCross,
	/// VWAP Cross (A cross for the purpose of executing a trade at a volume-weighted average price of a security traded for a continuous
	/// period on or during a trading day)
	#[serde(rename = "7")]
	VwapCross,
	/// STS Cross (A closing price cross resulting from an order placed by a Participant for execution in a Special Trading Session
	/// at the last sale price)
	#[serde(rename = "8")]
	StsCross,
	/// Customer to customer cross
	#[serde(rename = "9")]
	CustomerToCustomerCross,
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
	/// Locked
	#[serde(rename = "M")]
	Locked,
	/// Released
	#[serde(rename = "N")]
	Released,
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
	/// Surveillance option
	#[serde(rename = "12")]
	SurveillanceOption,
	/// Incorrect quantity
	#[serde(rename = "13")]
	IncorrectQuantity,
	/// Incorrect allocated quantity
	#[serde(rename = "14")]
	IncorrectAllocatedQuantity,
	/// Unknown account(s)
	#[serde(rename = "15")]
	UnknownAccountS,
	/// Price exceeds current price band
	#[serde(rename = "16")]
	PriceExceedsCurrentPriceBand,
	/// Invalid price increment
	#[serde(rename = "18")]
	InvalidPriceIncrement,
	/// Other
	#[serde(rename = "99")]
	Other,
	/// Reference price not available
	#[serde(rename = "19")]
	ReferencePriceNotAvailable,
	/// Notional value exceeds threshold
	#[serde(rename = "20")]
	NotionalValueExceedsThreshold,
	/// Algorithm risk threshold breached (Elaboration: Elaboration: A sell-side broker algorithm has detected that a risk limit has
	/// been breached which requires further communication with the client. Used in conjunction with Text(58) to convey the details
	/// of the specific event.
	#[serde(rename = "21")]
	AlgorithmRiskThresholdBreached,
	/// Short sell not permitted
	#[serde(rename = "22")]
	ShortSellNotPermitted,
	/// Short sell rejected due to security pre-borrow restriction
	#[serde(rename = "23")]
	ShortSellRejectedDueToSecurityPreBorrowRestriction,
	/// Short sell rejected due to account pre-borrow restriction
	#[serde(rename = "24")]
	ShortSellRejectedDueToAccountPreBorrowRestriction,
	/// Insufficient credit limit
	#[serde(rename = "25")]
	InsufficientCreditLimit,
	/// Exceeded clip size limit
	#[serde(rename = "26")]
	ExceededClipSizeLimit,
	/// Exceeded maximum notional order amount
	#[serde(rename = "27")]
	ExceededMaximumNotionalOrderAmount,
	/// Exceeded DV01/PV01 limit
	#[serde(rename = "28")]
	ExceededDv01Pv01Limit,
	/// Exceeded CS01 limit
	#[serde(rename = "29")]
	ExceededCs01Limit,
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
	/// Cancel On Connection Loss (This value is used only when unsolicited cancels are generated as a result of a network disconnect)
	#[serde(rename = "12")]
	CancelOnConnectionLoss,
	/// Cancel On Logout (This value is used when unsolicited cancels are generated as a result of a user logout)
	#[serde(rename = "13")]
	CancelOnLogout,
	/// Assign Time Priority
	#[serde(rename = "14")]
	AssignTimePriority,
	/// Canceled, Trade Price Violation
	#[serde(rename = "15")]
	CanceledTradePriceViolation,
	/// Canceled, Cross Imbalance
	#[serde(rename = "16")]
	CanceledCrossImbalance,
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
	/// Equities specialist
	#[serde(rename = "9")]
	EquitiesSpecialist,
	/// Options market maker
	#[serde(rename = "10")]
	OptionsMarketMaker,
	/// Options firm account
	#[serde(rename = "11")]
	OptionsFirmAccount,
	/// Account for customer and non-customer orders
	#[serde(rename = "12")]
	AccountForCustomerAndNonCustomerOrders,
	/// Account for orders from multiple customers
	#[serde(rename = "13")]
	AccountForOrdersFromMultipleCustomers,
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
	/// Internal Cross Order
	#[serde(rename = "A")]
	InternalCrossOrder,
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
	/// Sell undisclosed
	#[serde(rename = "H")]
	SellUndisclosed,
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
	/// Units of Measure per Time Unit (if used - must specify <a href="tag_996_UnitOfMeasure_.html">UnitofMeasure&nbsp;(996)</a> and <a href="tag_997_TimeUnit_.html">TimeUnit&nbsp;(997)</a> )
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
	/// Round lot based upon <a href="tag_996_UnitOfMeasure_.html">UnitOfMeasure&nbsp;(996)</a>
	#[serde(rename = "4")]
	RoundLotBasedUpon,
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
	/// Previous Fund Valuation Point (Historic pricing; for CIV)
	#[serde(rename = "L")]
	PreviousFundValuationPoint,
	/// Next Fund Valuation Point (Forward pricing; for CIV)
	#[serde(rename = "M")]
	NextFundValuationPoint,
	/// Pegged
	#[serde(rename = "P")]
	Pegged,
	/// Counter-order selection
	#[serde(rename = "Q")]
	CounterOrderSelection,
	/// Stop on Bid or Offer (A stop order that is triggered by a bid or offer price movement (quote) at which point the stopped order
	/// becomes a market order, also known as "stop on quote" in some markets (e.g. US markets). In the US equities market it is common
	/// to trigger a stop off the National Best Bid or Offer (NBBO))
	#[serde(rename = "R")]
	StopOnBidOrOffer,
	/// Stop Limit on Bid or Offer (A stop order that is triggered by a bid or offer price movement (quote) at which ponit the stopped
	/// order becomes a limit order, also known as "stop limit on quote" in some markets (e.g. US markets). In the US equities market
	/// it is common to trigger a stop off the National Best Bid or Offer (NBBO)
	#[serde(rename = "S")]
	StopLimitOnBidOrOffer,
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
	/// Good Through Crossing
	#[serde(rename = "8")]
	GoodThroughCrossing,
	/// At Crossing
	#[serde(rename = "9")]
	AtCrossing,
	/// Good For Time
	#[serde(rename = "A")]
	GoodForTime,
	/// Good for auction (GFA)
	#[serde(rename = "B")]
	GoodForAuction,
	/// Good for this Month (GFM)
	#[serde(rename = "C")]
	GoodForThisMonth,
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
	/// Minimum Guaranteed Fill Eligible
	#[serde(rename = "u")]
	MinimumGuaranteedFillEligible,
	/// Bypass Non-Displayed Liquidity
	#[serde(rename = "v")]
	BypassNonDisplayedLiquidity,
	/// Lock (mutually exclusive with q)
	#[serde(rename = "w")]
	Lock,
	/// Ignore notional value checks
	#[serde(rename = "x")]
	IgnoreNotionalValueChecks,
	/// Trade at reference price
	#[serde(rename = "y")]
	TradeAtReferencePrice,
	/// Allow facilitation
	#[serde(rename = "z")]
	AllowFacilitation,
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
	/// Principal
	#[serde(rename = "P")]
	Principal,
	/// Riskless Principal
	#[serde(rename = "R")]
	RisklessPrincipal,
	/// Agent for Other Member
	#[serde(rename = "W")]
	AgentForOtherMember,
	/// Mixed capacity
	#[serde(rename = "M")]
	MixedCapacity,
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
	/// Cross
	#[serde(rename = "F")]
	Cross,
	/// Insider Account
	#[serde(rename = "G")]
	InsiderAccount,
	/// Significant Shareholder
	#[serde(rename = "H")]
	SignificantShareholder,
	/// Normal Course Issuer Bid (NCIB)
	#[serde(rename = "I")]
	NormalCourseIssuerBid,
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
	Continuous,
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
	/// Riskless principal
	#[serde(rename = "5")]
	RisklessPrincipal,
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
pub enum OffshoreIndicator {
	/// Regular. Default if not specified. The notion of onshore and offshore rates does not apply.
	#[serde(rename = "0")]
	Regular,
	/// Offshore. Used to indicate that the rate specified is an offshore rate which differs from its onshore rate.
	#[serde(rename = "1")]
	Offshore,
	/// Onshore. Used to indicate that the rate specified is an onshore rate which differs from its offshore rate.
	#[serde(rename = "2")]
	Onshore,
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
	AutomatedExecutionOrderPrivate,
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
	ExemptBelow,
	/// Exempt - Client Money Type Exemption
	#[serde(rename = "2")]
	ExemptType,
	/// Exempt - Authorised Credit or Financial Institution
	#[serde(rename = "3")]
	ExemptAthorized,
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
	/// Auction execution
	#[serde(rename = "4")]
	AuctionExecution,
	/// Triggered stop order
	#[serde(rename = "5")]
	TriggeredStopOrder,
	/// Triggered contingency order
	#[serde(rename = "6")]
	TriggeredContingencyOrder,
	/// Triggered market order
	#[serde(rename = "7")]
	TriggeredMarketOrder,
	/// Neither added nor removed liquidity
	#[serde(rename = "0")]
	NeitherAddedNorRemovedLiquidity,
	/// Removed liquidity after firm order commitment
	#[serde(rename = "8")]
	RemovedLiquidityAfterFirmOrderCommitment,
	/// Auction execution after firm order commitment
	#[serde(rename = "9")]
	AuctionExecutionAfterFirmOrderCommitment,
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
	PriceAtExecutionPriceAddedAtInitialOrderEntry,
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
	DeliveryInstructionsSellersOption,
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
	Non,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ContingencyType {
	/// One Cancels the Other (OCO)
	#[serde(rename = "1")]
	OneCancelsTheOther,
	/// One Triggers the Other (OTO)
	#[serde(rename = "2")]
	OneTriggersTheOther,
	/// One Updates the Other (OUO) - Absolute Quantity Reduction
	#[serde(rename = "3")]
	OneUpdatesTheOtherAbsolute,
	/// One Updates the Other (OUO) - Proportional Quantity Reduction
	#[serde(rename = "4")]
	OneUpdatesTheOtherProportional,
	/// Bid and Offer (defines a contingency order with two or more sides)
	#[serde(rename = "5")]
	BidAndOffer,
	/// Bid and Offer OCO (defines a contingency order with two or more sides however one side is automatically canceled when the
	/// other side is fully traded away)
	#[serde(rename = "6")]
	BidAndOfferOco,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExDestinationIDSource {
	/// BIC (Bank Identification Code) (ISO 9362)
	#[serde(rename = "B")]
	Bic,
	/// Generally accepted market participant identifier (e.g. NASD mnemonic)
	#[serde(rename = "C")]
	GenerallyAcceptedMarketParticipantIdentifier,
	/// Proprietary / Custom code
	#[serde(rename = "D")]
	Proprietary,
	/// ISO Country Code
	#[serde(rename = "E")]
	IsoCountryCode,
	/// MIC (ISO 10383 - Market Identifier Code)
	#[serde(rename = "G")]
	Mic,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TradePriceNegotiationMethod {
	/// Percent of par
	#[serde(rename = "0")]
	PercentOfPar,
	/// Deal spread
	#[serde(rename = "1")]
	DealSpread,
	/// Upfront points
	#[serde(rename = "2")]
	UpfrontPoints,
	/// Upfront amount
	#[serde(rename = "3")]
	UpfrontAmount,
	/// Percent of par and upfront amount
	#[serde(rename = "4")]
	PercentOfParAndUpfrontAmount,
	/// Deal spread and upfront amount
	#[serde(rename = "5")]
	DealSpreadAndUpfrontAmount,
	/// Upfront points and upfront amount
	#[serde(rename = "6")]
	UpfrontPointsAndUpfrontAmount,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UpfrontPriceType {
	/// Percentage (i.e. percent of par) (often called "dollar price" for fixed income)
	#[serde(rename = "1")]
	Percentage,
	/// Fixed amount (absolute value)
	#[serde(rename = "3")]
	FixedAmount,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ShortSaleExemptionReason {
	/// Exemption reason unknown
	#[serde(rename = "0")]
	ExemptionReasonUnknown,
	/// Income sell short exempt
	#[serde(rename = "1")]
	IncomeSellShortExempt,
	/// Above national best bid (Broker Dealer provision)
	#[serde(rename = "2")]
	AboveNationalBestBid,
	/// Delayed delivery
	#[serde(rename = "3")]
	DelayedDelivery,
	/// Odd lot
	#[serde(rename = "4")]
	OddLot,
	/// Domestic arbitrage
	#[serde(rename = "5")]
	DomesticArbitrage,
	/// International arbitrage
	#[serde(rename = "6")]
	InternationalArbitrage,
	/// Underwriter or syndicate distribution
	#[serde(rename = "7")]
	UnderwriterOrSyndicateDistribution,
	/// Riskless principal
	#[serde(rename = "8")]
	RisklessPrincipal,
	/// VWAP
	#[serde(rename = "9")]
	Vwap,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RefOrderIDSource {
	/// Secondary order ID
	#[serde(rename = "0")]
	SecondaryOrderId,
	/// Order ID
	#[serde(rename = "1")]
	OrderId,
	/// Market data entry ID
	#[serde(rename = "2")]
	MarketDataEntryId,
	/// Quote entry ID
	#[serde(rename = "3")]
	QuoteEntryId,
	/// Original order ID
	#[serde(rename = "4")]
	OriginalOrderId,
	/// Quote ID
	#[serde(rename = "5")]
	QuoteId,
	/// Quote request ID
	#[serde(rename = "6")]
	QuoteRequestId,
	/// Previous order identifier
	#[serde(rename = "7")]
	PreviousOrderIdentifier,
	/// Previous quote identifier
	#[serde(rename = "8")]
	PreviousQuoteIdentifier,
	/// Parent order identifier
	#[serde(rename = "9")]
	ParentOrderIdentifier,
	/// Manual order identifier
	#[serde(rename = "10")]
	ManualOrderIdentifier,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Triggered {
	/// Not triggered (default)
	#[serde(rename = "0")]
	NotTriggered,
	/// Triggered
	#[serde(rename = "1")]
	Triggered,
	/// Stop order triggered
	#[serde(rename = "2")]
	StopOrderTriggered,
	/// One Cancels the Other (OCO) order triggered
	#[serde(rename = "3")]
	OneCancelsTheOther,
	/// One Triggers the Other (OTO) order triggered
	#[serde(rename = "4")]
	OneTriggersTheOther,
	/// One Updates the Other (OUO) order triggered
	#[serde(rename = "5")]
	OneUpdatesTheOther,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AuctionInstruction {
	/// Automated auction premitted
	#[serde(rename = "0")]
	AutomatedAuctionPremitted,
	/// Automated auction not permitted
	#[serde(rename = "1")]
	AutomatedAuctionNotPermitted,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TradingCapacity {
	/// Customer
	#[serde(rename = "1")]
	Customer,
	/// Customer professional
	#[serde(rename = "2")]
	CustomerProfessional,
	/// Broker-dealer
	#[serde(rename = "3")]
	BrokerDealer,
	/// Customer broker-dealer
	#[serde(rename = "4")]
	CustomerBrokerDealer,
	/// Principal
	#[serde(rename = "5")]
	Principal,
	/// Market maker
	#[serde(rename = "6")]
	MarketMaker,
	/// Away market maker
	#[serde(rename = "7")]
	AwayMarketMaker,
	/// Systematic internaliser
	#[serde(rename = "8")]
	SystematicInternaliser,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TradePublishIndicator {
	/// Do Not Publish Trade
	#[serde(rename = "0")]
	DoNotPublishTrade,
	/// Publish Trade
	#[serde(rename = "1")]
	PublishTrade,
	/// Deferred Publication
	#[serde(rename = "2")]
	DeferredPublication,
	/// Published
	#[serde(rename = "3")]
	Published,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MinQtyMethod {
	/// Once (applies only to first execution)
	#[serde(rename = "1")]
	Once,
	/// Multiple (applies to every execution)
	#[serde(rename = "2")]
	Multiple,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClearingAccountType {
	/// Customer
	#[serde(rename = "1")]
	Customer,
	/// Firm
	#[serde(rename = "2")]
	Firm,
	/// Market maker
	#[serde(rename = "3")]
	MarketMaker,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AuctionType {
	/// None
	#[serde(rename = "0")]
	None,
	/// Block order auction
	#[serde(rename = "1")]
	BlockOrderAuction,
	/// Directed order auction
	#[serde(rename = "2")]
	DirectedOrderAuction,
	/// Exposure order auction
	#[serde(rename = "3")]
	ExposureOrderAuction,
	/// Flash order auction
	#[serde(rename = "4")]
	FlashOrderAuction,
	/// Facilitation order auction
	#[serde(rename = "5")]
	FacilitationOrderAuction,
	/// Solicitation order auction
	#[serde(rename = "6")]
	SolicitationOrderAuction,
	/// Price improvement mechanism
	#[serde(rename = "7")]
	PriceImprovementMechanism,
	/// Directed order price improvement mechanism
	#[serde(rename = "8")]
	DirectedOrderPriceImprovementMechanism,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LockType {
	/// Not locked
	#[serde(rename = "0")]
	NotLocked,
	/// Away market netter
	#[serde(rename = "1")]
	AwayMarketNetter,
	/// Three tick locked
	#[serde(rename = "2")]
	ThreeTickLocked,
	/// Locked by market maker
	#[serde(rename = "3")]
	LockedByMarketMaker,
	/// Directed order lock
	#[serde(rename = "4")]
	DirectedOrderLock,
	/// Multileg lock
	#[serde(rename = "5")]
	MultilegLock,
	/// Market order lock
	#[serde(rename = "6")]
	MarketOrderLock,
	/// Pre-assignment lock
	#[serde(rename = "7")]
	PreAssignmentLock,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ReleaseInstruction {
	/// Intermarket Sweep Order (ISO)
	#[serde(rename = "1")]
	IntermarketSweepOrder,
	/// No away market better check
	#[serde(rename = "2")]
	NoAwayMarketBetterCheck,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RelatedPriceSource {
	/// NBB (National Best Bid)
	#[serde(rename = "1")]
	Nbb,
	/// NBO (National Best Offer)
	#[serde(rename = "2")]
	Nbo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OrderOrigination {
	/// Order received from a customer
	#[serde(rename = "1")]
	OrderReceivedFromACustomer,
	/// Order received from within the firm
	#[serde(rename = "2")]
	OrderReceivedFromWithinTheFirm,
	/// Order received from another broker-dealer
	#[serde(rename = "3")]
	OrderReceivedFromAnotherBrokerDealer,
	/// Order received from a customer or originated from within the firm
	#[serde(rename = "4")]
	OrderReceivedFromACustomerOrOriginatedFromWithinTheFirm,
	/// Order received from a direct access or sponsored access customer
	#[serde(rename = "5")]
	OrderReceivedFromADirectAccessOrSponsoredAccessCustomer,
	/// Order received from a foreign dealer equivalent
	#[serde(rename = "6")]
	OrderReceivedFromAForeignDealerEquivalent,
	/// Order received from an execution-only service
	#[serde(rename = "7")]
	OrderReceivedFromAnExecutionOnlyService,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OwnerType {
	/// Individual Investor
	#[serde(rename = "1")]
	IndividualInvestor,
	/// Public Company
	#[serde(rename = "2")]
	PublicCompany,
	/// Private Company
	#[serde(rename = "3")]
	PrivateCompany,
	/// Individual Trustee
	#[serde(rename = "4")]
	IndividualTrustee,
	/// Company Trustee
	#[serde(rename = "5")]
	CompanyTrustee,
	/// Pension Plan
	#[serde(rename = "6")]
	PensionPlan,
	/// Custodian Under Gifts to Minors Act
	#[serde(rename = "7")]
	CustodianUnderGiftsToMinorsAct,
	/// Trusts
	#[serde(rename = "8")]
	Trusts,
	/// Fiduciaries
	#[serde(rename = "9")]
	Fiduciaries,
	/// Networking Sub-Account
	#[serde(rename = "10")]
	NetworkingSubAccount,
	/// Non-Profit Organization
	#[serde(rename = "11")]
	NonProfitOrganization,
	/// Corporate Body
	#[serde(rename = "12")]
	CorporateBody,
	/// Nominee
	#[serde(rename = "13")]
	Nominee,
	/// Institutional customer
	#[serde(rename = "14")]
	InstitutionalCustomer,
	/// Combined (Elaboration: Representing more than one type of beneficial owner account)
	#[serde(rename = "15")]
	Combined,
	/// Member firm employee or associated person
	#[serde(rename = "16")]
	MemberFirmEmployeeOrAssociatedPerson,
	/// Market making account
	#[serde(rename = "17")]
	MarketMakingAccount,
	/// Proprietary account
	#[serde(rename = "18")]
	ProprietaryAccount,
	/// Non-broker-dealer
	#[serde(rename = "19")]
	NonBrokerDealer,
	/// Unknown beneficial owner type
	#[serde(rename = "20")]
	UnknownBeneficialOwnerType,
	/// Error account of firm
	#[serde(rename = "21")]
	ErrorAccountOfFirm,
	/// Firm agency average price account
	#[serde(rename = "22")]
	FirmAgencyAveragePriceAccount,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ShortMarkingExemptIndicator {
	/// Account is not SME
	#[serde(rename = "N")]
	AccountIsNotSme,
	/// Account is SME
	#[serde(rename = "Y")]
	AccountIsSme,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RefRiskLimitCheckIDType {
	/// RiskLimitRequestID(1666)
	#[serde(rename = "0")]
	RiskLimitRequestId,
	/// RiskLimitCheckID(2319)
	#[serde(rename = "1")]
	RiskLimitCheckId,
	/// Out of band identifier
	#[serde(rename = "3")]
	OutOfBandIdentifier3,
	/// Out of band identifier
	#[serde(rename = "2")]
	OutOfBandIdentifier2,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TrdType {
	/// Regular Trade
	#[serde(rename = "0")]
	RegularTrade,
	/// Block Trade
	#[serde(rename = "1")]
	BlockTrade,
	/// EFP (Exchange for physical)
	#[serde(rename = "2")]
	Efp,
	/// Transfer
	#[serde(rename = "3")]
	Transfer,
	/// Late Trade
	#[serde(rename = "4")]
	LateTrade,
	/// T Trade
	#[serde(rename = "5")]
	TTrade,
	/// Weighted Average Price Trade
	#[serde(rename = "6")]
	WeightedAveragePriceTrade,
	/// Bunched Trade
	#[serde(rename = "7")]
	BunchedTrade,
	/// Late Bunched Trade
	#[serde(rename = "8")]
	LateBunchedTrade,
	/// Prior Reference Price Trade
	#[serde(rename = "9")]
	PriorReferencePriceTrade,
	/// After Hours Trade
	#[serde(rename = "10")]
	AfterHoursTrade,
	/// Exchange for Risk (EFR)
	#[serde(rename = "11")]
	ExchangeForRisk,
	/// Exchange for Swap (EFS )
	#[serde(rename = "12")]
	ExchangeForSwap,
	/// Exchange of Futures for (in Market) Futures (EFM ) (e,g, full sized for mini)
	#[serde(rename = "13")]
	ExchangeOfFuturesFor,
	/// Exchange of Options for Options (EOO)
	#[serde(rename = "14")]
	ExchangeOfOptionsForOptions,
	/// Trading at Settlement
	#[serde(rename = "15")]
	TradingAtSettlement,
	/// All or None
	#[serde(rename = "16")]
	AllOrNone,
	/// Futures Large Order Execution
	#[serde(rename = "17")]
	FuturesLargeOrderExecution,
	/// Exchange of Futures for Futures (external market) (EFF)
	#[serde(rename = "18")]
	ExchangeOfFuturesForFutures,
	/// Option Interim Trade
	#[serde(rename = "19")]
	OptionInterimTrade,
	/// Option Cabinet Trade
	#[serde(rename = "20")]
	OptionCabinetTrade,
	/// Privately Negotiated Trades
	#[serde(rename = "22")]
	PrivatelyNegotiatedTrades,
	/// Substitution of Futures for Forwards
	#[serde(rename = "23")]
	SubstitutionOfFuturesForForwards,
	/// Error trade
	#[serde(rename = "24")]
	ErrorTrade,
	/// Special cum dividend (CD)
	#[serde(rename = "25")]
	SpecialCumDividend,
	/// Special ex dividend (XD)
	#[serde(rename = "26")]
	SpecialExDividend,
	/// Special cum coupon (CC)
	#[serde(rename = "27")]
	SpecialCumCoupon,
	/// Special ex coupon (XC)
	#[serde(rename = "28")]
	SpecialExCoupon,
	/// Cash settlement (CS)
	#[serde(rename = "29")]
	CashSettlement,
	/// Special price (usually net- or all-in price) (SP)
	#[serde(rename = "30")]
	SpecialPrice,
	/// Guaranteed delivery (GD)
	#[serde(rename = "31")]
	GuaranteedDelivery,
	/// Special cum rights (CR)
	#[serde(rename = "32")]
	SpecialCumRights,
	/// Special ex rights (XR)
	#[serde(rename = "33")]
	SpecialExRights,
	/// Special cum capital repayments (CP)
	#[serde(rename = "34")]
	SpecialCumCapitalRepayments,
	/// Special ex capital repayments (XP)
	#[serde(rename = "35")]
	SpecialExCapitalRepayments,
	/// Special cum bonus (CB)
	#[serde(rename = "36")]
	SpecialCumBonus,
	/// Special ex bonus (XB)
	#[serde(rename = "37")]
	SpecialExBonus,
	/// Block trade (same as large trade)
	#[serde(rename = "38")]
	BlockTradeLarge,
	/// Worked principal trade (UK-specific)
	#[serde(rename = "39")]
	WorkedPrincipalTrade,
	/// Block Trades - after market
	#[serde(rename = "40")]
	BlockTrades,
	/// Name change
	#[serde(rename = "41")]
	NameChange,
	/// Portfolio transfer
	#[serde(rename = "42")]
	PortfolioTransfer,
	/// Prorogation buy - Euronext Paris only. Is used to defer settlement under French SRD (deferred settlement system) . Trades
	/// must be reported as crosses at zero price
	#[serde(rename = "43")]
	ProrogationBuy,
	/// Prorogation sell - see prorogation buy
	#[serde(rename = "44")]
	ProrogationSell,
	/// Option exercise
	#[serde(rename = "45")]
	OptionExercise,
	/// Delta neutral transaction
	#[serde(rename = "46")]
	DeltaNeutralTransaction,
	/// Financing transaction (includes repo and stock lending)
	#[serde(rename = "47")]
	FinancingTransaction,
	/// Non-standard settlement
	#[serde(rename = "48")]
	NonStandardSettlement,
	/// Derivative related transaction
	#[serde(rename = "49")]
	DerivativeRelatedTransaction,
	/// Portfolio trade
	#[serde(rename = "50")]
	PortfolioTrade,
	/// Volume weighted average trade
	#[serde(rename = "51")]
	VolumeWeightedAverageTrade,
	/// Exchange granted trade
	#[serde(rename = "52")]
	ExchangeGrantedTrade,
	/// Repurchase agreement
	#[serde(rename = "53")]
	RepurchaseAgreement,
	/// OTC
	#[serde(rename = "54")]
	Otc,
	/// Exchange absis facility (EBF)
	#[serde(rename = "55")]
	ExchangeAbsisFacility,
	/// Opening Trade
	#[serde(rename = "56")]
	OpeningTrade,
	/// Netted trade
	#[serde(rename = "57")]
	NettedTrade,
	/// Block swap trade or large notional off-facility swap
	#[serde(rename = "58")]
	BlockSwapTradeOrLargeNotionalOffFacilitySwap,
	/// Give-up/Give-in trade
	#[serde(rename = "61")]
	GiveUpGiveInTrade,
	/// Dark trade (Elaboration: A Market Model Typology dark trade might also come from a lit/hybrid book, when an aggressive lit
	/// order hits a resting dark order.)
	#[serde(rename = "62")]
	DarkTrade,
	/// Technical trade
	#[serde(rename = "63")]
	TechnicalTrade,
	/// Benchmark
	#[serde(rename = "64")]
	Benchmark,
	/// Credit event trade
	#[serde(rename = "59")]
	CreditEventTrade,
	/// Succession event trade
	#[serde(rename = "60")]
	SuccessionEventTrade,
	/// Package Trade (Identifies the pseudotrade of a stream or collection of trades to be cleared and be reported as an atomic unit.
	/// The subsequent actual trades reported should not have this value)
	#[serde(rename = "65")]
	PackageTrade,
	/// Roll trade
	#[serde(rename = "66")]
	RollTrade,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RegulatoryTransactionType {
	/// None (Default if not specified) (Elaboration: The transaction does not fall under any special regulatory rule or mandate.)
	#[serde(rename = "0")]
	None,
	/// Swap Execution Facility (SEF) Required Transaction (Elaboration: The transaction is a Required transaction under DoddFrank
	/// Act SEF Rules. Required transactions are subject to the trade execution mandate under section 2(h)(8) of the CEA and are not
	/// block trades)
	#[serde(rename = "1")]
	SwapExecutionFacilityRequired,
	/// Swap Execution Facility (SEF) Permitted Transaction (Elaboration: The transaction is a Permitted transaction under Dodd-Frank
	/// Act SEF Rules. Permitted transactions are not subject to the clearing and trade execution mandates, illiquid or bespoke swaps,
	/// or block trades)
	#[serde(rename = "2")]
	SwapExecutionFacilityPermitted,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExecTypeReason {
	/// Order added upon request
	#[serde(rename = "1")]
	OrderAddedUponRequest,
	/// Order replaced upon request
	#[serde(rename = "2")]
	OrderReplacedUponRequest,
	/// Order cancelled upon request
	#[serde(rename = "3")]
	OrderCancelledUponRequest,
	/// Unsolicited order cancellation
	#[serde(rename = "4")]
	UnsolicitedOrderCancellation,
	/// Non-resting order added upon request
	#[serde(rename = "5")]
	NonRestingOrderAddedUponRequest,
	/// Order replaced with non-resting order upon request
	#[serde(rename = "6")]
	OrderReplacedWithNonRestingOrderUponRequest,
	/// Trigger order replaced upon request
	#[serde(rename = "7")]
	TriggerOrderReplacedUponRequest,
	/// Suspended order replaced upon request
	#[serde(rename = "8")]
	SuspendedOrderReplacedUponRequest,
	/// Suspended order canceled upon request
	#[serde(rename = "9")]
	SuspendedOrderCanceledUponRequest,
	/// Order cancellation pending
	#[serde(rename = "10")]
	OrderCancellationPending,
	/// Pending cancellation executed
	#[serde(rename = "11")]
	PendingCancellationExecuted,
	/// Resting order triggered
	#[serde(rename = "12")]
	RestingOrderTriggered,
	/// Suspended order activated
	#[serde(rename = "13")]
	SuspendedOrderActivated,
	/// Active order suspended
	#[serde(rename = "14")]
	ActiveOrderSuspended,
	/// Order expired
	#[serde(rename = "15")]
	OrderExpired,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExposureDurationUnit {
	/// Seconds (default if not specified)
	#[serde(rename = "0")]
	Seconds,
	/// Tenths of a second
	#[serde(rename = "1")]
	TenthsOfASecond,
	/// Hundredths of a second
	#[serde(rename = "2")]
	HundredthsOfASecond,
	/// milliseconds
	#[serde(rename = "3")]
	Milliseconds,
	/// microseconds
	#[serde(rename = "4")]
	Microseconds,
	/// nanoseconds
	#[serde(rename = "5")]
	Nanoseconds,
	/// minutes
	#[serde(rename = "10")]
	Minutes,
	/// hours
	#[serde(rename = "11")]
	Hours,
	/// days
	#[serde(rename = "12")]
	Days,
	/// weeks
	#[serde(rename = "13")]
	Weeks,
	/// months
	#[serde(rename = "14")]
	Months,
	/// years
	#[serde(rename = "15")]
	Years,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VenueType {
	/// Electronic
	#[serde(rename = "E")]
	Electronic,
	/// Bit
	#[serde(rename = "P")]
	Bit,
	/// Ex-Pit
	#[serde(rename = "X")]
	ExPit,
	/// Clearing house
	#[serde(rename = "C")]
	ClearingHouse,
	/// Registered market
	#[serde(rename = "R")]
	RegisteredMarket,
	/// Off-market
	#[serde(rename = "O")]
	OffMarket,
	/// Central limit order book
	#[serde(rename = "B")]
	CentralLimitOrderBook,
	/// Quote driven market
	#[serde(rename = "Q")]
	QuoteDrivenMarket,
	/// Dark order book
	#[serde(rename = "D")]
	DarkOrderBook,
	/// Auction driven market
	#[serde(rename = "A")]
	AuctionDrivenMarket,
	/// Quote negotiation
	#[serde(rename = "N")]
	QuoteNegotiation,
	/// Voice neotiation
	#[serde(rename = "V")]
	VoiceNeotiation,
	/// Hybrid market
	#[serde(rename = "H")]
	HybridMarket,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CrossedIndicator {
	/// No cross
	#[serde(rename = "0")]
	NoCross,
	/// Cross rejected
	#[serde(rename = "1")]
	CrossRejected,
	/// Cross accepted
	#[serde(rename = "2")]
	CrossAccepted,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AlgorithmicTradeIndicator {
	/// Non-algorithmic trade
	#[serde(rename = "0")]
	NonAlgorithmicTrade,
	/// Algorithmic trade
	#[serde(rename = "1")]
	AlgorithmicTrade,
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
	Nm,
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
	R,
	/// RO (Transaction which resulted from the exercise of a traditional option or a stock-settled covered warrant)
	#[serde(rename = "27")]
	Ro,
	/// RT (Risk transaction in a SEATS security, (excluding AIM security) reported by a market maker registered in that security)
	#[serde(rename = "28")]
	Rt,
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
	TradeAtCashOpen,
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
pub enum TradeReportingIndicator {
	/// Trade has not (yet) been reported
	#[serde(rename = "0")]
	TradeHasNot,
	/// Trade has been or will be reported by a trading venue as an "on-book" trade
	#[serde(rename = "1")]
	TradeHasBeenOrWillBeReportedByATradingVenueAsAnOnBook,
	/// Trade has been or will be reported as a "systematic internaliser" seller trade
	#[serde(rename = "2")]
	TradeHasBeenOrWillBeReportedAsASeller,
	/// Trade has been or will be reported as a "systematic internaliser" buyer trade
	#[serde(rename = "3")]
	TradeHasBeenOrWillBeReportedAsABuyer,
	/// Trade has been or will be reported as a "non-systematic internaliser" seller trade
	#[serde(rename = "4")]
	TradeHasBeenOrWillBeReportedAsANonSeller,
	/// Trade has been or will be reported under a sub-delegation arrangement by an investment firm to a reporting facility (e.g.
	/// APA) on behalf of another investment firm
	#[serde(rename = "5")]
	TradeHasBeenOrWillBeReportedUnderASubDelegationArrangementByAnInvestmentFirmToAReportingFacility,
	/// Trade has been or will be reported
	#[serde(rename = "6")]
	TradeHasBeenOrWillBeReported,
	/// Trade has been or will be reported as a "non-Systematic Internaliser" buyer trade
	#[serde(rename = "7")]
	TradeHasBeenOrWillBeReportedAsA,
	/// Trade has been or will be reported by a trading venue as an "off-book" trade
	#[serde(rename = "8")]
	TradeHasBeenOrWillBeReportedByATradingVenueAsAnOffBook,
	/// Trade is not reportable
	#[serde(rename = "9")]
	TradeIsNotReportable,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OrderOwnershipIndicator {
	/// No change of ownership (default)
	#[serde(rename = "0")]
	NoChangeOfOwnership,
	/// Change of ownership to executing party
	#[serde(rename = "1")]
	ChangeOfOwnershipToExecutingParty,
	/// Change of ownership to entering party
	#[serde(rename = "2")]
	ChangeOfOwnershipToEnteringParty,
	/// Change of ownership to specified party
	#[serde(rename = "3")]
	ChangeOfOwnershipToSpecifiedParty,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExDestinationType {
	/// No restriction
	#[serde(rename = "0")]
	NoRestriction,
	/// Can be traded only on a trading venue
	#[serde(rename = "1")]
	CanBeTradedOnlyOnATradingVenue,
	/// Can be traded only on a Systematic Internaliser (SI)
	#[serde(rename = "2")]
	CanBeTradedOnlyOnASystematicInternaliser,
	/// Can be traded on a trading venue or Systematic internaliser (SI)
	#[serde(rename = "3")]
	CanBeTradedOnATradingVenueOrSystematicInternaliser,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EventInitiatorType {
	/// Customer or client
	#[serde(rename = "C")]
	CustomerOrClient,
	/// Exchange or execution venue
	#[serde(rename = "E")]
	ExchangeOrExecutionVenue,
	/// Firm or broker
	#[serde(rename = "F")]
	FirmOrBroker,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RoutingArrangmentIndicator {
	/// No routing arrangement in place
	#[serde(rename = "0")]
	NoRoutingArrangementInPlace,
	/// Routing arrangement in place
	#[serde(rename = "1")]
	RoutingArrangementInPlace,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TradeContinuation {
	/// Novation
	#[serde(rename = "0")]
	Novation,
	/// Partial novation
	#[serde(rename = "1")]
	PartialNovation,
	/// Trade unwind
	#[serde(rename = "2")]
	TradeUnwind,
	/// Partial trade unwind
	#[serde(rename = "3")]
	PartialTradeUnwind,
	/// Exercise
	#[serde(rename = "4")]
	Exercise,
	/// Compression/Netting
	#[serde(rename = "5")]
	CompressionNetting,
	/// Full netting
	#[serde(rename = "6")]
	FullNetting,
	/// Partial netting
	#[serde(rename = "7")]
	PartialNetting,
	/// Amendment
	#[serde(rename = "8")]
	Amendment,
	/// Increase
	#[serde(rename = "9")]
	Increase,
	/// Credit event
	#[serde(rename = "10")]
	CreditEvent,
	/// Strategic restructuring
	#[serde(rename = "11")]
	StrategicRestructuring,
	/// Succession event reorganization
	#[serde(rename = "12")]
	SuccessionEventReorganization,
	/// Succession event renaming
	#[serde(rename = "13")]
	SuccessionEventRenaming,
	/// Porting
	#[serde(rename = "14")]
	Porting,
	/// Withdrawal
	#[serde(rename = "15")]
	Withdrawal,
	/// Void
	#[serde(rename = "16")]
	Void,
	/// Other price-forming continuation data
	#[serde(rename = "99")]
	OtherPriceFormingContinuationData,
	/// Account transfer
	#[serde(rename = "17")]
	AccountTransfer,
	/// Give up
	#[serde(rename = "18")]
	GiveUp,
	/// Take up
	#[serde(rename = "19")]
	TakeUp,
	/// Average pricing
	#[serde(rename = "20")]
	AveragePricing,
	/// Reversal
	#[serde(rename = "21")]
	Reversal,
	/// Allocation / Trade posting
	#[serde(rename = "22")]
	Allocation,
	/// Cascade
	#[serde(rename = "23")]
	Cascade,
	/// Delivery
	#[serde(rename = "24")]
	Delivery,
	/// Option assignment
	#[serde(rename = "25")]
	OptionAssignment,
	/// Expiration
	#[serde(rename = "26")]
	Expiration,
	/// Maturity
	#[serde(rename = "27")]
	Maturity,
	/// Equal position adjustment
	#[serde(rename = "28")]
	EqualPositionAdjustment,
	/// Unequal position adjustment
	#[serde(rename = "29")]
	UnequalPositionAdjustment,
	/// Correction
	#[serde(rename = "30")]
	Correction,
	/// Early termination
	#[serde(rename = "31")]
	EarlyTermination,
	/// Rerate
	#[serde(rename = "32")]
	Rerate,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RegulatoryReportType {
	/// Real-time (RT)
	#[serde(rename = "0")]
	RealTime,
	/// Primary economic terms (PET)
	#[serde(rename = "1")]
	PrimaryEconomicTerms,
	/// Snapshot
	#[serde(rename = "2")]
	Snapshot,
	/// Confirmation
	#[serde(rename = "3")]
	Confirmation,
	/// Combination of RT and PET
	#[serde(rename = "4")]
	CombinationOfRtAndPet,
	/// Combination of PET and confirmation
	#[serde(rename = "5")]
	CombinationOfPetAndConfirmation,
	/// Combination of RT, PET and confirmation
	#[serde(rename = "6")]
	CombinationOfRt,
	/// Post-trade valuation
	#[serde(rename = "7")]
	PostTradeValuation,
	/// Verification
	#[serde(rename = "8")]
	Verification,
	/// Post-trade event
	#[serde(rename = "9")]
	PostTradeEvent,
	/// Post trade event RT reportable (Report of a regulated transaction continuation event that falls within the requirements for
	/// real-time reporting and public dissemination. If dissemination is to be suppressed due to an end user exception or to local
	/// regulatory rules that allow suppression of certain types of transactions, use TradePublishIndicator(1390) = 0 (Do not publish
	/// trade)
	#[serde(rename = "10")]
	PostTradeEventRtReportable,
	/// Limited Details Trade
	#[serde(rename = "11")]
	LimitedDetailsTrade,
	/// Daily Aggregated Trade
	#[serde(rename = "12")]
	DailyAggregatedTrade,
	/// Volume Omission Trade
	#[serde(rename = "13")]
	VolumeOmissionTrade,
	/// Four Weeks Aggregation Trade
	#[serde(rename = "14")]
	FourWeeksAggregationTrade,
	/// Indefinite Aggregation Trade
	#[serde(rename = "15")]
	IndefiniteAggregationTrade,
	/// Volume Omission Trade Eligible for Subsequent Aggregated Enrichment
	#[serde(rename = "16")]
	VolumeOmissionTradeEligibleForSubsequentAggregatedEnrichment,
	/// Full Details Trade of "Limited Details Trade"
	#[serde(rename = "17")]
	FullDetailsTradeOf,
	/// Full Details of "Daily Aggregated Trade"
	#[serde(rename = "18")]
	FullDetailsOfDailyAggregated,
	/// Full Details of "Volume Omission Trade"
	#[serde(rename = "19")]
	FullDetailsOfVolumeOmission,
	/// Full Details of "Four Weeks Aggregation Trade"
	#[serde(rename = "20")]
	FullDetailsOfFourWeeks,
	/// Full Details in Aggregated Form of "Volume Omission Trade Eligible for Subsequent Aggregated Enrichment"
	#[serde(rename = "21")]
	FullDetailsInAggregatedFormOf,
	/// Order
	#[serde(rename = "22")]
	Order,
	/// Child order
	#[serde(rename = "23")]
	ChildOrder,
	/// Order route
	#[serde(rename = "24")]
	OrderRoute,
	/// Trade
	#[serde(rename = "25")]
	Trade,
	/// Quote
	#[serde(rename = "26")]
	Quote,
	/// Supplement
	#[serde(rename = "27")]
	Supplement,
	/// New transaction
	#[serde(rename = "28")]
	NewTransaction,
	/// Transaction correction
	#[serde(rename = "29")]
	TransactionCorrection,
	/// Transaction modification
	#[serde(rename = "30")]
	TransactionModification,
	/// Collateral update
	#[serde(rename = "31")]
	CollateralUpdate,
	/// Margin update
	#[serde(rename = "32")]
	MarginUpdate,
	/// Transaction reported in error
	#[serde(rename = "33")]
	TransactionReportedInError,
	/// Termination / Early termination
	#[serde(rename = "34")]
	Termination,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AffiliatedFirmsTradeIndicator {
	/// Transaction or position is not between two affiliated firms
	#[serde(rename = "N")]
	TransactionOrPositionIsNotBetweenTwoAffiliatedFirms,
	/// Transaction or position is between two affiliated firms
	#[serde(rename = "Y")]
	TransactionOrPositionIsBetweenTwoAffiliatedFirms,
}
