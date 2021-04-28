
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TradeCaptureReport {
	/// MsgType = AE
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'A', 'E'>,
	/// ApplicationSequenceControl
	#[serde(flatten)]
	pub application_sequence_control: Option<super::super::application_sequence_control::ApplicationSequenceControl>,
	/// TradeReportID is conditionally required in a message-chaining model in which a subsequent message may refer to a prior message
	/// via TradeReportRefID. The alternative to a message-chain model is an entity-based model in which <a href="tag_1003_TradeID.html" target="bottom">TradeID&nbsp;(1003)</a> is used to identify a trade. In this case, <a href="tag_1003_TradeID.html" target="bottom">TradeID&nbsp;(1003)</a> is required and <a href="tag_571_TradeReportID.html" target="bottom">TradeReportID&nbsp;(571)</a> can be optionally specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "571")]
	pub trade_report_id: Option<String>,
	/// TradeID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1003")]
	pub trade_id: Option<String>,
	/// SecondaryTradeID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1040")]
	pub secondary_trade_id: Option<String>,
	/// FirmTradeID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1041")]
	pub firm_trade_id: Option<String>,
	/// SecondaryFirmTradeID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1042")]
	pub secondary_firm_trade_id: Option<String>,
	/// TradeReportTransType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "487")]
	pub trade_report_trans_type: Option<TradeReportTransType>,
	/// TradeReportType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "856")]
	pub trade_report_type: Option<TradeReportType>,
	/// <p>Status of the trade report. In 3-party listed derivatives model, this is used to convey status of a trade to a counterparty.
	/// Used specifically in a "give-up" (also known as "claim") model.
	/// </p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "939")]
	pub trd_rpt_status: Option<TrdRptStatus>,
	/// Identifier for the trade capture report request associated with this trade capture report.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "568")]
	pub trade_request_id: Option<String>,
	/// TrdType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "828")]
	pub trd_type: Option<TrdType>,
	/// TrdSubType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "829")]
	pub trd_sub_type: Option<TrdSubType>,
	/// SecondaryTrdType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "855")]
	pub secondary_trd_type: Option<SecondaryTrdType>,
	/// TradeHandlingInstr
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1123")]
	pub trade_handling_instr: Option<TradeHandlingInstr>,
	/// OrigTradeHandlingInstr
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1124")]
	pub orig_trade_handling_instr: Option<OrigTradeHandlingInstr>,
	/// OrigTradeDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1125")]
	pub orig_trade_date: Option<fix_common::LocalMktDate>,
	/// OrigTradeID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1126")]
	pub orig_trade_id: Option<String>,
	/// OrigSecondaryTradeID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1127")]
	pub orig_secondary_trade_id: Option<String>,
	/// TransferReason
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "830")]
	pub transfer_reason: Option<String>,
	/// Type of execution being reported. Uses subset of <a href="tag_150_ExecType.html" target="bottom">ExecType&nbsp;(150)</a> for trade capture reports
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "150")]
	pub exec_type: Option<ExecType>,
	/// TotNumTradeReports
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "748")]
	pub tot_num_trade_reports: Option<i32>,
	/// LastRptRequested
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "912")]
	pub last_rpt_requested: Option<LastRptRequested>,
	/// Set to 'Y' if message is sent as a result of a subscription request or out of band configuration
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "325")]
	pub unsolicited_indicator: Option<UnsolicitedIndicator>,
	/// If the field is absent, SubscriptionRequestType(263)=0(Snapshot) will be the default
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "263")]
	pub subscription_request_type: Option<SubscriptionRequestType>,
	/// The <a href="tag_571_TradeReportID.html" target="bottom">TradeReportID&nbsp;(571)</a> that is being referenced for trade correction or cancellation
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "572")]
	pub trade_report_ref_id: Option<String>,
	/// (Deprecated in FIX.5.0)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "881")]
	pub secondary_trade_report_ref_id: Option<String>,
	/// (Deprecated in FIX.5.0)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "818")]
	pub secondary_trade_report_id: Option<String>,
	/// TradeLinkID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "820")]
	pub trade_link_id: Option<String>,
	/// TrdMatchID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "880")]
	pub trd_match_id: Option<String>,
	/// Market (exchange) assigned execution identifier as provided in the ExecutionReport(35=8) message. Conditionally required if
	/// ExecRefID(19) is present and refers to the new execution identifer assigned by the market (exchange).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "17")]
	pub exec_id: Option<String>,
	/// SecondaryExecID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "527")]
	pub secondary_exec_id: Option<String>,
	/// ExecRestatementReason
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "378")]
	pub exec_restatement_reason: Option<ExecRestatementReason>,
	/// PreviouslyReported
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "570")]
	pub previously_reported: Option<PreviouslyReported>,
	/// Can be used to indicate cabinet trade pricing
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "423")]
	pub price_type: Option<PriceType>,
	/// RootParties
	#[serde(flatten)]
	pub root_parties: Option<super::super::root_parties::RootParties>,
	/// AsOfIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1015")]
	pub as_of_indicator: Option<AsOfIndicator>,
	/// SettlSessID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "716")]
	pub settl_sess_id: Option<SettlSessID>,
	/// SettlSessSubID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "717")]
	pub settl_sess_sub_id: Option<String>,
	/// VenueType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1430")]
	pub venue_type: Option<VenueType>,
	/// MarketSegmentID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1300")]
	pub market_segment_id: Option<String>,
	/// MarketID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1301")]
	pub market_id: Option<String>,
	/// Instrument
	#[serde(flatten)]
	pub instrument: Option<super::super::instrument::Instrument>,
	/// FinancingDetails
	#[serde(flatten)]
	pub financing_details: Option<super::super::financing_details::FinancingDetails>,
	/// QtyType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "854")]
	pub qty_type: Option<QtyType>,
	/// YieldData
	#[serde(flatten)]
	pub yield_data: Option<super::super::yield_data::YieldData>,
	/// UndInstrmtGrp
	#[serde(flatten)]
	pub und_instrmt_grp: Option<super::super::und_instrmt_grp::UndInstrmtGrp>,
	/// UnderlyingTradingSessionID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "822")]
	pub underlying_trading_session_id: Option<String>,
	/// UnderlyingTradingSessionSubID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "823")]
	pub underlying_trading_session_sub_id: Option<String>,
	/// LastQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "32")]
	pub last_qty: Option<f64>,
	/// LastPx
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "31")]
	pub last_px: Option<f64>,
	/// CalculatedCcyLastQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1056")]
	pub calculated_ccy_last_qty: Option<f64>,
	/// PriceMarkup
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2762")]
	pub price_markup: Option<f64>,
	/// AveragePriceDetail
	#[serde(flatten)]
	pub average_price_detail: Option<super::super::average_price_detail::AveragePriceDetail>,
	/// Primary currency of the specified currency pair. Used to qualify <a href="tag_32_LastQty.html" target="bottom">LastQty&nbsp;(32)</a> and <a href="tag_381_GrossTradeAmt.html" target="bottom">GrossTradeAmout&nbsp;(381)</a>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "15")]
	pub currency: Option<Currency>,
	/// Contra currency of the deal. Used to qualify <a href="tag_1056_CalculatedCcyLastQty.html" target="bottom">CalculatedCcyLastQty&nbsp;(1056)</a>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "120")]
	pub settl_currency: Option<SettlCurrency>,
	/// LastParPx
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
	/// LastSwapPoints
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1071")]
	pub last_swap_points: Option<f64>,
	/// LastMkt
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "30")]
	pub last_mkt: Option<String>,
	/// Used when reporting other than current day trades.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "75")]
	pub trade_date: Option<fix_common::LocalMktDate>,
	/// ClearingBusinessDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "715")]
	pub clearing_business_date: Option<fix_common::LocalMktDate>,
	/// Average <a href="tag_44_Price.html" target="bottom">Price&nbsp;(44)</a> - if present then the <a href="tag_31_LastPx.html" target="bottom">LastPx&nbsp;(31)</a> will contain the original price on the execution
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "6")]
	pub avg_px: Option<f64>,
	/// Insert here the set of "SpreadOrBenchmarkCurveData" fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub spread_or_benchmark_curve_data: Option<super::super::spread_or_benchmark_curve_data::SpreadOrBenchmarkCurveData>,
	/// Average Pricing indicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "819")]
	pub avg_px_indicator: Option<AvgPxIndicator>,
	/// PositionAmountData
	#[serde(flatten)]
	pub position_amount_data: Option<super::super::position_amount_data::PositionAmountData>,
	/// Type of report if multileg instrument. Provided to support a scenario for trades of multileg instruments between two parties.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "442")]
	pub multi_leg_reporting_type: Option<MultiLegReportingType>,
	/// Reference to the leg of a multileg instrument to which this trade refers. Used when <a href="tag_442_MultiLegReportingType.html" target="bottom">MultiLegReportingType&nbsp;(442)</a> = 2 (Individual Leg of a multileg security)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "824")]
	pub trade_leg_ref_id: Option<String>,
	/// Identifies a multi-leg execution if present and non-zero.
	#[serde(flatten)]
	pub trd_instrmt_leg_grp: Option<super::super::trd_instrmt_leg_grp::TrdInstrmtLegGrp>,
	/// Time the transaction represented by when this <a href="message_TradeCaptureReport_AE.html" target="main">Trade Capture Report&nbsp;(AE)</a> occurred. Execution time of trade. Also describes the time of block trades.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<fix_common::UTCTimestamp>,
	/// TrdRegTimestamps
	#[serde(flatten)]
	pub trd_reg_timestamps: Option<super::super::trd_reg_timestamps::TrdRegTimestamps>,
	/// SettlType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "63")]
	pub settl_type: Option<SettlType>,
	/// Takes precedence over <a href="tag_63_SettlType.html" target="bottom">SettlType&nbsp;(63)</a> value and conditionally required/omitted for specific <a href="tag_63_SettlType.html" target="bottom">SettlType&nbsp;(63)</a> values.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "64")]
	pub settl_date: Option<fix_common::LocalMktDate>,
	/// The settlement date for the underlying instrument of a derivatives security.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "987")]
	pub underlying_settlement_date: Option<fix_common::LocalMktDate>,
	/// MatchStatus
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "573")]
	pub match_status: Option<MatchStatus>,
	/// MatchType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "574")]
	pub match_type: Option<MatchType>,
	/// Number of sides.
	#[serde(flatten)]
	pub trd_cap_rpt_side_grp: super::super::trd_cap_rpt_side_grp::TrdCapRptSideGrp,
	/// Volatility
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1188")]
	pub volatility: Option<f64>,
	/// DividendYield
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1380")]
	pub dividend_yield: Option<f32>,
	/// RiskFreeRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1190")]
	pub risk_free_rate: Option<f64>,
	/// CurrencyRatio
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1382")]
	pub currency_ratio: Option<f64>,
	/// CopyMsgIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "797")]
	pub copy_msg_indicator: Option<fix_common::Boolean>,
	/// TrdRepIndicatorsGrp
	#[serde(flatten)]
	pub trd_rep_indicators_grp: Option<super::super::trd_rep_indicators_grp::TrdRepIndicatorsGrp>,
	/// (Deprecated in FIX.5.0)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "852")]
	pub publish_trd_indicator: Option<PublishTrdIndicator>,
	/// TradePublishIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1390")]
	pub trade_publish_indicator: Option<TradePublishIndicator>,
	/// ShortSaleReason
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "853")]
	pub short_sale_reason: Option<ShortSaleReason>,
	/// Indicates the algorithm (tier) used to match a trade
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "994")]
	pub tier_code: Option<String>,
	/// MessageEventSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1011")]
	pub message_event_source: Option<String>,
	/// Used to indicate reports after a specific time
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "779")]
	pub last_update_time: Option<fix_common::UTCTimestamp>,
	/// Specifies the rounded price to quoted precision.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "991")]
	pub rnd_px: Option<f64>,
	/// TZTransactTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1132")]
	pub tz_transact_time: Option<fix_common::TZTimestamp>,
	/// ReportedPxDiff
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1134")]
	pub reported_px_diff: Option<fix_common::Boolean>,
	/// ( <a href="tag_32_LastQty.html" target="bottom">LastQty&nbsp;(32)</a> * <a href="tag_31_LastPx.html" target="bottom">LastPx&nbsp;(31)</a> or <a href="tag_669_LastParPx.html" target="bottom">LastParPx&nbsp;(669)</a> ) For Fixed Income, <a href="tag_669_LastParPx.html" target="bottom">LastParPx&nbsp;(669)</a> is used when <a href="tag_31_LastPx.html" target="bottom">LastPx&nbsp;(31)</a> is not expressed as "percent of par" price.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "381")]
	pub gross_trade_amt: Option<f64>,
	/// Used by CCP to send a reason for rejection
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1328")]
	pub reject_text: Option<String>,
	/// FeeMultiplier
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1329")]
	pub fee_multiplier: Option<f64>,
	/// CrossType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "549")]
	pub cross_type: Option<CrossType>,
	/// Must be set if <a href="tag_1665_EncodedRejectText.html" target="bottom">EncodedRejectTextLen(1665)&nbsp;(1665)</a> field is specified and must immediately precede it.
	#[serde(rename = "1664")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_1328_RejectText.html" target="bottom">RejectedText(1328)&nbsp;(1328)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding(347)&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "1665")]
	pub encoded_reject_text: Option<fix_common::EncodedText<1665>>,
	/// Used to specify the differential price when reporting the individual leg of a spread trade.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1522")]
	pub differential_price: Option<f64>,
	/// Indicates the reason that a trade report was rejected.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "751")]
	pub trade_report_reject_reason: Option<TradeReportRejectReason>,
	/// Used when clearing price differs from execution price.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1596")]
	pub clearing_trade_price: Option<f64>,
	/// TradePriceNegotiationMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1740")]
	pub trade_price_negotiation_method: Option<TradePriceNegotiationMethod>,
	/// Upfront Price for CDS transactions. Conditionally required if TradePriceNegotiationMethod(1740) = 4(Percent of Par and Upfront
	/// Amount), 5(Deal Spread and Upfront Amount) or 6(Upfront Points and Upfront Amount)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1743")]
	pub last_upfront_price: Option<f64>,
	/// UpfrontPriceType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1741")]
	pub upfront_price_type: Option<UpfrontPriceType>,
	/// LastQtyVariance
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1828")]
	pub last_qty_variance: Option<f64>,
	/// OffsetInstruction
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1849")]
	pub offset_instruction: Option<OffsetInstruction>,
	/// TradePriceConditionGrp
	#[serde(flatten)]
	pub trade_price_condition_grp: Option<super::super::trade_price_condition_grp::TradePriceConditionGrp>,
	/// AvgPxGroupID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1731")]
	pub avg_px_group_id: Option<String>,
	/// TradeQtyGrp
	#[serde(flatten)]
	pub trade_qty_grp: Option<super::super::trade_qty_grp::TradeQtyGrp>,
	/// TimeToExpiration
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1189")]
	pub time_to_expiration: Option<f64>,
	/// PriceDelta
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "811")]
	pub price_delta: Option<f64>,
	/// RegulatoryTradeIDGrp
	#[serde(flatten)]
	pub regulatory_trade_id_grp: Option<super::super::regulatory_trade_id_grp::RegulatoryTradeIDGrp>,
	/// PaymentGrp
	#[serde(flatten)]
	pub payment_grp: Option<super::super::payment_grp::PaymentGrp>,
	/// ClearedIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1832")]
	pub cleared_indicator: Option<ClearedIndicator>,
	/// ClearingIntention
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1924")]
	pub clearing_intention: Option<ClearingIntention>,
	/// TradeClearingInstruction
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1925")]
	pub trade_clearing_instruction: Option<TradeClearingInstruction>,
	/// BackloadedTradeIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1926")]
	pub backloaded_trade_indicator: Option<fix_common::Boolean>,
	/// ConfirmationMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1927")]
	pub confirmation_method: Option<ConfirmationMethod>,
	/// MandatoryClearingIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1928")]
	pub mandatory_clearing_indicator: Option<fix_common::Boolean>,
	/// MixedSwapIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1929")]
	pub mixed_swap_indicator: Option<fix_common::Boolean>,
	/// OffMarketPriceIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1930")]
	pub off_market_price_indicator: Option<fix_common::Boolean>,
	/// VerificationMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1931")]
	pub verification_method: Option<VerificationMethod>,
	/// ClearingRequirementException
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1932")]
	pub clearing_requirement_exception: Option<ClearingRequirementException>,
	/// IRSDirection
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1933")]
	pub irs_direction: Option<IRSDirection>,
	/// RegulatoryReportType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1934")]
	pub regulatory_report_type: Option<RegulatoryReportType>,
	/// VoluntaryRegulatoryReport
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1935")]
	pub voluntary_regulatory_report: Option<fix_common::Boolean>,
	/// TradeCollateralization
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1936")]
	pub trade_collateralization: Option<TradeCollateralization>,
	/// TradeContinuation
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1937")]
	pub trade_continuation: Option<TradeContinuation>,
	/// AttachmentGrp
	#[serde(flatten)]
	pub attachment_grp: Option<super::super::attachment_grp::AttachmentGrp>,
	/// LastQtyChanged
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2301")]
	pub last_qty_changed: Option<f64>,
	/// ValuationDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2085")]
	pub valuation_date: Option<fix_common::LocalMktDate>,
	/// ValuationTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2086")]
	pub valuation_time: Option<String>,
	/// ValuationBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2087")]
	pub valuation_business_center: Option<String>,
	/// MandatoryClearingJurisdictionGrp
	#[serde(flatten)]
	pub mandatory_clearing_jurisdiction_grp: Option<super::super::mandatory_clearing_jurisdiction_grp::MandatoryClearingJurisdictionGrp>,
	/// TradeVersion
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2302")]
	pub trade_version: Option<String>,
	/// HistoricalReportIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2303")]
	pub historical_report_indicator: Option<fix_common::Boolean>,
	/// RiskLimitCheckStatus
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2343")]
	pub risk_limit_check_status: Option<RiskLimitCheckStatus>,
	/// MidPx
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "631")]
	pub mid_px: Option<f64>,
	/// RegulatoryTransactionType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2347")]
	pub regulatory_transaction_type: Option<RegulatoryTransactionType>,
	/// TaxonomyType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2375")]
	pub taxonomy_type: Option<TaxonomyType>,
	/// CollateralAmountGrp
	#[serde(flatten)]
	pub collateral_amount_grp: Option<super::super::collateral_amount_grp::CollateralAmountGrp>,
	/// LastMultipliedQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2368")]
	pub last_multiplied_qty: Option<f64>,
	/// TotalTradeQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2367")]
	pub total_trade_qty: Option<f64>,
	/// TotalTradeMultipliedQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2370")]
	pub total_trade_multiplied_qty: Option<f64>,
	/// For FX trades expresses whether to multiply or divide LastPx(31) to arrive at GrossTradeAmt(381).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2366")]
	pub settl_price_fx_rate_calc: Option<SettlPriceFxRateCalc>,
	/// TotalGrossTradeAmt
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2369")]
	pub total_gross_trade_amt: Option<f64>,
	/// TradeContinuationText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2374")]
	pub trade_continuation_text: Option<String>,
	/// Must be set if EncodedTradeContinuationText(2371) field is specified and must immediately precede it.
	#[serde(rename = "2372")]
	/// Encoded (non-ASCII characters) representation of the TradeContinuationText(2374) field in the encoded format specified via
	/// the MessageEncoding(347) field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "2371")]
	pub encoded_trade_continuation_text: Option<fix_common::EncodedText<2371>>,
	/// IntraFirmTradeIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2373")]
	pub intra_firm_trade_indicator: Option<IntraFirmTradeIndicator>,
	/// ExecMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2405")]
	pub exec_method: Option<ExecMethod>,
	/// RelatedInstrumentGrp
	#[serde(flatten)]
	pub related_instrument_grp: Option<super::super::related_instrument_grp::RelatedInstrumentGrp>,
	/// RateSource
	#[serde(flatten)]
	pub rate_source: Option<super::super::rate_source::RateSource>,
	/// PricePrecision
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2349")]
	pub price_precision: Option<i32>,
	/// TradeContingency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2387")]
	pub trade_contingency: Option<TradeContingency>,
	/// PackageID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2489")]
	pub package_id: Option<String>,
	/// TradeNumber
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2490")]
	pub trade_number: Option<i32>,
	/// InstrumentExtension
	#[serde(flatten)]
	pub instrument_extension: Option<super::super::instrument_extension::InstrumentExtension>,
	/// MultiAssetSwapIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2527")]
	pub multi_asset_swap_indicator: Option<MultiAssetSwapIndicator>,
	/// InternationalSwapIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2526")]
	pub international_swap_indicator: Option<InternationalSwapIndicator>,
	/// AffiliatedFirmsTradeIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2525")]
	pub affiliated_firms_trade_indicator: Option<AffiliatedFirmsTradeIndicator>,
	/// DeltaCrossed
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2596")]
	pub delta_crossed: Option<fix_common::Boolean>,
	/// AlgorithmicTradeIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2667")]
	pub algorithmic_trade_indicator: Option<AlgorithmicTradeIndicator>,
	/// TrdRegPublicationGrp
	#[serde(flatten)]
	pub trd_reg_publication_grp: Option<super::super::trd_reg_publication_grp::TrdRegPublicationGrp>,
	/// PriceQualifierGrp
	#[serde(flatten)]
	pub price_qualifier_grp: Option<super::super::price_qualifier_grp::PriceQualifierGrp>,
	/// CollateralizationValueDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2868")]
	pub collateralization_value_date: Option<fix_common::LocalMktDate>,
	/// TransactionAttributeGrp
	#[serde(flatten)]
	pub transaction_attribute_grp: Option<super::super::transaction_attribute_grp::TransactionAttributeGrp>,
	/// ClearingPortfolioID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2870")]
	pub clearing_portfolio_id: Option<String>,
	/// TerminationDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2878")]
	pub termination_date: Option<fix_common::LocalMktDate>,
	/// May be used when the business event date differs from when the regulatory report is actually being submitted (typically specified
	/// in TrdRegTimestamps component).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2869")]
	pub regulatory_report_type_business_date: Option<fix_common::LocalMktDate>,
	/// May be used to indicate manual reporting of the trade.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1028")]
	pub manual_order_indicator: Option<fix_common::Boolean>,
	/// Reference to an execution identifier previously assigned by the market (exchange). If specified, ExecID(17) is required.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "19")]
	pub exec_ref_id: Option<String>,
	/// TradeReportingIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2524")]
	pub trade_reporting_indicator: Option<TradeReportingIndicator>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TradeReportTransType {
	/// New
	#[serde(rename = "0")]
	New,
	/// Cancel
	#[serde(rename = "1")]
	Cancel,
	/// Replace
	#[serde(rename = "2")]
	Replace,
	/// Release
	#[serde(rename = "3")]
	Release,
	/// Reverse
	#[serde(rename = "4")]
	Reverse,
	/// Cancel Due To Back Out of Trade
	#[serde(rename = "5")]
	CancelDueToBackOutOfTrade,
}

impl Default for TradeReportTransType {
	fn default() -> Self {
		TradeReportTransType::New
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TradeReportType {
	/// Submit
	#[serde(rename = "0")]
	Submit,
	/// Alleged
	#[serde(rename = "1")]
	Alleged,
	/// Accept
	#[serde(rename = "2")]
	Accept,
	/// Decline
	#[serde(rename = "3")]
	Decline,
	/// Addendum (Used to provide material supplemental data to a previously submitted trade)
	#[serde(rename = "4")]
	Addendum,
	/// No/Was (Used to report a full replacement of a previously submitted trade)
	#[serde(rename = "5")]
	NoWas,
	/// Trade Report Cancel
	#[serde(rename = "6")]
	TradeReportCancel,
	/// (Locked-In) Trade Break
	#[serde(rename = "7")]
	TradeBreak,
	/// Defaulted
	#[serde(rename = "8")]
	Defaulted,
	/// Invalid CMTA
	#[serde(rename = "9")]
	InvalidCmta,
	/// Pended
	#[serde(rename = "10")]
	Pended,
	/// Alleged New
	#[serde(rename = "11")]
	AllegedNew,
	/// Alleged Addendum
	#[serde(rename = "12")]
	AllegedAddendum,
	/// Alleged No/Was
	#[serde(rename = "13")]
	AllegedNoWas,
	/// Alleged Trade Report Cancel
	#[serde(rename = "14")]
	AllegedTradeReportCancel,
	/// Alleged (Locked-In) Trade Break
	#[serde(rename = "15")]
	AllegedTradeBreak,
	/// Verify (Used in reports from a trading party to the SDR to confirm trade details. Omit RegulatoryReportType(1934))
	#[serde(rename = "16")]
	Verify,
	/// Dispute (Used in reports from a trading party to the SDR to dispute trade details. Omit RegulatoryReportType(1934))
	#[serde(rename = "17")]
	Dispute,
	/// Non-material Update (Used to provide nonmaterial supplemental data to a previously submitted trade)
	#[serde(rename = "18")]
	NonMaterialUpdate,
}

impl Default for TradeReportType {
	fn default() -> Self {
		TradeReportType::Submit
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TrdRptStatus {
	/// Accepted
	#[serde(rename = "0")]
	Accepted,
	/// Rejected
	#[serde(rename = "1")]
	Rejected,
	/// Accepted with errors
	#[serde(rename = "3")]
	AcceptedWithErrors,
	/// Cancelled
	#[serde(rename = "2")]
	Cancelled,
	/// Pending New
	#[serde(rename = "4")]
	PendingNew,
	/// Pending Cancel
	#[serde(rename = "5")]
	PendingCancel,
	/// Pending Replace
	#[serde(rename = "6")]
	PendingReplace,
	/// Terminated
	#[serde(rename = "7")]
	Terminated,
	/// Pending verification (Used in reports from the SDR to the regulator and to trading parties to indicate that the trade details
	/// have not been verified by one or both parties.)
	#[serde(rename = "8")]
	PendingVerification,
	/// Deemed verified (Used in reports from the SDR to the regulator and to trading parties to indicate that the trade details are
	/// deemed verified by the SDR by have not been confirmed by the trading parties.)
	#[serde(rename = "9")]
	DeemedVerified,
	/// Verified (Used in reports from the SDR to the regulator and to trading parties to indicate that the trade details have been
	/// confirmed by the trading parties)
	#[serde(rename = "10")]
	Verified,
	/// Disputed (Used in reports from the SDR to the regulator and to trading parties to indicate that the trade details have been
	/// disputed by a trading party)
	#[serde(rename = "11")]
	Disputed,
}

impl Default for TrdRptStatus {
	fn default() -> Self {
		TrdRptStatus::Accepted
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

impl Default for TrdType {
	fn default() -> Self {
		TrdType::N0
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

impl Default for TrdSubType {
	fn default() -> Self {
		TrdSubType::Cmta
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SecondaryTrdType {
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

impl Default for SecondaryTrdType {
	fn default() -> Self {
		SecondaryTrdType::N0
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TradeHandlingInstr {
	/// Trade Confirmation
	#[serde(rename = "0")]
	TradeConfirmation,
	/// Two-Party Report
	#[serde(rename = "1")]
	TwoPartyReport,
	/// One-Party Report for Matching
	#[serde(rename = "2")]
	OnePartyReportForMatching,
	/// One-Party Report for Pass Through
	#[serde(rename = "3")]
	OnePartyReportForPassThrough,
	/// Automated Floor Order Routing
	#[serde(rename = "4")]
	AutomatedFloorOrderRouting,
	/// Two party report for claim
	#[serde(rename = "5")]
	TwoPartyReportForClaim,
	/// One-party report
	#[serde(rename = "6")]
	OnePartyReport,
	/// Third-party report for pass through
	#[serde(rename = "7")]
	ThirdPartyReportForPassThrough,
	/// One-party report for auto-match
	#[serde(rename = "8")]
	OnePartyReportForAutoMatch,
}

impl Default for TradeHandlingInstr {
	fn default() -> Self {
		TradeHandlingInstr::TradeConfirmation
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum OrigTradeHandlingInstr {
	/// Trade Confirmation
	#[serde(rename = "0")]
	TradeConfirmation,
	/// Two-Party Report
	#[serde(rename = "1")]
	TwoPartyReport,
	/// One-Party Report for Matching
	#[serde(rename = "2")]
	OnePartyReportForMatching,
	/// One-Party Report for Pass Through
	#[serde(rename = "3")]
	OnePartyReportForPassThrough,
	/// Automated Floor Order Routing
	#[serde(rename = "4")]
	AutomatedFloorOrderRouting,
	/// Two party report for claim
	#[serde(rename = "5")]
	TwoPartyReportForClaim,
}

impl Default for OrigTradeHandlingInstr {
	fn default() -> Self {
		OrigTradeHandlingInstr::TradeConfirmation
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
pub enum UnsolicitedIndicator {
	/// Message is being sent as a result of a prior request
	#[serde(rename = "N")]
	MessageIsBeingSentAsAResultOfAPriorRequest,
	/// Message is being sent unsolicited
	#[serde(rename = "Y")]
	MessageIsBeingSentUnsolicited,
}

impl Default for UnsolicitedIndicator {
	fn default() -> Self {
		UnsolicitedIndicator::MessageIsBeingSentAsAResultOfAPriorRequest
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SubscriptionRequestType {
	/// Snapshot
	#[serde(rename = "0")]
	Snapshot,
	/// Snapshot + Updates (Subscribe)
	#[serde(rename = "1")]
	SnapshotUpdates,
	/// Disable previous Snapshot + Update Request (Unsubscribe)
	#[serde(rename = "2")]
	DisablePreviousSnapshotUpdateRequest,
}

impl Default for SubscriptionRequestType {
	fn default() -> Self {
		SubscriptionRequestType::Snapshot
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

impl Default for ExecRestatementReason {
	fn default() -> Self {
		ExecRestatementReason::GtCorporateAction
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PreviouslyReported {
	/// Not reported to counterparty or market
	#[serde(rename = "N")]
	NotReportedToCounterpartyOrMarket,
	/// Previously reported to counterparty or market
	#[serde(rename = "Y")]
	PreviouslyReportedToCounterpartyOrMarket,
}

impl Default for PreviouslyReported {
	fn default() -> Self {
		PreviouslyReported::NotReportedToCounterpartyOrMarket
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

impl Default for PriceType {
	fn default() -> Self {
		PriceType::Percentage
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum AsOfIndicator {
	/// false - trade is not an AsOf trade
	#[serde(rename = "0")]
	FalseTradeIsNotAnAsOfTrade,
	/// true - trade is an AsOf trade
	#[serde(rename = "1")]
	TrueTradeIsAnAsOfTrade,
}

impl Default for AsOfIndicator {
	fn default() -> Self {
		AsOfIndicator::FalseTradeIsNotAnAsOfTrade
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SettlSessID {
	/// Intraday
	#[serde(rename = "ITD")]
	Intraday,
	/// Regular Trading Hours
	#[serde(rename = "RTH")]
	RegularTradingHours,
	/// Electronic Trading Hours
	#[serde(rename = "ETH")]
	ElectronicTradingHours,
	/// End Of Day
	#[serde(rename = "EOD")]
	EndOfDay,
}

impl Default for SettlSessID {
	fn default() -> Self {
		SettlSessID::Intraday
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

impl Default for VenueType {
	fn default() -> Self {
		VenueType::Electronic
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
	UnitsOfMeasurePerTimeUnitAAndTimeUnit,
}

impl Default for QtyType {
	fn default() -> Self {
		QtyType::Units
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

impl Default for AvgPxIndicator {
	fn default() -> Self {
		AvgPxIndicator::NoAveragePricing
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
	BrokenDateForFxExpressingNonStandardTenorSettlDateMustBeSpecified,
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

impl Default for MatchType {
	fn default() -> Self {
		MatchType::ActAcceptedTrade
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PublishTrdIndicator {
	/// Do Not Report Trade
	#[serde(rename = "N")]
	DoNotReportTrade,
	/// Report Trade
	#[serde(rename = "Y")]
	ReportTrade,
}

impl Default for PublishTrdIndicator {
	fn default() -> Self {
		PublishTrdIndicator::DoNotReportTrade
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

impl Default for TradePublishIndicator {
	fn default() -> Self {
		TradePublishIndicator::DoNotPublishTrade
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ShortSaleReason {
	/// Dealer Sold Short
	#[serde(rename = "0")]
	DealerSoldShort,
	/// Dealer Sold Short Exempt
	#[serde(rename = "1")]
	DealerSoldShortExempt,
	/// Selling Customer Sold Short
	#[serde(rename = "2")]
	SellingCustomerSoldShort,
	/// Selling Customer Sold Short Exempt
	#[serde(rename = "3")]
	SellingCustomerSoldShortExempt,
	/// Qualified Service Representative (QSR) or Automatic Give-up (AGU) Contra Side Sold Short
	#[serde(rename = "4")]
	QualifiedServiceRepresentativeOrAutomaticGiveUpContraSideSoldShort,
	/// QSR or AGU Contra Side Sold Short Exempt
	#[serde(rename = "5")]
	QsrOrAguContraSideSoldShortExempt,
}

impl Default for ShortSaleReason {
	fn default() -> Self {
		ShortSaleReason::DealerSoldShort
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum CrossType {
	/// Cross AON - cross tade which is executed complete or not. Both sides are treated in the same manner. This is equivalent to
	/// an "All or None"."
	#[serde(rename = "1")]
	CrossAonCrossTadeWhichIsExecutedCompleteOrNotBothSidesAreTreatedInTheSameMannerThisIsEquivalentToAnAllOrNone,
	/// Cross IOC - cross trade which is executed partially and the rest is cancelled. One side is fully executed, the other side
	/// is partially executed with the remainder being cancelled. This is equivalent to an IOC on the other side. Note: <a href="tag_550_CrossPrioritization.html" target="bottom">CrossPrioritization&nbsp;(550)</a> field may be used to indicate which side should fully execute in this scenario.
	#[serde(rename = "2")]
	CrossIocCrossTradeWhichIsExecutedPartiallyAndTheRestIsCancelledOneSideIsFullyExecutedTheOtherSideIsPartiallyExecutedWithTheRemainderBeingCancelledThisIsEquivalentToAnIocOnTheOtherSideNoteCrossPrioritizationFieldMayBeUsedToIndicateWhichSideShouldFullyExecuteInThisScenario,
	/// Cross One Side - cross trade which is partially executed with the unfilled portions remaining active. One side of the corss
	/// is fully executed (as denoted by the <a href="tag_550_CrossPrioritization.html" target="bottom">CrossPrioritization&nbsp;(550)</a> field), but the unfilled portion remains active.
	#[serde(rename = "3")]
	CrossOneSideCrossTradeWhichIsPartiallyExecutedWithTheUnfilledPortionsRemainingActiveOneSideOfTheCorssIsFullyExecutedAFieldButTheUnfilledPortionRemainsActive,
	/// Cross Same Price - cross trade is executed with existing orders with the same price. In this case other orders exist with
	/// the same price, the quantity of the Cross is executed against the existing orders and quotes, the remainder of the corss is
	/// executed against the other side of the cross. The two sides potentially have different quantities.
	#[serde(rename = "4")]
	CrossSamePriceCrossTradeIsExecutedWithExistingOrdersWithTheSamePriceInThisCaseOtherOrdersExistWithTheSamePriceTheQuantityOfTheCrossIsExecutedAgainstTheExistingOrdersAndQuotesTheRemainderOfTheCorssIsExecutedAgainstTheOtherSideOfTheCrossTheTwoSidesPotentiallyHaveDifferentQuantities,
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

impl Default for CrossType {
	fn default() -> Self {
		CrossType::CrossAonCrossTadeWhichIsExecutedCompleteOrNotBothSidesAreTreatedInTheSameMannerThisIsEquivalentToAnAllOrNone
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TradeReportRejectReason {
	/// Successful (default)
	#[serde(rename = "0")]
	Successful,
	/// Invalid party information
	#[serde(rename = "1")]
	InvalidPartyInformation,
	/// Unknown instrument
	#[serde(rename = "2")]
	UnknownInstrument,
	/// Unauthorized to report trades
	#[serde(rename = "3")]
	UnauthorizedToReportTrades,
	/// Invalid trade type
	#[serde(rename = "4")]
	InvalidTradeType,
	/// Other
	#[serde(rename = "99")]
	Other,
	/// Price exceeds current price band
	#[serde(rename = "5")]
	PriceExceedsCurrentPriceBand,
	/// Reference price not available
	#[serde(rename = "6")]
	ReferencePriceNotAvailable,
	/// Notional value exceeds threshold
	#[serde(rename = "7")]
	NotionalValueExceedsThreshold,
}

impl Default for TradeReportRejectReason {
	fn default() -> Self {
		TradeReportRejectReason::Successful
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

impl Default for TradePriceNegotiationMethod {
	fn default() -> Self {
		TradePriceNegotiationMethod::PercentOfPar
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UpfrontPriceType {
	/// Percentage (i.e. percent of par) (often called "dollar price" for fixed income)
	#[serde(rename = "1")]
	Percentage,
	/// Fixed amount (absolute value)
	#[serde(rename = "3")]
	FixedAmount,
}

impl Default for UpfrontPriceType {
	fn default() -> Self {
		UpfrontPriceType::Percentage
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum OffsetInstruction {
	/// Offset
	#[serde(rename = "0")]
	Offset,
	/// Onset
	#[serde(rename = "1")]
	Onset,
}

impl Default for OffsetInstruction {
	fn default() -> Self {
		OffsetInstruction::Offset
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ClearedIndicator {
	/// Not cleared
	#[serde(rename = "0")]
	NotCleared,
	/// Cleared
	#[serde(rename = "1")]
	Cleared,
	/// Submitted
	#[serde(rename = "2")]
	Submitted,
	/// Rejected
	#[serde(rename = "3")]
	Rejected,
}

impl Default for ClearedIndicator {
	fn default() -> Self {
		ClearedIndicator::NotCleared
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ClearingIntention {
	/// Do not intend to clear
	#[serde(rename = "0")]
	DoNotIntendToClear,
	/// Intend to clear
	#[serde(rename = "1")]
	IntendToClear,
}

impl Default for ClearingIntention {
	fn default() -> Self {
		ClearingIntention::DoNotIntendToClear
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TradeClearingInstruction {
	/// Process normally
	#[serde(rename = "0")]
	ProcessNormally,
	/// Exclude from all netting
	#[serde(rename = "1")]
	ExcludeFromAllNetting,
	/// Bilateral netting only
	#[serde(rename = "2")]
	BilateralNettingOnly,
	/// Ex clearing
	#[serde(rename = "3")]
	ExClearing,
	/// Special trade
	#[serde(rename = "4")]
	SpecialTrade,
	/// Multilateral netting
	#[serde(rename = "5")]
	MultilateralNetting,
	/// Clear against central counterparty
	#[serde(rename = "6")]
	ClearAgainstCentralCounterparty,
	/// Exclude from central counterparty
	#[serde(rename = "7")]
	ExcludeFromCentralCounterparty,
	/// Manual mode (pre-posting and/or pre-giveup)
	#[serde(rename = "8")]
	ManualMode,
	/// Automatic posting mode (trade posting to the position account number specified)
	#[serde(rename = "9")]
	AutomaticPostingMode,
	/// Automatic give-up mode (trade give-up to the give-up destination number specified)
	#[serde(rename = "10")]
	AutomaticGiveUpMode,
	/// Qualified Service Representative QSR
	#[serde(rename = "11")]
	QualifiedServiceRepresentativeQsr,
	/// Customer trade
	#[serde(rename = "12")]
	CustomerTrade,
	/// Self clearing
	#[serde(rename = "13")]
	SelfClearing,
	/// Buy-in
	#[serde(rename = "14")]
	BuyIn,
}

impl Default for TradeClearingInstruction {
	fn default() -> Self {
		TradeClearingInstruction::ProcessNormally
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ConfirmationMethod {
	/// Non-electronic
	#[serde(rename = "0")]
	NonElectronic,
	/// Electronic
	#[serde(rename = "1")]
	Electronic,
	/// Unconfirmed
	#[serde(rename = "2")]
	Unconfirmed,
}

impl Default for ConfirmationMethod {
	fn default() -> Self {
		ConfirmationMethod::NonElectronic
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum VerificationMethod {
	/// Non-electronic
	#[serde(rename = "0")]
	NonElectronic,
	/// Electronic
	#[serde(rename = "1")]
	Electronic,
}

impl Default for VerificationMethod {
	fn default() -> Self {
		VerificationMethod::NonElectronic
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ClearingRequirementException {
	/// No exception
	#[serde(rename = "0")]
	NoException,
	/// Exception
	#[serde(rename = "1")]
	Exception,
	/// End-user exception (Elaboration: In the US, see CFTC Final Rule on End-User Exception to Clearing Requirements for Swaps Fact
	/// Sheet <a xmlns="http://www.b2bits.com/FIXProtocol" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" href="http://www.cftc.gov/ucm/groups/public/@newsroom/documents/file/eue_factsheet_final.pdf" target="_blank">http://www.cftc.gov/ucm/groups/public/@newsroom/documents/file/eue_factsheet_final.pdf</a> )
	#[serde(rename = "2")]
	EndUserException,
	/// Inter-affiliate exception (Elaboration: In the US, see CFTC Final Rule - Clearing Exemption for Swaps Between Certain Affiliated
	/// Entities <a xmlns="http://www.b2bits.com/FIXProtocol" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" href="http://www.cftc.gov//ucm/groups/public/@lrfederalregister/documents/file/2013-07970a.pdf" target="_blank">http://www.cftc.gov//ucm/groups/public/@lrfederalregister/documents/file/2013-07970a.pdf</a> )
	#[serde(rename = "3")]
	InterAffiliateException,
	/// Treasury affiliate exception (Elaboration: In the US, see CFTC NoAction Letter 13-22 No Action Relief from the Clearing Requirement
	/// for Swaps Entered into by Eligible Treasury Affiliates <a xmlns="http://www.b2bits.com/FIXProtocol" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" href="http://www.cftc.gov/ucm/groups/public/@lrlettergeneral/documents/letter/13-22.pdf" target="_blank">http://www.cftc.gov/ucm/groups/public/@lrlettergeneral/documents/letter/13-22.pdf</a> )
	#[serde(rename = "4")]
	TreasuryAffiliateException,
	/// Cooperative exception (Elaboration: Clearing exception for certain swaps entered into by cooperatives. In the US, see Regulation
	/// 50.51(a) Definition of Exempt Cooperative: <a xmlns="http://www.b2bits.com/FIXProtocol" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" href="https://www.federalregister.gov/articles/2013/08/22/2013-19945/clearingexemption-for-certain-swaps-enteredinto-by-cooperatives" target="_blank">https://www.federalregister.gov/articles/2013/08/22/2013-19945/clearingexemption-for-certain-swaps-enteredinto-by-cooperatives</a> )
	#[serde(rename = "5")]
	CooperativeExceptionDefinitionOfExemptCooperativeAXmlnsHttpWwwB2BitsComFixProtocolXmlnsXsiHttpWwwW3Org2001XmlSchemaInstanceHrefHttpsWwwFederalregisterGovArticles20130822201319945ClearingexemptionForCertainSwapsEnteredintoByCooperativesTargetBlankHttpsWwwFederalregisterGovArticles20130822201319945ClearingexemptionForCertainSwapsEnteredintoByCooperativesA,
}

impl Default for ClearingRequirementException {
	fn default() -> Self {
		ClearingRequirementException::NoException
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum IRSDirection {
	/// Principal is paying fixed rate
	#[serde(rename = "PAY")]
	PrincipalIsPayingFixedRate,
	/// Principal is receiving fixed rate
	#[serde(rename = "RCV")]
	PrincipalIsReceivingFixedRate,
	/// Swap is float/float or fixed/fixed
	#[serde(rename = "NA")]
	SwapIsFloatFloatOrFixedFixed,
}

impl Default for IRSDirection {
	fn default() -> Self {
		IRSDirection::PrincipalIsPayingFixedRate
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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
	CombinationOfRtPetAndConfirmation,
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
	PostTradeEventRtReportable0,
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

impl Default for RegulatoryReportType {
	fn default() -> Self {
		RegulatoryReportType::RealTime
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TradeCollateralization {
	/// Uncollateralized
	#[serde(rename = "0")]
	Uncollateralized,
	/// Partially collateralized
	#[serde(rename = "1")]
	PartiallyCollateralized,
	/// One-way collaterallization
	#[serde(rename = "2")]
	OneWayCollaterallization,
	/// Fully collateralized
	#[serde(rename = "3")]
	FullyCollateralized,
	/// Net exposure
	#[serde(rename = "4")]
	NetExposure,
}

impl Default for TradeCollateralization {
	fn default() -> Self {
		TradeCollateralization::Uncollateralized
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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
	AllocationTradePosting,
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

impl Default for TradeContinuation {
	fn default() -> Self {
		TradeContinuation::Novation
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum RiskLimitCheckStatus {
	/// Accepted
	#[serde(rename = "0")]
	Accepted,
	/// Rejected
	#[serde(rename = "1")]
	Rejected,
	/// Claim required
	#[serde(rename = "2")]
	ClaimRequired,
	/// Pre-defined limit check succeeded
	#[serde(rename = "3")]
	PreDefinedLimitCheckSucceeded,
	/// Pre-defined limit check failed
	#[serde(rename = "4")]
	PreDefinedLimitCheckFailed,
	/// Pre-defined auto-accept rule invoked
	#[serde(rename = "5")]
	PreDefinedAutoAcceptRuleInvoked,
	/// Pre-defined auto-reject rule invoked
	#[serde(rename = "6")]
	PreDefinedAutoRejectRuleInvoked,
	/// Accepted by clearing firm
	#[serde(rename = "7")]
	AcceptedByClearingFirm,
	/// Rejected by clearing firm
	#[serde(rename = "8")]
	RejectedByClearingFirm,
	/// Pending
	#[serde(rename = "9")]
	Pending,
	/// Accepted by credit hub (Indicates that a credit hub accepted the trade. An ID identifier assigned by the credit hub may appear
	/// in the appropriate RefRiskLimitCheckID(2334) field.)
	#[serde(rename = "10")]
	AcceptedByCreditHubField,
	/// Rejected by credit hub (Indicates that a credit hub rejected the trade.)
	#[serde(rename = "11")]
	RejectedByCreditHub,
	/// Pending credit hub check (Indicates that a check is pending at a credit hub.)
	#[serde(rename = "12")]
	PendingCreditHubCheck,
	/// Accepted by execution venue (Indicates acceptance by an execution venue, such as a SEF.)
	#[serde(rename = "13")]
	AcceptedByExecutionVenue,
	/// Rejected by execution venue (Indicates that the trade was rejected by an execution venue, such as a SEF.)
	#[serde(rename = "14")]
	RejectedByExecutionVenue,
}

impl Default for RiskLimitCheckStatus {
	fn default() -> Self {
		RiskLimitCheckStatus::Accepted
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum RegulatoryTransactionType {
	/// None (Default if not specified) (Elaboration: The transaction does not fall under any special regulatory rule or mandate.)
	#[serde(rename = "0")]
	None,
	/// Swap Execution Facility (SEF) Required Transaction (Elaboration: The transaction is a Required transaction under DoddFrank
	/// Act SEF Rules. Required transactions are subject to the trade execution mandate under section 2(h)(8) of the CEA and are not
	/// block trades)
	#[serde(rename = "1")]
	SwapExecutionFacilityRequiredTransactionOfTheCeaAndAreNotBlockTrades,
	/// Swap Execution Facility (SEF) Permitted Transaction (Elaboration: The transaction is a Permitted transaction under Dodd-Frank
	/// Act SEF Rules. Permitted transactions are not subject to the clearing and trade execution mandates, illiquid or bespoke swaps,
	/// or block trades)
	#[serde(rename = "2")]
	SwapExecutionFacilityPermittedTransaction,
}

impl Default for RegulatoryTransactionType {
	fn default() -> Self {
		RegulatoryTransactionType::None
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TaxonomyType {
	/// ISIN or Alternate instrument identifier plus CFI
	#[serde(rename = "I")]
	IsinOrAlternateInstrumentIdentifierPlusCfi,
	/// Interim taxonomy
	#[serde(rename = "E")]
	InterimTaxonomy,
}

impl Default for TaxonomyType {
	fn default() -> Self {
		TaxonomyType::IsinOrAlternateInstrumentIdentifierPlusCfi
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SettlPriceFxRateCalc {
	/// Multiply
	#[serde(rename = "M")]
	Multiply,
	/// Divide
	#[serde(rename = "D")]
	Divide,
}

impl Default for SettlPriceFxRateCalc {
	fn default() -> Self {
		SettlPriceFxRateCalc::Multiply
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum IntraFirmTradeIndicator {
	/// Trade or position is not an intra-firm transaction
	#[serde(rename = "N")]
	TradeOrPositionIsNotAnIntraFirmTransaction,
	/// Trade or position is an intra-firm transaction
	#[serde(rename = "Y")]
	TradeOrPositionIsAnIntraFirmTransaction,
}

impl Default for IntraFirmTradeIndicator {
	fn default() -> Self {
		IntraFirmTradeIndicator::TradeOrPositionIsNotAnIntraFirmTransaction
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ExecMethod {
	/// Undefined/unspecified - (default when not specified)
	#[serde(rename = "0")]
	UndefinedUnspecified,
	/// Manual (The transaction was executed in a manual or other non-automated manner e.g. by voice directly between the counterparties.
	/// Also used to identify MTT code M "Off Book Non-Automated".)
	#[serde(rename = "1")]
	Manual,
	/// Automated (The transaction was executed on an automated execution platform such as an automated systematic internaliser system,
	/// broker crossing network, broker crossing system, dark pool trading, "direct to capital" systems, broker position unwind mechanisms,
	/// etc.)
	#[serde(rename = "2")]
	Automated,
	/// Voice brokered
	#[serde(rename = "3")]
	VoiceBrokered,
}

impl Default for ExecMethod {
	fn default() -> Self {
		ExecMethod::UndefinedUnspecified
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TradeContingency {
	/// Does not apply (default if not specified) (The trade is for an asset classes that areis not traded with contingency)
	#[serde(rename = "0")]
	DoesNotApply,
	/// Contingent trade (The trade is terminated as soon as its paired trade is cleared or denied clearing)
	#[serde(rename = "1")]
	ContingentTrade,
	/// Non-contingent trade (Identifies a trade that is not contingent but is for an asset class that may be contingent)
	#[serde(rename = "2")]
	NonContingentTrade,
}

impl Default for TradeContingency {
	fn default() -> Self {
		TradeContingency::DoesNotApply
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MultiAssetSwapIndicator {
	/// Not multi-asset swap
	#[serde(rename = "N")]
	NotMultiAssetSwap,
	/// Multi-asset swap
	#[serde(rename = "Y")]
	MultiAssetSwap,
}

impl Default for MultiAssetSwapIndicator {
	fn default() -> Self {
		MultiAssetSwapIndicator::NotMultiAssetSwap
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum InternationalSwapIndicator {
	/// Not international swap
	#[serde(rename = "N")]
	NotInternationalSwap,
	/// International swap
	#[serde(rename = "Y")]
	InternationalSwap,
}

impl Default for InternationalSwapIndicator {
	fn default() -> Self {
		InternationalSwapIndicator::NotInternationalSwap
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum AffiliatedFirmsTradeIndicator {
	/// Transaction or position is not between two affiliated firms
	#[serde(rename = "N")]
	TransactionOrPositionIsNotBetweenTwoAffiliatedFirms,
	/// Transaction or position is between two affiliated firms
	#[serde(rename = "Y")]
	TransactionOrPositionIsBetweenTwoAffiliatedFirms,
}

impl Default for AffiliatedFirmsTradeIndicator {
	fn default() -> Self {
		AffiliatedFirmsTradeIndicator::TransactionOrPositionIsNotBetweenTwoAffiliatedFirms
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum AlgorithmicTradeIndicator {
	/// Non-algorithmic trade
	#[serde(rename = "0")]
	NonAlgorithmicTrade,
	/// Algorithmic trade
	#[serde(rename = "1")]
	AlgorithmicTrade,
}

impl Default for AlgorithmicTradeIndicator {
	fn default() -> Self {
		AlgorithmicTradeIndicator::NonAlgorithmicTrade
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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
	/// Trade has been or will be reported under a sub-delegation arrangement by an investment firm to a reporting facility (e.g.
	/// APA) on behalf of another investment firm
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

impl Default for TradeReportingIndicator {
	fn default() -> Self {
		TradeReportingIndicator::TradeHasNotBeenReported
	}
}
