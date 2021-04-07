
use serde::{Deserialize, Serialize};

use crate::entities::{ApplVerID, Boolean, Currency, EncodedText, LocalMktDate, RepeatingValues, SeparatedValues, UTCTimestamp, fix50::underlying::Underlying, fix50sp2::{commission_data::CommissionData, discretion_instructions::DiscretionInstructions, display_instruction::DisplayInstruction, financing_details::FinancingDetails, instrument::Instrument, order_qty_data::OrderQtyData, parties::Parties, peg_instructions::PegInstructions, spread_or_benchmark_curve_data::SpreadOrBenchmarkCurveData, stipulations::Stipulations, trd_reg_timestamps::TrdRegTimestamps, triggering_instruction::TriggeringInstruction, yield_data::YieldData}, fixt11::{Trailer, header::{HasHeader, Header, MsgType}}, version::FixVersion};

/// MsgType = D
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NewOrderSingle {
	/// Standard Message Header
	#[serde(flatten)]
	pub header: Header,
	/// Unique identifier of the order as assigned by institution or by the intermediary (CIV term, not a hub/service bureau) with
	/// closest association with the investor.
	#[serde(rename = "11")]
	pub cl_ord_id: String,
	/// SecondaryClOrdID
	#[serde(rename = "526")]
	pub secondary_cl_ord_id: Option<String>,
	/// ClOrdLinkID
	#[serde(rename = "583")]
	pub cl_ord_link_id: Option<String>,
	/// Insert here the set of "Parties" (firm identification) fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub parties: Option<Parties>,
	/// TradeOriginationDate
	#[serde(rename = "229")]
	pub trade_origination_date: Option<LocalMktDate>,
	/// TradeDate
	#[serde(rename = "75")]
	pub trade_date: Option<LocalMktDate>,
	/// Account
	#[serde(rename = "1")]
	pub account: Option<String>,
	/// AcctIDSource
	#[serde(rename = "660")]
	pub acct_id_source: Option<AcctIDSource>,
	/// Type of account associated with the order (Origin)
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
	/// Used to assign an overall allocation id to the block of preallocations
	#[serde(rename = "70")]
	pub alloc_id: Option<String>,
	/// Number of repeating groups for pre-trade allocation
	#[serde(rename = "78")]
	pub allocs: Option<RepeatingValues<Alloc>>,
	/// SettlType
	#[serde(rename = "63")]
	pub settl_type: Option<SettlType>,
	/// Takes precedence over <a href="tag_63_SettlType_.html">SettlType&nbsp;(63)</a> value and conditionally required/omitted for specific <a href="tag_63_SettlType_.html">SettlType&nbsp;(63)</a> values.
	#[serde(rename = "64")]
	pub settl_date: Option<LocalMktDate>,
	/// CashMargin
	#[serde(rename = "544")]
	pub cash_margin: Option<CashMargin>,
	/// ClearingFeeIndicator
	#[serde(rename = "635")]
	pub clearing_fee_indicator: Option<ClearingFeeIndicator>,
	/// HandlInst
	#[serde(rename = "21")]
	pub handl_inst: Option<HandlInst>,
	/// Can contain multiple instructions, space delimited. If <a href="tag_40_OrdType_.html">OrdType&nbsp;(40)</a> =P, exactly one of the following values ( <a href="tag_18_ExecInst_.html">ExecInst&nbsp;(18)</a> = L, R, M, P, O, T, W, a, d) must be specified.
	#[serde(rename = "18")]
	pub exec_inst: Option<SeparatedValues<ExecInst>>,
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
	/// (Deprecated in FIX.5.0)
	#[serde(rename = "111")]
	pub max_floor: Option<f64>,
	/// ExDestination
	#[serde(rename = "100")]
	pub ex_destination: Option<String>,
	/// ExDestinationIDSource
	#[serde(rename = "1133")]
	pub ex_destination_id_source: Option<ExDestinationIDSource>,
	/// Specifies the number of repeating TradingSessionIDs
	#[serde(rename = "386")]
	pub trading_sessions: Option<RepeatingValues<TradingSession>>,
	/// Used to identify soft trades at order entry.
	#[serde(rename = "81")]
	pub process_code: Option<ProcessCode>,
	/// Insert here the set of "Instrument" fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub instrument: Instrument,
	/// Insert here the set of "FinancingDetails" fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub financing_details: Option<FinancingDetails>,
	/// Number of underlyings
	#[serde(rename = "711")]
	pub underlyings: Option<RepeatingValues<Underlying>>,
	/// Useful for verifying security identification
	#[serde(rename = "140")]
	pub prev_close_px: Option<f64>,
	/// Side
	#[serde(rename = "54")]
	pub side: Side,
	/// Required for short sell orders
	#[serde(rename = "114")]
	pub locate_reqd: Option<LocateReqd>,
	/// Time this order request was initiated/released by the trader, trading system, or intermediary.
	#[serde(rename = "60")]
	pub transact_time: UTCTimestamp,
	/// Insert here the set of "Stipulations" (repeating group of Fixed Income stipulations) fields defined in "Common Components
	/// of Application Messages".
	#[serde(flatten)]
	pub stipulations: Option<Stipulations>,
	/// QtyType
	#[serde(rename = "854")]
	pub qty_type: Option<QtyType>,
	/// Insert here the set of "OrderQtyData" fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub order_qty_data: OrderQtyData,
	/// OrdType
	#[serde(rename = "40")]
	pub ord_type: OrdType,
	/// PriceType
	#[serde(rename = "423")]
	pub price_type: Option<PriceType>,
	/// Required for limit OrdTypes. For F/X orders, should be the "all-in" rate (spot rate adjusted for forward points). Can be used
	/// to specify a limit price for a pegged order, previously indicated, etc.
	#[serde(rename = "44")]
	pub price: Option<f64>,
	/// PriceProtectionScope
	#[serde(rename = "1092")]
	pub price_protection_scope: Option<PriceProtectionScope>,
	/// Required for <a href="tag_40_OrdType_.html">OrdType&nbsp;(40)</a> = "Stop" or <a href="tag_40_OrdType_.html">OrdType&nbsp;(40)</a> = "Stop limit".
	#[serde(rename = "99")]
	pub stop_px: Option<f64>,
	/// Insert here the set of "TriggeringInstruction" fields defined in "common components of application messages".
	#[serde(flatten)]
	pub triggering_instruction: Option<TriggeringInstruction>,
	/// Insert here the set of "SpreadOrBenchmarkCurveData" (Fixed Income spread or benchmark curve) fields defined in "Common Components
	/// of Application Messages".
	#[serde(flatten)]
	pub spread_or_benchmark_curve_data: Option<SpreadOrBenchmarkCurveData>,
	/// Insert here the set of "YieldData" (yield-related) fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub yield_data: Option<YieldData>,
	/// Currency
	#[serde(rename = "15")]
	pub currency: Option<Currency>,
	/// ComplianceID
	#[serde(rename = "376")]
	pub compliance_id: Option<String>,
	/// SolicitedFlag
	#[serde(rename = "377")]
	pub solicited_flag: Option<SolicitedFlag>,
	/// Required for Previously Indicated Orders (OrdType=E)
	#[serde(rename = "23")]
	pub ioiid: Option<String>,
	/// Required for Previously Quoted Orders (OrdType=D)
	#[serde(rename = "117")]
	pub quote_id: Option<String>,
	/// Absence of this field indicates Day order
	#[serde(rename = "59")]
	pub time_in_force: Option<TimeInForce>,
	/// Can specify the time at which the order should be considered valid
	#[serde(rename = "168")]
	pub effective_time: Option<UTCTimestamp>,
	/// Conditionally required if <a href="tag_59_TimeInForce_.html">TimeInForce&nbsp;(59)</a> = GTD and <a href="tag_126_ExpireTime_.html">ExpireTime&nbsp;(126)</a> is not specified.
	#[serde(rename = "432")]
	pub expire_date: Option<LocalMktDate>,
	/// Conditionally required if <a href="tag_59_TimeInForce_.html">TimeInForce&nbsp;(59)</a> = GTD and <a href="tag_432_ExpireDate_.html">ExpireDate&nbsp;(432)</a> is not specified.
	#[serde(rename = "126")]
	pub expire_time: Option<UTCTimestamp>,
	/// States whether executions are booked out or accumulated on a partially filled GT order.
	#[serde(rename = "427")]
	pub gt_booking_inst: Option<GTBookingInst>,
	/// Insert here the set of "CommissionData" fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub commission_data: Option<CommissionData>,
	/// OrderCapacity
	#[serde(rename = "528")]
	pub order_capacity: Option<OrderCapacity>,
	/// OrderRestrictions
	#[serde(rename = "529")]
	pub order_restrictions: Option<SeparatedValues<OrderRestrictions>>,
	/// PreTradeAnonymity
	#[serde(rename = "1091")]
	pub pre_trade_anonymity: Option<Boolean>,
	/// CustOrderCapacity
	#[serde(rename = "582")]
	pub cust_order_capacity: Option<CustOrderCapacity>,
	/// Indicates that broker is requested to execute a Forex accommodation trade in conjunction with the security trade.
	#[serde(rename = "121")]
	pub forex_req: Option<ForexReq>,
	/// Required if <a href="tag_121_ForexReq_.html">ForexReq&nbsp;(121)</a> = Y.
	#[serde(rename = "120")]
	pub settl_currency: Option<Currency>,
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
	/// (Deprecated in FIX.5.0)Can be used with <a href="tag_40_OrdType_.html">OrdType&nbsp;(40)</a> = "Forex - Swap" to specify the price for the future portion of a F/X swap which is also a limit order. For F/X orders, should
	/// be the "all-in" rate (spot rate adjusted for forward points).
	#[serde(rename = "640")]
	pub price_2: Option<f64>,
	/// For use in derivatives omnibus accounting
	#[serde(rename = "77")]
	pub position_effect: Option<PositionEffect>,
	/// For use with derivatives, such as options
	#[serde(rename = "203")]
	pub covered_or_uncovered: Option<CoveredOrUncovered>,
	/// (Deprecated in FIX.5.0)
	#[serde(rename = "210")]
	pub max_show: Option<f64>,
	/// Insert here the set of "PegInstruction" fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub peg_instructions: Option<PegInstructions>,
	/// Insert here the set of "DiscretionInstruction" fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub discretion_instructions: Option<DiscretionInstructions>,
	/// The target strategy of the order
	#[serde(rename = "847")]
	pub target_strategy: Option<TargetStrategy>,
	/// Indicates number of strategy parameters
	#[serde(rename = "957")]
	pub strategy_parameters: Option<RepeatingValues<StrategyParameter>>,
	/// (Deprecated in FIX.5.0)For further specification of the TargetStrategy
	#[serde(rename = "848")]
	pub target_strategy_parameters: Option<String>,
	/// (Deprecated in FIX.5.0)Mandatory for a TargetStrategy=Participate order and specifies the target particpation rate. For other
	/// order types optionally specifies a volume limit (i.e. do not be more than this percent of the market volume)
	#[serde(rename = "849")]
	pub participation_rate: Option<f32>,
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
	pub cust_order_handling_inst: Option<SeparatedValues<CustOrderHandlingInst>>,
	/// OrderHandlingInstSource
	#[serde(rename = "1032")]
	pub order_handling_inst_source: Option<OrderHandlingInstSource>,
	/// TrdRegTimestamps
	#[serde(flatten)]
	pub trd_reg_timestamps: Option<TrdRegTimestamps>,
	/// Required for counter-order selection / Hit / Take Orders. (OrdType = Q)
	#[serde(rename = "1080")]
	pub ref_order_id: Option<String>,
	/// Conditionally required if <a href="tag_1080_RefOrderID_.html">RefOrderID&nbsp;(1080)</a> is specified.
	#[serde(rename = "1081")]
	pub ref_order_id_source: Option<RefOrderIDSource>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub trailer: Trailer,
}

impl NewOrderSingle {
    pub fn new() -> Self {
        NewOrderSingle {
            header: Header {
                begin_string: Some(FixVersion::FIXT11),
                msg_type: Some(MsgType::NewOrderSingle),
                appl_ver_id: Some(ApplVerID::FIX50),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl HasHeader for NewOrderSingle {
    fn get_header(&self) -> &Header {
        &self.header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.header
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Alloc {
	/// Required if <a href="tag_78_NoAllocs_.html">NoAllocs&nbsp;(78)</a> &gt; 0. Must be first field in repeating group.
	#[serde(rename = "79")]
	pub alloc_account: Option<String>,
	/// AllocAcctIDSource
	#[serde(rename = "661")]
	pub alloc_acct_id_source: Option<AllocAcctIDSource>,
	/// AllocSettlCurrency
	#[serde(rename = "736")]
	pub alloc_settl_currency: Option<Currency>,
	/// IndividualAllocID
	#[serde(rename = "467")]
	pub individual_alloc_id: Option<String>,
	/// AllocQty
	#[serde(rename = "80")]
	pub alloc_qty: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TradingSession {
	/// Required if <a href="tag_386_NoTradingSessions_.html">NoTradingSessions&nbsp;(386)</a> is &gt; 0.
	#[serde(rename = "336")]
	pub trading_session_id: Option<String>,
	/// TradingSessionSubID
	#[serde(rename = "625")]
	pub trading_session_sub_id: Option<String>,
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
pub enum ProcessCode {
	/// Regular
	#[serde(rename = "0")]
	Regular,
	/// Soft Dollar
	#[serde(rename = "1")]
	SoftDollar,
	/// Step-In
	#[serde(rename = "2")]
	StepIn,
	/// Step-Out
	#[serde(rename = "3")]
	StepOut,
	/// Soft-dollar Step-In
	#[serde(rename = "4")]
	SoftDollarStepIn,
	/// Soft-dollar Step-Out
	#[serde(rename = "5")]
	SoftDollarStepOut,
	/// Plan Sponsor
	#[serde(rename = "6")]
	PlanSponsor,
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
pub enum LocateReqd {
	/// Indicates the broker is responsible for locating the stock
	#[serde(rename = "Y")]
	IndicatesTheBrokerIsResponsibleForLocatingTheStock,
	/// Indicates the broker is not required to locate
	#[serde(rename = "N")]
	IndicatesTheBrokerIsNotRequiredToLocate,
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

impl Default for OrdType {
    fn default() -> Self {
        OrdType::Market
    }
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
pub enum ForexReq {
	/// Execute Forex after security trade
	#[serde(rename = "Y")]
	ExecuteForexAfterSecurityTrade,
	/// Do not execute Forex after security trade
	#[serde(rename = "N")]
	DoNotExecuteForexAfterSecurityTrade,
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
pub enum CoveredOrUncovered {
	/// Covered
	#[serde(rename = "0")]
	Covered,
	/// Uncovered
	#[serde(rename = "1")]
	Uncovered,
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
	NoIntitutional,
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
	ExemptBelowTheLimit,
	/// Exempt - Client Money Type Exemption
	#[serde(rename = "2")]
	ExemptClientMoney,
	/// Exempt - Authorised Credit or Financial Institution
	#[serde(rename = "3")]
	ExemptAuthorizedCredit,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RefOrderIDSource {
	/// <a href="tag_198_SecondaryOrderID_.html">SecondaryOrdeID&nbsp;(198)</a>
	#[serde(rename = "0")]
	SecondaryOrdeID,
	/// <a href="tag_37_OrderID_.html">OrdeID&nbsp;(37)</a>
	#[serde(rename = "1")]
	OrdeID,
	/// <a href="tag_278_MDEntryID_.html">MEntryID&nbsp;(278)</a>
	#[serde(rename = "2")]
	MEntryID,
	/// <a href="tag_299_QuoteEntryID_.html">QuotEntryID&nbsp;(299)</a>
	#[serde(rename = "3")]
	QuotEntryID,
}
