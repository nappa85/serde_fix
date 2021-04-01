
use serde::{Serialize, Deserialize};

use crate::entities::{ApplVerID, Boolean, EncodedText, LocalMktDate, MultipleValue, UTCTimestamp, fix50sp2::{commission_data::CommissionData, commission_data_grp::CommissionDataGrp, disclosure_instruction_grp::DisclosureInstructionGrp, discretion_instructions::DiscretionInstructions, display_instruction::DisplayInstruction, financing_details::FinancingDetails, instrument::{Currency, Instrument}, matching_instructions::MatchingInstructions, order_attribute_grp::OrderAttributeGrp, order_qty_data::OrderQtyData, parties::Parties, peg_instructions::PegInstructions, pre_alloc_grp::PreAllocGrp, rate_source::RateSource, spread_or_benchmark_curve_data::SpreadOrBenchmarkCurveData, stipulations::Stipulations, strategy_parameters_grp::StrategyParametersGrp, target_parties::TargetParties, trd_reg_publication_grp::TrdRegPublicationGrp, trd_reg_timestamps::TrdRegTimestamps, trdg_ses_grp::TrdgSesGrp, triggering_instruction::TriggeringInstruction, und_instrmt_grp::UndInstrmtGrp, value_checks_grp::ValueChecksGrp, yield_data::YieldData}, fixt11::{Trailer, header::{Header, HasHeader, MsgType}}, version::FixVersion};

/// MsgType = D
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NewOrderSingle {
    #[serde(flatten)]
    pub header: Header,
    /// Unique identifier of the order as assigned by institution or by the intermediary (CIV term, not a hub/service bureau) with closest association with the investor. 
    #[serde(rename = "11")]
    pub cl_ord_id: String,
    /// SecondaryClOrdID
    #[serde(rename = "526")]
    pub secondary_cl_ord_id: Option<String>,
    /// ClOrdLinkID
    #[serde(rename = "583")]
    pub cl_ord_link_id: Option<String>,
    /// This is party information related to the submitter of the request
    #[serde(flatten)]
    pub parties: Parties,
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
    /// Number of repeating groups for pre-trade allocation.
    #[serde(flatten)]
    pub pre_alloc_grp: PreAllocGrp,
    /// SettlType
    #[serde(rename = "63")]
    pub settl_type: Option<SettlType>,
    /// Takes precedence over SettlType (63) value and conditionally required/omitted for specific SettlType (63) values.
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
    /// Can contain multiple instructions, space delimited. If OrdType (40) =P, exactly one of the following values ( ExecInst (18) = L, R, M, P, O, T, W, a, d) must be specified.
    #[serde(rename = "18")]
    pub exec_inst: Option<MultipleValue<ExecInst>>,
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
    pub display_instruction: DisplayInstruction,
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
    #[serde(flatten)]
    pub trdg_ses_grp: TrdgSesGrp,	
    /// Used to identify soft trades at order entry.
    #[serde(rename = "81")]
    pub process_code: Option<ProcessCode>,
    /// Insert here the set of "Instrument" fields defined in "Common Components of Application Messages".
    #[serde(flatten)]
    pub instrument: Instrument,
    /// Insert here the set of "FinancingDetails" fields defined in "Common Components of Application Messages".
    #[serde(flatten)]
    pub financing_details: FinancingDetails,
    /// Number of underlyings
    #[serde(flatten)]
    pub und_instrmt_grp: UndInstrmtGrp,
    /// Useful for verifying security identification
    #[serde(rename = "140")]
    pub prev_close_px: Option<f64>,
    /// Side
    #[serde(rename = "54")]
    pub side: Side,
    /// Required for short sell orders
    #[serde(rename = "114")]
    pub locate_reqd: Option<Boolean>,
    /// Time this order request was initiated/released by the trader, trading system, or intermediary.
    #[serde(rename = "60")]
    pub transact_time: UTCTimestamp,
    /// Insert here the set of "Stipulations" (repeating group of Fixed Income stipulations) fields defined in "Common Components of Application Messages".
    #[serde(flatten)]
    pub stipulations: Stipulations,
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
    /// Required for limit OrdTypes. For F/X orders, should be the "all-in" rate (spot rate adjusted for forward points). Can be used to specify a limit price for a pegged order, previously indicated, etc.
    #[serde(rename = "44")]
    pub price: Option<f64>,
    /// PriceProtectionScope
    #[serde(rename = "1092")]
    pub price_protection_scope: Option<PriceProtectionScope>,
    /// Required for OrdType (40) = "Stop" or OrdType (40) = "Stop limit".
    #[serde(rename = "99")]
    pub stop_px: Option<f64>,
    /// Insert here the set of "TriggeringInstruction" fields defined in "common components of application messages".
    #[serde(flatten)]
    pub triggering_instruction: TriggeringInstruction,
    /// Insert here the set of "SpreadOrBenchmarkCurveData" (Fixed Income spread or benchmark curve) fields defined in "Common Components of Application Messages".
    #[serde(flatten)]
    pub spread_or_benchmark_curve_data: SpreadOrBenchmarkCurveData,
    /// Insert here the set of "YieldData" (yield-related) fields defined in "Common Components of Application Messages".
    #[serde(flatten)]
    pub yield_data: YieldData,
    /// Currency
    #[serde(rename = "15")]
    pub currency: Option<Currency>,
    /// ComplianceID
    #[serde(rename = "376")]
    pub compliance_id: Option<String>,
    /// SolicitedFlag
    #[serde(rename = "377")]
    pub solicited_flag: Option<Boolean>,
    /// Required for Previously Indicated Orders (OrdType=E)
    #[serde(rename = "23")]
    pub ioi_id: Option<String>,
    /// Required for Previously Quoted Orders (OrdType=D)
    #[serde(rename = "117")]
    pub quote_id: Option<String>,
    /// Absence of this field indicates Day order
    #[serde(rename = "59")]
    pub time_in_force: Option<TimeInForce>,
    /// Can specify the time at which the order should be considered valid
    #[serde(rename = "168")]
    pub effective_time: Option<UTCTimestamp>,
    /// Conditionally required if TimeInForce (59) = GTD and ExpireTime (126) is not specified.
    #[serde(rename = "432")]
    pub expire_date: Option<LocalMktDate>,
    /// Conditionally required if TimeInForce (59) = GTD and ExpireDate (432) is not specified.
    #[serde(rename = "126")]
    pub expire_time: Option<UTCTimestamp>,
    /// States whether executions are booked out or accumulated on a partially filled GT order.
    #[serde(rename = "427")]
    pub gt_booking_inst: Option<GTBookingInst>,
    /// CommissionData
    #[serde(flatten)]
    pub commission_data: CommissionData,
    /// OrderCapacity
    #[serde(rename = "528")]
    pub order_capacity: Option<OrderCapacity>,
    /// OrderRestrictions
    #[serde(rename = "529")]
    pub order_restrictions: Option<MultipleValue<OrderRestrictions>>,
    /// PreTradeAnonymity
    #[serde(rename = "1091")]
    pub pre_trade_anonymity: Option<Boolean>,
    /// CustOrderCapacity
    #[serde(rename = "582")]
    pub cust_order_capacity: Option<CustOrderCapacity>,
    /// Indicates that broker is requested to execute a Forex accommodation trade in conjunction with the security trade.
    #[serde(rename = "121")]
    pub forex_req: Option<Boolean>,
    /// Required if ForexReq (121) = Y.
    #[serde(rename = "120")]
    pub settl_currency: Option<Currency>,
    /// RateSource
    #[serde(flatten)]
    pub rate_source: RateSource,
    /// OffshoreIndicator
    #[serde(rename = "2795")]
    pub offshore_indicator: Option<OffshoreIndicator>,
    /// Method for booking out this order. Used when notifying a broker that an order to be settled by that broker is to be booked out as an OTC derivative (e.g. CFD or similar). Absence of this field implies regular booking.
    #[serde(rename = "775")]
    pub booking_type: Option<BookingType>,
    /// Text
    #[serde(rename = "58")]
    pub text: Option<String>,
    /// Must be set if EncodedText (355) field is specified and must immediately precede it.
    #[serde(rename = "354")]
    /// Encoded (non-ASCII characters) representation of the Text (58) field in the encoded format specified via the MessageEncoding (347) field.
    #[serde(alias = "355")]
    pub encoded_text: Option<EncodedText<355>>,
    /// (Deprecated in FIX.5.0)Can be used with OrdType (40) = "Forex - Swap" to specify the "value date" for the future portion of a F/X swap.
    #[serde(rename = "193")]
    pub settl_date2: Option<LocalMktDate>,
    /// (Deprecated in FIX.5.0)Can be used with OrdType (40) = "Forex - Swap" to specify the order quantity for the future portion of a F/X swap.
    #[serde(rename = "192")]
    pub order_qty2: Option<f64>,
    /// (Deprecated in FIX.5.0)Can be used with OrdType (40) = "Forex - Swap" to specify the price for the future portion of a F/X swap which is also a limit order. For F/X orders, should be the "all-in" rate (spot rate adjusted for forward points).
    #[serde(rename = "640")]
    pub price2: Option<f64>,
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
    pub peg_instructions: PegInstructions,
    /// Insert here the set of "DiscretionInstruction" fields defined in "Common Components of Application Messages".
    #[serde(flatten)]
    pub discretion_instructions: DiscretionInstructions,
    /// The target strategy of the order
    #[serde(rename = "847")]
    pub target_strategy: Option<TargetStrategy>,
    /// Strategy parameter block
    pub strategy_parameters_grp: StrategyParametersGrp,
    /// (Deprecated in FIX.5.0)For further specification of the TargetStrategy
    #[serde(rename = "848")]
    pub target_strategy_parameters: Option<String>,
    /// (Deprecated in FIX.5.0)Mandatory for a TargetStrategy=Participate order and specifies the target particpation rate. For other order types optionally specifies a volume limit (i.e. do not be more than this percent of the market volume)
    #[serde(rename = "849")]
    pub participation_rate: Option<f32>,
    /// For CIV - Optional
    #[serde(rename = "480")]
    pub cancellation_rights: Option<CancellationRights>,
    /// MoneyLaunderingStatus
    #[serde(rename = "481")]
    pub money_laundering_status: Option<MoneyLaunderingStatus>,
    /// Reference to Registration Instructions (o) message for this Order.
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
    pub cust_order_handling_inst: Option<MultipleValue<CustOrderHandlingInst>>,
    /// OrderHandlingInstSource
    #[serde(rename = "1032")]
    pub order_handling_inst_source: Option<OrderHandlingInstSource>,
    /// TrdRegTimestamps
    #[serde(flatten)]
    pub trd_reg_timestamps: TrdRegTimestamps,
    /// Required for counter-order selection / Hit / Take Orders. (OrdType = Q)
    #[serde(rename = "1080")]
    pub ref_order_id: Option<String>,
    /// Conditionally required if RefOrderID (1080) is specified.
    #[serde(rename = "1081")]
    pub ref_order_id_source: Option<RefOrderIDSource>,
    /// MatchingInstructions
    #[serde(flatten)]
    pub matching_instructions: MatchingInstructions,
    /// ExposureDuration
    #[serde(rename = "1629")]
    pub exposure_duration: Option<i32>,
    /// ThrottleInst
    #[serde(rename = "1685")]
    pub throttle_inst: Option<ThrottleInst>,
    /// TradePriceNegotiationMethod
    #[serde(rename = "1740")]
    pub trade_price_negotiation_method: Option<TradePriceNegotiationMethod>,
    /// UpfrontPriceType
    #[serde(rename = "1741")]
    pub upfront_price_type: Option<UpfrontPriceType>,
    /// Upfront Price for CDS transactions. Conditionally required if TradePriceNegotiationMethod(1740) = 4(Percent of Par and Upfront Amount), 5(Deal Spread and Upfront Amount) or 6(Upfront Points and Upfront Amount)
    #[serde(rename = "1742")]
    pub upfront_price: Option<f64>,
    /// Available for optional use when Side(54) = 6(Sell short exempt)
    #[serde(rename = "1688")]
    pub short_sale_exemption_reason: Option<ShortSaleExemptionReason>,
    /// Identifies parties not directly associated with or owning the order, who are to be informed to effect processing of the order
    #[serde(flatten)]
    pub target_parties: TargetParties,
    /// AuctionInstruction
    #[serde(rename = "1805")]
    pub auction_instruction: Option<AuctionInstruction>,
    /// MinQtyMethod
    #[serde(rename = "1822")]
    pub min_qty_method: Option<MinQtyMethod>,
    /// Specifies instructions to disclose certain order level information in market data
    #[serde(flatten)]
    pub disclosure_instruction_grp: DisclosureInstructionGrp,
    /// MarketSegmentID
    #[serde(rename = "1300")]
    pub market_segment_id: Option<String>,
    /// TradingCapacity
    #[serde(rename = "1815")]
    pub trading_capacity: Option<TradingCapacity>,
    /// Applies to trades resulting from the order
    #[serde(rename = "1390")]
    pub trade_publish_indicator: Option<TradePublishIndicator>,
    /// ClearingAccountType
    #[serde(rename = "1816")]
    pub clearing_account_type: Option<ClearingAccountType>,
    /// RefClOrdID
    #[serde(rename = "1806")]
    pub ref_cl_ord_id: Option<String>,
    /// Conditionally required for auction orders
    #[serde(rename = "1803")]
    pub auction_type: Option<AuctionType>,
    /// AuctionAllocationPct
    #[serde(rename = "1804")]
    pub auction_allocation_pct: Option<f32>,
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
    /// ExposureDurationUnit
    #[serde(rename = "1916")]
    pub exposure_duration_unit: Option<ExposureDurationUnit>,
    /// ShortMarkingExemptIndicator
    #[serde(rename = "2102")]
    pub short_marking_exempt_indicator: Option<Boolean>,
    /// ComplianceText
    #[serde(rename = "2404")]
    pub compliance_text: Option<String>,
    /// Must be set if EncodedComplianceText(2352) field is specified and must immediately precede it.
    #[serde(rename = "2351")]
    /// Encoded (non-ASCII characters) representation of the ComplianceText(2404) field in the encoded format specified via the MessageEncoding(347) field.
    #[serde(alias = "2352")]
    pub encoded_compliance_text: Option<EncodedText<2352>>,
    /// OrderRequestID
    #[serde(rename = "2422")]
    pub order_request_id: Option<i32>,
    /// Use as an alternative to CommissionData component if multiple commissions or enhanced attributes are needed.
    #[serde(flatten)]
    pub commission_data_grp: CommissionDataGrp,
    /// May be used as an alternative to MatchingInstructions when the identifier does not appear in another field.
    #[serde(rename = "2362")]
    pub self_match_prevention_id: Option<String>,
    /// OrderAttributeGrp
    #[serde(flatten)]
    pub order_attribute_grp: OrderAttributeGrp,
    /// MaximumPriceDeviation
    #[serde(rename = "2676")]
    pub maximum_price_deviation: Option<f32>,
    /// ValueChecksGrp
    #[serde(flatten)]
    pub value_checks_grp: ValueChecksGrp,
    /// ExDestinationType
    #[serde(rename = "2704")]
    pub ex_destination_type: Option<ExDestinationType>,
    /// DuplicateClOrdIDIndicator
    #[serde(rename = "2829")]
    pub duplicate_cl_ord_id_indicator: Option<Boolean>,
    /// May be used for new (child) orders stemming from the split of a parent order. Refers to the working price of the parent order.
    #[serde(rename = "2838")]
    pub current_working_price: Option<f64>,
    /// May be used when intentionally sending an order more than once, e.g. an order being received manually as well as electronically in conjunction with a regulatory requirement to report both events.
    #[serde(rename = "797")]
    pub copy_msg_indicator: Option<Boolean>,
    /// RoutingArrangmentIndicator
    #[serde(rename = "2883")]
    pub routing_arrangment_indicator: Option<RoutingArrangmentIndicator>,
    /// May be used for cross orders submitted with single order messages.
    #[serde(rename = "2884")]
    pub contra_routing_arrangment_indicator: Option<i32>,
    /// May be used for cross orders submitted with single order messages.
    #[serde(rename = "2882")]
    pub contra_order_origination: Option<i32>,
    /// RegulatoryReportType
    #[serde(rename = "1934")]
    pub regulatory_report_type: Option<RegulatoryReportType>,
    /// AffiliatedFirmsTradeIndicator
    #[serde(rename = "2525")]
    pub affiliated_firms_trade_indicator: Option<Boolean>,
    /// TrdRegPublicationGrp
    #[serde(flatten)]
    pub trd_reg_publication_grp: TrdRegPublicationGrp,
    /// TradeReportingIndicator
    #[serde(rename = "2524")]
    pub trade_reporting_indicator: Option<TradeReportingIndicator>,
    #[serde(flatten)]
    pub trailer: Trailer,
}

impl NewOrderSingle {
    pub fn new() -> Self {
        NewOrderSingle {
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

impl HasHeader for NewOrderSingle {
    fn get_header(&self) -> &Header {
        &self.header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.header
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AcctIDSource {
    /// BIC
    #[serde(rename = "1")]
    BIC,
    /// SID code
    #[serde(rename = "2")]
    SIDCode,
    /// TFM (GSPTA)
    #[serde(rename = "3")]
    TFM,
    /// OMGEO (AlertID)
    #[serde(rename = "4")]
    OMGEO,
    /// DTCC code
    #[serde(rename = "5")]
    DTCCCode,
    /// Special Segregated Account ID
    #[serde(rename = "6")]
    SpecialSegregatedAccountID,
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
    Auto,
    /// Speak with order initiator before booking ("speak first")
    #[serde(rename = "1")]
    SpeakFirst,
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
    AggregatePartialExecutionsOnThisOrderAndBookOneTradePerOrder,
    /// Aggregate executions for this symbol, side, and settlement date
    #[serde(rename = "2")]
    AggregateExecutionsForThisSymbolSideAndSettlementDate,
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
    /// Broken date - for FX expressing non-standard tenor, SettlDate (64) must be specified
    #[serde(rename = "B")]
    BrokenDate,
    /// FX Spot Next settlement (Spot+1, aka next day)
    #[serde(rename = "C")]
    FXSpotNextSettlement,
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
    First,
    /// 2nd year delegate trading for own account
    #[serde(rename = "2")]
    Second,
    /// 3rd year delegate trading for own account
    #[serde(rename = "3")]
    Third,
    /// 4th year delegate trading for own account
    #[serde(rename = "4")]
    Fourth,
    /// 5th year delegate trading for own account
    #[serde(rename = "5")]
    Fifth,
    /// 6th year delegate trading for own account
    #[serde(rename = "9")]
    Sixth,
    /// CBOE Member
    #[serde(rename = "B")]
    CBOEMember,
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
    Firms106HJ,
    /// GIM, IDEM and COM Membership Interest Holders
    #[serde(rename = "I")]
    GIMIDEMAndCOMMembershipInterestHolders,
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
    AutomatedPrivate,
    /// Automated execution order, public, Broker intervention OK
    #[serde(rename = "2")]
    AutomatedPublic,
    /// Manual order, best execution
    #[serde(rename = "3")]
    Manual,
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
    ParticipateDontInitiate,
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
    OKToCross,
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
    PegToVWAP,
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
    ExecuteAsFXNeutral,
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
pub enum ExDestinationIDSource {
    /// BIC (Bank Identification Code) (ISO 9362)
    #[serde(rename = "B")]
    BIC,
    /// Generally accepted market participant identifier (e.g. NASD mnemonic)
    #[serde(rename = "C")]
    GenerallyAcceptedMarketParticipantIdentifier,
    /// Proprietary / Custom code
    #[serde(rename = "D")]
    ProprietaryCustomCode,
    /// ISO Country Code
    #[serde(rename = "E")]
    ISOCountryCode,
    /// MIC (ISO 10383 - Market Identifier Code)
    #[serde(rename = "G")]
    MIC,
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
    /// Contracts (if used - must specify ContractMultiplier (231) )
    #[serde(rename = "1")]
    Contracts,
    /// Units of Measure per Time Unit (if used - must specify UnitofMeasure (996) and TimeUnit (997) ) 
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
    StopLoss,
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
    /// Stop on Bid or Offer (A stop order that is triggered by a bid or offer price movement (quote) at which point the stopped order becomes a market order, also known as "stop on quote" in some markets (e.g. US markets). In the US equities market it is common to trigger a stop off the National Best Bid or Offer (NBBO))
    #[serde(rename = "R")]
    StopOnBidOrOffer,
    /// Stop Limit on Bid or Offer (A stop order that is triggered by a bid or offer price movement (quote) at which ponit the stopped order becomes a limit order, also known as "stop limit on quote" in some markets (e.g. US markets). In the US equities market it is common to trigger a stop off the National Best Bid or Offer (NBBO) 
    #[serde(rename = "S")]
    StopLimitOnBidOrOffer,
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
    TEDPrice,
    /// TED yield
    #[serde(rename = "8")]
    TEDYield,
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
    /// Normal rate representation (e.g. FX rate) (represents the number of units of currency 2 that is required to purchase one unit of currency 1.)
    #[serde(rename = "20")]
    NormalRateRepresentation,
    /// Inverse rate representation (e.g. FX rate) (represents the number of units of 1 that is needed to purchase one unit of currency 2.)
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
pub enum GTBookingInst {
    /// Book out all trades on day of execution
    #[serde(rename = "0")]
    BookOut,
    /// Accumulate executions until order is filled or expires
    #[serde(rename = "1")]
    Accumulate,
    /// Accumulate until verbally notified otherwise
    #[serde(rename = "2")]
    AccumulateUntil,
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
pub enum BookingType {
    /// Regular booking
    #[serde(rename = "0")]
    RegularBooking,
    /// CFD (Contract for difference)
    #[serde(rename = "1")]
    ContractForDifference,
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
    FIFO,
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
    VWAP,
    /// Participate (i.e. aim to be x percent of the market volume)
    #[serde(rename = "2")]
    Participate,
    /// Mininize market impact
    #[serde(rename = "3")]
    MininizeMarketImpact,
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
    ExemptBelowTheLimit,
    /// Exempt - Client Money Type Exemption
    #[serde(rename = "2")]
    ExemptClientMoneyTypeExemption,
    /// Exempt - Authorised Credit or Financial Institution
    #[serde(rename = "3")]
    ExemptAuthorisedCreditOrFinancialInstitution,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CustOrderHandlingInst {
    /// Add-on Order
    #[serde(rename = "ADD")]
    AddonOrder,
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
    FCMProvidedScreen,
    /// Other provided screen
    #[serde(rename = "D")]
    OtherProvidedScreen,
    /// Client provided platform controlled by FCM
    #[serde(rename = "E")]
    ClientProvidedPlatformControlledByFCM,
    /// Client provided platform direct to exchange
    #[serde(rename = "F")]
    ClientProvidedPlatformDirectToExchange,
    /// FCM API or FIX
    #[serde(rename = "G")]
    FCMAPIOrFIX,
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
    OKToCross,
    /// Call first
    #[serde(rename = "FC")]
    Callfirst,
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
    PegToVWAP,
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
    NASDOATS,
    /// FIA Execution Source Code
    #[serde(rename = "2")]
    FIAExecutionSourceCode,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RefOrderIDSource {
    /// Secondary order ID
    #[serde(rename = "0")]
    SecondaryOrderID,
    /// Order ID
    #[serde(rename = "1")]
    OrderID,
    /// Market data entry ID
    #[serde(rename = "2")]
    MarketDataEntryID,
    /// Quote entry ID
    #[serde(rename = "3")]
    QuoteEntryID,
    /// Original order ID
    #[serde(rename = "4")]
    OriginalOrderID,
    /// Quote ID
    #[serde(rename = "5")]
    QuoteID,
    /// Quote request ID
    #[serde(rename = "6")]
    QuoteRequestID,
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
pub enum ThrottleInst {
    /// Reject if throttle limit exceeded
    #[serde(rename = "0")]
    RejectIfThrottleLimitExceeded,
    /// Queue if throttle limit exceeded
    #[serde(rename = "1")]
    QueueIfThrottleLimitExceeded,
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
    VWAP,
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
pub enum MinQtyMethod {
    /// Once (applies only to first execution)
    #[serde(rename = "1")]
    Once,
    /// Multiple (applies to every execution)
    #[serde(rename = "2")]
    Multiple,
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
pub enum RoutingArrangmentIndicator {
    /// No routing arrangement in place
    #[serde(rename = "0")]
    NoRoutingArrangementInPlace,
    /// Routing arrangement in place
    #[serde(rename = "1")]
    RoutingArrangementInPlace,
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
    CombinationOfRTAndPET,
    /// Combination of PET and confirmation
    #[serde(rename = "5")]
    CombinationOfPETAndConfirmation,
    /// Combination of RT, PET and confirmation
    #[serde(rename = "6")]
    CombinationOfRTPETAndConfirmation,
    /// Post-trade valuation
    #[serde(rename = "7")]
    PostTradeValuation,
    /// Verification
    #[serde(rename = "8")]
    Verification,
    /// Post-trade event
    #[serde(rename = "9")]
    PostTradeEvent,
    /// Post trade event RT reportable (Report of a regulated transaction continuation event that falls within the requirements for real-time reporting and public dissemination. If dissemination is to be suppressed due to an end user exception or to local regulatory rules that allow suppression of certain types of transactions, use TradePublishIndicator(1390) = 0 (Do not publish trade)
    #[serde(rename = "10")]
    PostTradeEventRTReportable,
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
    FullDetailsTradeOfLimitedDetailsTrade,
    /// Full Details of "Daily Aggregated Trade"
    #[serde(rename = "18")]
    FullDetailsOfDailyAggregatedTrade,
    /// Full Details of "Volume Omission Trade"
    #[serde(rename = "19")]
    FullDetailsOfVolumeOmissionTrade,
    /// Full Details of "Four Weeks Aggregation Trade"
    #[serde(rename = "20")]
    FullDetailsOfFourWeeksAggregationTrade,
    /// Full Details in Aggregated Form of "Volume Omission Trade Eligible for Subsequent Aggregated Enrichment"
    #[serde(rename = "21")]
    FullDetailsInAggregatedFormOfVolumeOmissionTradeEligibleForSubsequentAggregatedEnrichment,
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
    TerminationEarlyTermination,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TradeReportingIndicator {
    /// Trade has not (yet) been reported
    #[serde(rename = "0")]
    TradeHasNotBeenReported,
    /// Trade has been or will be reported by a trading venue as an "on-book" trade
    #[serde(rename = "1")]
    TradeHasBeenOrWillBeReportedByATradingVenueAsAnOnBookTrade,
    /// Trade has been or will be reported as a "systematic internaliser" seller trade
    #[serde(rename = "2")]
    TradeHasBeenOrWillBeReportedAsASystematicInternaliserSellerTrade,
    /// Trade has been or will be reported as a "systematic internaliser" buyer trade
    #[serde(rename = "3")]
    TradeHasBeenOrWillBeReportedAsASystematicInternaliserBuyerTrade,
    /// Trade has been or will be reported as a "non-systematic internaliser" seller trade
    #[serde(rename = "4")]
    TradeHasBeenOrWillBeReportedAsANonSystematicInternaliserSellerTrade,
    /// Trade has been or will be reported under a sub-delegation arrangement by an investment firm to a reporting facility (e.g. APA) on behalf of another investment firm
    #[serde(rename = "5")]
    TradeHasBeenOrWillBeReportedUnderASubDelegationArrangementByAnInvestmentFirmToAReportingFacilityOnBehalfOfAnotherInvestmentFirm,
    /// Trade has been or will be reported
    #[serde(rename = "6")]
    TradeHasBeenOrWillBeReported,
    /// Trade has been or will be reported as a "non-Systematic Internaliser" buyer trade
    #[serde(rename = "7")]
    TradeHasBeenOrWillBeReportedAsANonSystematicInternaliserBuyerTrade,
    /// Trade has been or will be reported by a trading venue as an "off-book" trade
    #[serde(rename = "8")]
    TradeHasBeenOrWillBeReportedByATradingVenueAsAnOffBookTrade,
    /// Trade is not reportable
    #[serde(rename = "9")]
    TradeIsNotReportable,
}
