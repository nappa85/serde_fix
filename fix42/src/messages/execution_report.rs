
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExecutionReport {
	/// MsgType = 8
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'8'>,
	/// <a href="tag_37_OrderID.html" target="bottom">OrderID&nbsp;(37)</a> is required to be unique for each chain of orders.
	#[serde(rename = "37")]
	pub order_id: String,
	/// Can be used to provide order id used by exchange or executing system.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "198")]
	pub secondary_order_id: Option<String>,
	/// Required for executions against electronically submitted orders which were assigned an ID by the institution. Not required
	/// for orders manually entered by the broker.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "11")]
	pub cl_ord_id: Option<String>,
	/// Conditionally required for response to an electronic Cancel or Cancel/Replace request ( <a href="tag_150_ExecType.html" target="bottom">ExecType&nbsp;(150)</a> =PendingCancel, Replaced, or Canceled). <a href="tag_11_ClOrdID.html" target="bottom">ClOrdID&nbsp;(11)</a> of the previous order (NOT the initial order of the day) when canceling or replacing an order.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41")]
	pub orig_cl_ord_id: Option<String>,
	/// Used for firm identification in third-party transactions (should not be a substitute for <a href="tag_115_OnBehalfOfCompID.html" target="bottom">OnBehalfOfCompID&nbsp;(115)</a> / <a href="tag_128_DeliverToCompID.html" target="bottom">DeliverToCompID&nbsp;(128)</a> .
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "109")]
	pub client_id: Option<String>,
	/// Used for firm identification in third-party transactions (should not be a substitute for <a href="tag_115_OnBehalfOfCompID.html" target="bottom">OnBehalfOfCompID&nbsp;(115)</a> / <a href="tag_128_DeliverToCompID.html" target="bottom">DeliverToCompID&nbsp;(128)</a> .
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "76")]
	pub exec_broker: Option<String>,
	/// Number of ContraBrokers repeating group instances.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "382")]
	pub contra_brokers: Option<fix_common::RepeatingValues<ContraBroker>>,
	/// Required for executions against orders which were submitted as part of a list.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "66")]
	pub list_id: Option<String>,
	/// Must be unique for each <a href="message_Execution_Report_8.html" target="main">Execution Report&nbsp;(8)</a> message
	#[serde(rename = "17")]
	pub exec_id: String,
	/// ExecTransType
	#[serde(rename = "20")]
	pub exec_trans_type: ExecTransType,
	/// Required for Cancel and Correct <a href="tag_20_ExecTransType.html" target="bottom">ExecTransType&nbsp;(20)</a> messages
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "19")]
	pub exec_ref_id: Option<String>,
	/// Describes the type of execution report. Same possible values as <a href="tag_39_OrdStatus.html" target="bottom">OrdStatus&nbsp;(39)</a> .
	#[serde(rename = "150")]
	pub exec_type: ExecType,
	/// Describes the current state of a CHAIN of orders, same scope as <a href="tag_38_OrderQty.html" target="bottom">OrderQty&nbsp;(38)</a> , <a href="tag_14_CumQty.html" target="bottom">CumQty&nbsp;(14)</a> , <a href="tag_151_LeavesQty.html" target="bottom">LeavesQty&nbsp;(151)</a> , and <a href="tag_6_AvgPx.html" target="bottom">AvgPx&nbsp;(6)</a>
	#[serde(rename = "39")]
	pub ord_status: OrdStatus,
	/// For optional use with <a href="tag_150_ExecType.html" target="bottom">ExecType&nbsp;(150)</a> = 8 (Rejected)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "103")]
	pub ord_rej_reason: Option<OrdRejReason>,
	/// Required for <a href="tag_150_ExecType.html" target="bottom">ExecType&nbsp;(150)</a> = D (Restated).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "378")]
	pub exec_restatement_reason: Option<ExecRestatementReason>,
	/// Required for executions against electronically submitted orders which were assigned an account by the institution
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1")]
	pub account: Option<String>,
	/// Absence of this field is interpreted as Regular.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "63")]
	pub settlmnt_typ: Option<SettlmntTyp>,
	/// Required when <a href="tag_63_SettlmntTyp.html" target="bottom">SettlmntTyp&nbsp;(63)</a> = 6 (Future) or <a href="tag_63_SettlmntTyp.html" target="bottom">SettlmntTyp&nbsp;(63)</a> = 8 (Sellers Option)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "64")]
	pub fut_sett_date: Option<fix_common::LocalMktDate>,
	/// Symbol
	#[serde(rename = "55")]
	pub symbol: String,
	/// SymbolSfx
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "65")]
	pub symbol_sfx: Option<String>,
	/// SecurityID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "48")]
	pub security_id: Option<String>,
	/// IDSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "22")]
	pub id_source: Option<IDSource>,
	/// Must be specified if a Future or Option. If a Future: <a href="tag_55_Symbol.html" target="bottom">Symbol&nbsp;(55)</a> , <a href="tag_167_SecurityType.html" target="bottom">SecurityType&nbsp;(167)</a> , and <a href="tag_200_MaturityMonthYear.html" target="bottom">MaturityMonthYear&nbsp;(200)</a> are required. If an Option: <a href="tag_55_Symbol.html" target="bottom">Symbol&nbsp;(55)</a> , <a href="tag_167_SecurityType.html" target="bottom">SecurityType&nbsp;(167)</a> , <a href="tag_200_MaturityMonthYear.html" target="bottom">MaturityMonthYear&nbsp;(200)</a> , <a href="tag_201_PutOrCall.html" target="bottom">PutOrCall&nbsp;(201)</a> , and <a href="tag_202_StrikePrice.html" target="bottom">StrikePrice&nbsp;(202)</a> are required.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "167")]
	pub security_type: Option<SecurityType>,
	/// Specifies the month and year of maturity. Required if <a href="tag_205_MaturityDay.html" target="bottom">MaturityDay&nbsp;(205)</a> is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "200")]
	pub maturity_month_year: Option<fix_common::MonthYear>,
	/// Can be used in conjunction with <a href="tag_200_MaturityMonthYear.html" target="bottom">MaturityMonthYear&nbsp;(200)</a> to specify a particular maturity date.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "205")]
	pub maturity_day: Option<u8>,
	/// For Options.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "201")]
	pub put_or_call: Option<PutOrCall>,
	/// For Options.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "202")]
	pub strike_price: Option<f64>,
	/// For Options.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "206")]
	pub opt_attribute: Option<OptAttribute>,
	/// For Fixed Income, Convertible Bonds, Derivatives, etc. Note: If used, quantities should be expressed in the "nominal" (e.g.
	/// contracts vs. shares) amount.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "231")]
	pub contract_multiplier: Option<f64>,
	/// For Fixed Income.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "223")]
	pub coupon_rate: Option<f64>,
	/// Can be used to identify the security.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "207")]
	pub security_exchange: Option<String>,
	/// Issuer
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "106")]
	pub issuer: Option<String>,
	/// Must be set if <a href="tag_349_EncodedIssuer.html" target="bottom">EncodedIssuer&nbsp;(349)</a> field is specified and must immediately precede it.
	#[serde(rename = "348")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_106_Issuer.html" target="bottom">Issuer&nbsp;(106)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "349")]
	pub encoded_issuer: Option<fix_common::EncodedText<349>>,
	/// SecurityDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "107")]
	pub security_desc: Option<String>,
	/// Must be set if <a href="tag_351_EncodedSecurityDesc.html" target="bottom">EncodedSecurityDesc&nbsp;(351)</a> field is specified and must immediately precede it.
	#[serde(rename = "350")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_107_SecurityDesc.html" target="bottom">SecurityDesc&nbsp;(107)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "351")]
	pub encoded_security_desc: Option<fix_common::EncodedText<351>>,
	/// Side
	#[serde(rename = "54")]
	pub side: Side,
	/// Either <a href="tag_152_CashOrderQty.html" target="bottom">CashOrderQty&nbsp;(152)</a> or <a href="tag_38_OrderQty.html" target="bottom">OrderQty&nbsp;(38)</a> is required. Not required for a rejected cash order or an order ack for a cash order.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "38")]
	pub order_qty: Option<f64>,
	/// Either <a href="tag_152_CashOrderQty.html" target="bottom">CashOrderQty&nbsp;(152)</a> or <a href="tag_38_OrderQty.html" target="bottom">OrderQty&nbsp;(38)</a> is required. Specifies the approximate "monetary quantity" conveyed on the order. Broker is responsible for converting and
	/// calculating <a href="tag_38_OrderQty.html" target="bottom">OrderQty&nbsp;(38)</a> in shares for subsequent messages involving fills.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "152")]
	pub cash_order_qty: Option<f64>,
	/// OrdType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40")]
	pub ord_type: Option<OrdType>,
	/// Required if specified on the order
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "44")]
	pub price: Option<f64>,
	/// Required if specified on the order
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "99")]
	pub stop_px: Option<f64>,
	/// Required if specified on the order
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "211")]
	pub peg_difference: Option<f64>,
	/// Code to identify the price a <a href="tag_389_DiscretionOffset.html" target="bottom">DiscretionOffset&nbsp;(389)</a> is related to and should be mathematically added to. Required if <a href="tag_389_DiscretionOffset.html" target="bottom">DiscretionOffset&nbsp;(389)</a> is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "388")]
	pub discretion_inst: Option<DiscretionInst>,
	/// Amount (signed) added to the "related to" price specified via <a href="tag_388_DiscretionInst.html" target="bottom">DiscretionInst&nbsp;(388)</a> .
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "389")]
	pub discretion_offset: Option<f64>,
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
	/// Can contain multiple instructions, space delimited
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "18")]
	pub exec_inst: Option<fix_common::SeparatedValues<ExecInst>>,
	/// Rule80A
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "47")]
	pub rule_80_a: Option<Rule80A>,
	/// Quantity of shares bought/sold on this (last) fill. Not required for <a href="tag_20_ExecTransType.html" target="bottom">ExecTransType&nbsp;(20)</a> = 3 (Status). When required, should be "0" for non-fills ("fill" defined as <a href="tag_20_ExecTransType.html" target="bottom">ExecTransType&nbsp;(20)</a> = New and <a href="tag_150_ExecType.html" target="bottom">ExecType&nbsp;(150)</a> = Partial Fill or Fill) unless noted below. If <a href="tag_150_ExecType.html" target="bottom">ExecType&nbsp;(150)</a> =Stopped, represents the quantity stopped/guaranteed/protected for.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "32")]
	pub last_shares: Option<f64>,
	/// Price of this (last) fill. Not required for <a href="tag_20_ExecTransType.html" target="bottom">ExecTransType&nbsp;(20)</a> = 3 (Status), Should represent the "all-in" ( <a href="tag_194_LastSpotRate.html" target="bottom">LastSpotRate&nbsp;(194)</a> + <a href="tag_195_LastForwardPoints.html" target="bottom">LastForwardPoints&nbsp;(195)</a> ) rate for F/X orders. ). When required, should be "0" for non-fills ("fill" defined as <a href="tag_20_ExecTransType.html" target="bottom">ExecTransType&nbsp;(20)</a> = New and <a href="tag_150_ExecType.html" target="bottom">ExecType&nbsp;(150)</a> = Partial Fill or Fill) unless noted below. If <a href="tag_150_ExecType.html" target="bottom">ExecType&nbsp;(150)</a> = Stopped, represents the price stopped/guaranteed/protected at.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "31")]
	pub last_px: Option<f64>,
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
	/// LastMkt
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "30")]
	pub last_mkt: Option<String>,
	/// TradingSessionID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "336")]
	pub trading_session_id: Option<String>,
	/// LastCapacity
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "29")]
	pub last_capacity: Option<LastCapacity>,
	/// Amount of shares open for further execution. If the <a href="tag_39_OrdStatus.html" target="bottom">OrdStatus&nbsp;(39)</a> is Canceled, DoneForTheDay, Expired, Calculated, or Rejected (in which case the order is no longer active) then <a href="tag_151_LeavesQty.html" target="bottom">LeavesQty&nbsp;(151)</a> could be 0, otherwise <a href="tag_151_LeavesQty.html" target="bottom">LeavesQty&nbsp;(151)</a> = <a href="tag_38_OrderQty.html" target="bottom">OrderQty&nbsp;(38)</a> - <a href="tag_14_CumQty.html" target="bottom">CumQty&nbsp;(14)</a> .
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "151")]
	pub leaves_qty: f64,
	/// Currently executed shares for chain of orders.
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "14")]
	pub cum_qty: f64,
	/// AvgPx
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "6")]
	pub avg_px: f64,
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
	/// States whether executions are booked out or accumulated on a partially-filled GT order
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "427")]
	pub gt_booking_inst: Option<GTBookingInst>,
	/// Used when reporting other than current day trades.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "75")]
	pub trade_date: Option<fix_common::LocalMktDate>,
	/// Time the transaction represented by this <a href="message_Execution_Report_8.html" target="main">Execution Report&nbsp;(8)</a> occurred
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<fix_common::UTCTimestamp>,
	/// ReportToExch
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "113")]
	pub report_to_exch: Option<ReportToExch>,
	/// On a fill/partial fill messages, it represents value for that fill/partial fill, on <a href="tag_150_ExecType.html" target="bottom">ExecType&nbsp;(150)</a> =Calculated, it represents cumulative value for the order. Monetary commission values are expressed in the currency reflected
	/// by the <a href="tag_15_Currency.html" target="bottom">Currency&nbsp;(15)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "12")]
	pub commission: Option<f64>,
	/// CommType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "13")]
	pub comm_type: Option<CommType>,
	/// GrossTradeAmt
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "381")]
	pub gross_trade_amt: Option<f64>,
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
	/// Foreign exchange rate used to compute <a href="tag_119_SettlCurrAmt.html" target="bottom">SettlCurrAmt&nbsp;(119)</a> from <a href="tag_15_Currency.html" target="bottom">Currency&nbsp;(15)</a> to <a href="tag_120_SettlCurrency.html" target="bottom">SettlCurrency&nbsp;(120)</a>
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
	/// MaxFloor
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "111")]
	pub max_floor: Option<f64>,
	/// For options
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "77")]
	pub open_close: Option<OpenClose>,
	/// MaxShow
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "210")]
	pub max_show: Option<f64>,
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
	/// Can be used with <a href="tag_40_OrdType.html" target="bottom">OrdType&nbsp;(40)</a> = "Forex - Swap" to specify the "value date" for the future portion of a F/X swap.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "193")]
	pub fut_sett_date_2: Option<fix_common::LocalMktDate>,
	/// Can be used with <a href="tag_40_OrdType.html" target="bottom">OrdType&nbsp;(40)</a> = "Forex - Swap" to specify the order quantity for the future portion of a F/X swap.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "192")]
	pub order_qty_2: Option<f64>,
	/// ClearingFirm
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "439")]
	pub clearing_firm: Option<String>,
	/// ClearingAccount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "440")]
	pub clearing_account: Option<String>,
	/// Default is a single security if not specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "442")]
	pub multi_leg_reporting_type: Option<MultiLegReportingType>,
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
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ExecTransType {
	/// New
	#[serde(rename = "0")]
	New,
	/// Cancel
	#[serde(rename = "1")]
	Cancel,
	/// Correct
	#[serde(rename = "2")]
	Correct,
	/// Status
	#[serde(rename = "3")]
	Status,
}

impl Default for ExecTransType {
	fn default() -> Self {
		ExecTransType::New
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ExecType {
	/// New
	#[serde(rename = "0")]
	New,
	/// Partial fill
	#[serde(rename = "1")]
	PartialFill,
	/// Fill
	#[serde(rename = "2")]
	Fill,
	/// Done for day
	#[serde(rename = "3")]
	DoneForDay,
	/// Canceled
	#[serde(rename = "4")]
	Canceled,
	/// Replace
	#[serde(rename = "5")]
	Replace,
	/// Pending Cancel (e.g. result of <a href="message_Order_Cancel_Request_F.html" target="main">Order Cancel Request&nbsp;(F)</a> )
	#[serde(rename = "6")]
	PendingCancelA,
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
	/// Restated ( <a href="message_Execution_Report_8.html" target="main">ExecutionRpt&nbsp;(8)</a> sent unsolicited by sellside, with <a href="tag_378_ExecRestatementReason.html" target="bottom">ExecRestatementReason&nbsp;(378)</a> set)
	#[serde(rename = "D")]
	RestatedASentUnsolicitedBySellsideWithExecRestatementReasonSet,
	/// Pending Replace (e.g. result of <a href="message_Order_Cancel_Replace_Request_G.html" target="main">Order Cancel/Replace Request&nbsp;(G)</a> )
	#[serde(rename = "E")]
	PendingReplaceA,
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
	/// Replaced
	#[serde(rename = "5")]
	Replaced,
	/// Pending Cancel (e.g. result of <a href="message_Order_Cancel_Request_F.html" target="main">Order Cancel Request&nbsp;(F)</a> )
	#[serde(rename = "6")]
	PendingCancelA,
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
	/// Accepted for bidding
	#[serde(rename = "D")]
	AcceptedForBidding,
	/// Pending Replace (e.g. result of <a href="message_Order_Cancel_Replace_Request_G.html" target="main">Order Cancel/Replace Request&nbsp;(G)</a> )
	#[serde(rename = "E")]
	PendingReplaceA,
}

impl Default for OrdStatus {
	fn default() -> Self {
		OrdStatus::New
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum OrdRejReason {
	/// Broker option
	#[serde(rename = "0")]
	BrokerOption,
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
	/// Unknown Order
	#[serde(rename = "5")]
	UnknownOrder,
	/// Duplicate Order (e.g. dupe ClOrdID)
	#[serde(rename = "6")]
	DuplicateOrder,
	/// Duplicate of a verbally communicated order
	#[serde(rename = "7")]
	DuplicateOfAVerballyCommunicatedOrder,
	/// Stale Order
	#[serde(rename = "8")]
	StaleOrder,
}

impl Default for OrdRejReason {
	fn default() -> Self {
		OrdRejReason::BrokerOption
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
	PartialDeclineOfOrderQty,
}

impl Default for ExecRestatementReason {
	fn default() -> Self {
		ExecRestatementReason::GtCorporateAction
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SettlmntTyp {
	/// Regular
	#[serde(rename = "0")]
	Regular,
	/// Cash
	#[serde(rename = "1")]
	Cash,
	/// Next Day
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
	/// When Issued
	#[serde(rename = "7")]
	WhenIssued,
	/// Sellers Option
	#[serde(rename = "8")]
	SellersOption,
	/// T+5
	#[serde(rename = "9")]
	T5,
}

impl Default for SettlmntTyp {
	fn default() -> Self {
		SettlmntTyp::Regular
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum IDSource {
	/// CUSIP
	#[serde(rename = "1")]
	Cusip,
	/// SEDOL
	#[serde(rename = "2")]
	Sedol,
	/// QUIK
	#[serde(rename = "3")]
	Quik,
	/// ISIN number
	#[serde(rename = "4")]
	IsinNumber,
	/// RIC code
	#[serde(rename = "5")]
	RicCode,
	/// ISO Currency Code
	#[serde(rename = "6")]
	IsoCurrencyCode,
	/// ISO Country Code
	#[serde(rename = "7")]
	IsoCountryCode,
	/// Exchange Symbol
	#[serde(rename = "8")]
	ExchangeSymbol,
	/// Consolidated Tape Association (CTA) Symbol (SIAC CTS/CQS line format)
	#[serde(rename = "9")]
	ConsolidatedTapeAssociationSymbol,
}

impl Default for IDSource {
	fn default() -> Self {
		IDSource::Cusip
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SecurityType {
	/// Bankers Acceptance
	#[serde(rename = "BA")]
	BankersAcceptance,
	/// Convertible Bond (Note not part of ISITC spec)
	#[serde(rename = "CB")]
	ConvertibleBond,
	/// Certificate Of Deposit
	#[serde(rename = "CD")]
	CertificateOfDeposit,
	/// Collateralized Mortgage Obligation
	#[serde(rename = "CMO")]
	CollateralizedMortgageObligation,
	/// Corporate Bond
	#[serde(rename = "CORP")]
	CorporateBond,
	/// Commercial Paper
	#[serde(rename = "CP")]
	CommercialPaper,
	/// Corporate Private Placement
	#[serde(rename = "CPP")]
	CorporatePrivatePlacement,
	/// Common Stock
	#[serde(rename = "CS")]
	CommonStock,
	/// Federal Housing Authority
	#[serde(rename = "FHA")]
	FederalHousingAuthority,
	/// Federal Home Loan
	#[serde(rename = "FHL")]
	FederalHomeLoan,
	/// Federal National Mortgage Association
	#[serde(rename = "FN")]
	FederalNationalMortgageAssociation,
	/// Foreign Exchange Contract
	#[serde(rename = "FOR")]
	ForeignExchangeContract,
	/// Future
	#[serde(rename = "FUT")]
	Future,
	/// Government National Mortgage Association
	#[serde(rename = "GN")]
	GovernmentNationalMortgageAssociation,
	/// Treasuries + Agency DebentureI
	#[serde(rename = "GOVT")]
	TreasuriesAgencyDebentureI,
	/// Mutual Fund
	#[serde(rename = "ET Mortgage IOETTEMF")]
	MutualFund,
	/// Mortgage Interest Only
	#[serde(rename = "MIO")]
	MortgageInterestOnly,
	/// Mortgage Principal Only
	#[serde(rename = "MPO")]
	MortgagePrincipalOnly,
	/// Mortgage Private Placement
	#[serde(rename = "MPP")]
	MortgagePrivatePlacement,
	/// Miscellaneous Pass-Thru
	#[serde(rename = "MPT")]
	MiscellaneousPassThru,
	/// Municipal Bond
	#[serde(rename = "MUNI")]
	MunicipalBond,
	/// No ISITC Security Type
	#[serde(rename = "NONE")]
	NoIsitcSecurityType,
	/// Option
	#[serde(rename = "OPT")]
	Option,
	/// Preferred Stock
	#[serde(rename = "PS")]
	PreferredStock,
	/// Repurchase Agreement
	#[serde(rename = "RP")]
	RepurchaseAgreement,
	/// Reverse Repurchase Agreement
	#[serde(rename = "RVRP")]
	ReverseRepurchaseAgreement,
	/// Student Loan Marketing Association
	#[serde(rename = "SL")]
	StudentLoanMarketingAssociation,
	/// Time Deposit
	#[serde(rename = "TD")]
	TimeDeposit,
	/// US Treasury Bill
	#[serde(rename = "USTB")]
	UsTreasuryBill,
	/// Warrant
	#[serde(rename = "WAR")]
	Warrant,
	/// Cats, Tigers &amp; Lions (a real code: US Treasury Receipts)
	#[serde(rename = "ZOO")]
	CatsTigersAmpLions,
	/// 'Wildcard' entry (used on Security Definition Request message)
	#[serde(rename = "?")]
	WildcardEntry,
}

impl Default for SecurityType {
	fn default() -> Self {
		SecurityType::BankersAcceptance
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PutOrCall {
	/// Put
	#[serde(rename = "0")]
	Put,
	/// Call
	#[serde(rename = "1")]
	Call,
}

impl Default for PutOrCall {
	fn default() -> Self {
		PutOrCall::Put
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum OptAttribute {
	/// Long (a.k.a. 'American')
	#[serde(rename = "L")]
	Long,
	/// Short (a.k.a. 'European')
	#[serde(rename = "S")]
	Short,
}

impl Default for OptAttribute {
	fn default() -> Self {
		OptAttribute::Long
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
	/// Undisclosed (valid for <a href="message_Indication_of_Interest_6.html" target="main">Indication of Interest&nbsp;(6)</a> and <a href="message_New_Order_List_E.html" target="main">List Order&nbsp;(E)</a> messages only)
	#[serde(rename = "7")]
	UndisclosedAAndListOrderMessagesOnly,
	/// Cross (orders where counterparty is an exchange, valid for all messages except IOIs)
	#[serde(rename = "8")]
	Cross,
	/// Cross short
	#[serde(rename = "9")]
	CrossShort,
}

impl Default for Side {
	fn default() -> Self {
		Side::Buy
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
	/// Stop
	#[serde(rename = "3")]
	Stop,
	/// Stop limit
	#[serde(rename = "4")]
	StopLimit,
	/// Market on close
	#[serde(rename = "5")]
	MarketOnClose,
	/// With or without
	#[serde(rename = "6")]
	WithOrWithout,
	/// Limit or better
	#[serde(rename = "7")]
	LimitOrBetter,
	/// Limit with or without
	#[serde(rename = "8")]
	LimitWithOrWithout,
	/// On basis
	#[serde(rename = "9")]
	OnBasis,
	/// On close
	#[serde(rename = "A")]
	OnClose,
	/// Limit on close
	#[serde(rename = "B")]
	LimitOnClose,
	/// Forex - Market
	#[serde(rename = "C")]
	ForexMarket,
	/// Previously quoted
	#[serde(rename = "D")]
	PreviouslyQuoted,
	/// Previously indicated
	#[serde(rename = "E")]
	PreviouslyIndicated,
	/// Forex - Limit
	#[serde(rename = "F")]
	ForexLimit,
	/// Forex - Swap
	#[serde(rename = "G")]
	ForexSwap,
	/// Forex - Previously Quoted
	#[serde(rename = "H")]
	ForexPreviouslyQuoted,
	/// Funari (Limit Day Order with unexecuted portion handled as Market On Close. e.g. Japan)
	#[serde(rename = "I")]
	Funari,
	/// Pegged
	#[serde(rename = "P")]
	Pegged,
}

impl Default for OrdType {
	fn default() -> Self {
		OrdType::Market
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum DiscretionInst {
	/// Related to displayed price
	#[serde(rename = "0")]
	RelatedToDisplayedPrice,
	/// Related to market price
	#[serde(rename = "1")]
	RelatedToMarketPrice,
	/// Related to primary price
	#[serde(rename = "2")]
	RelatedToPrimaryPrice,
	/// Related to local primary price
	#[serde(rename = "3")]
	RelatedToLocalPrimaryPrice,
	/// Related to midpoint price
	#[serde(rename = "4")]
	RelatedToMidpointPrice,
	/// Related to last trade price
	#[serde(rename = "5")]
	RelatedToLastTradePrice,
}

impl Default for DiscretionInst {
	fn default() -> Self {
		DiscretionInst::RelatedToDisplayedPrice
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
	/// Was solicited
	#[serde(rename = "Y")]
	WasSolicited,
	/// Was not solicited
	#[serde(rename = "N")]
	WasNotSolicited,
}

impl Default for SolicitedFlag {
	fn default() -> Self {
		SolicitedFlag::WasSolicited
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TimeInForce {
	/// Day
	#[serde(rename = "0")]
	Day,
	/// Good Till Cancel (GTC)
	#[serde(rename = "1")]
	GoodTillCancel,
	/// At the Opening (OPG)
	#[serde(rename = "2")]
	AtTheOpening,
	/// Immediate or Cancel (IOC)
	#[serde(rename = "3")]
	ImmediateOrCancel,
	/// Fill or Kill (FOK)
	#[serde(rename = "4")]
	FillOrKill,
	/// Good Till Crossing (GTX)
	#[serde(rename = "5")]
	GoodTillCrossing,
	/// Good Till Date
	#[serde(rename = "6")]
	GoodTillDate,
}

impl Default for TimeInForce {
	fn default() -> Self {
		TimeInForce::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ExecInst {
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
	/// Stay on offerside
	#[serde(rename = "0")]
	StayOnOfferside,
	/// No cross (cross is forbidden)
	#[serde(rename = "A")]
	NoCross,
	/// OK to cross
	#[serde(rename = "B")]
	OkToCross,
	/// Call first
	#[serde(rename = "C")]
	CallFirst,
	/// Percent of volume '(indicates that the sender does not want to be all of the volume on the floor vs. a specific percentage)'
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
	/// Institutions only
	#[serde(rename = "I")]
	InstitutionsOnly,
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
}

impl Default for ExecInst {
	fn default() -> Self {
		ExecInst::NotHeld
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Rule80A {
	/// Agency single order
	#[serde(rename = "A")]
	A,
	/// Short exempt transaction (refer to A type)
	#[serde(rename = "B")]
	B,
	/// Program Order, non-index arb, for Member firm/org
	#[serde(rename = "C")]
	C,
	/// Program Order, index arb, for Member firm/org
	#[serde(rename = "D")]
	D,
	/// Short Exempt Transaction for Principal (was incorrectly identified in the FIX spec as "Registered Equity Market Maker trades")
	#[serde(rename = "E")]
	E,
	/// Short exempt transaction (refer to W type)
	#[serde(rename = "F")]
	F,
	/// Short exempt transaction (refer to I type)
	#[serde(rename = "H")]
	H,
	/// Individual Investor, single order
	#[serde(rename = "I")]
	I,
	/// Program Order, index arb, for individual customer
	#[serde(rename = "J")]
	J,
	/// Program Order, non-index arb, for individual customer
	#[serde(rename = "K")]
	K,
	/// Short exempt transaction for member competing market-maker affiliated with the firm clearing the trade (refer to P and O types)
	#[serde(rename = "L")]
	L,
	/// Program Order, index arb, for other member
	#[serde(rename = "M")]
	M,
	/// Program Order, non-index arb, for other member
	#[serde(rename = "N")]
	N,
	/// Proprietary transactions for competing market-maker that is affiliated with the clearing member (was incorrectly identified
	/// in the FIX spec as "Competing dealer trades")
	#[serde(rename = "O")]
	O,
	/// Principal
	#[serde(rename = "P")]
	P,
	/// Transactions for the account of a non-member competing market maker (was incorrectly identified in the FIX spec as "Competing
	/// dealer trades")
	#[serde(rename = "R")]
	R,
	/// Specialist trades
	#[serde(rename = "S")]
	S,
	/// Competing dealer trades
	#[serde(rename = "T")]
	T,
	/// Program Order, index arb, for other agency
	#[serde(rename = "U")]
	U,
	/// All other orders as agent for other member
	#[serde(rename = "W")]
	W,
	/// Short exempt transaction for member competing market-maker not affiliated with the firm clearing the trade (refer to W and
	/// T types)
	#[serde(rename = "X")]
	X,
	/// Program Order, non-index arb, for other agency
	#[serde(rename = "Y")]
	Y,
	/// Short exempt transaction for non-member competing market-maker (refer to A and R types)
	#[serde(rename = "Z")]
	Z,
}

impl Default for Rule80A {
	fn default() -> Self {
		Rule80A::A
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
pub enum GTBookingInst {
	/// book out all trades on day of execution
	#[serde(rename = "0")]
	BookOutAllTradesOnDayOfExecution,
	/// accumulate executions until order is filled or expires
	#[serde(rename = "1")]
	AccumulateExecutionsUntilOrderIsFilledOrExpires,
	/// accumulate until verbally notified otherwise
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
	/// Indicates that party receiving message must report trade
	#[serde(rename = "Y")]
	IndicatesThatPartyReceivingMessageMustReportTrade,
	/// Indicates that party sending message will report trade
	#[serde(rename = "N")]
	IndicatesThatPartySendingMessageWillReportTrade,
}

impl Default for ReportToExch {
	fn default() -> Self {
		ReportToExch::IndicatesThatPartyReceivingMessageMustReportTrade
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum CommType {
	/// per share
	#[serde(rename = "1")]
	PerShare,
	/// percentage
	#[serde(rename = "2")]
	Percentage,
	/// absolute
	#[serde(rename = "3")]
	Absolute,
}

impl Default for CommType {
	fn default() -> Self {
		CommType::PerShare
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
pub enum OpenClose {
	/// Open
	#[serde(rename = "O")]
	Open,
	/// Close
	#[serde(rename = "C")]
	Close,
}

impl Default for OpenClose {
	fn default() -> Self {
		OpenClose::Open
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
