
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct QuoteStatusReport {
	/// MsgType = AI
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'A', 'I'>,
	/// QuoteStatusReqID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "649")]
	pub quote_status_req_id: Option<String>,
	/// Required when quote is in response to a <a href="message_Quote_Request_R.html" target="main">Quote Request&nbsp;(R)</a> message
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "131")]
	pub quote_req_id: Option<String>,
	/// <p>Contains <a href="tag_117_QuoteID.html" target="bottom">QuoteID&nbsp;(117)</a> of a single <a href="message_Quote_S.html" target="main">Quote&nbsp;(S)</a> or <a href="tag_299_QuoteEntryID.html" target="bottom">QuoteEntryID&nbsp;(299)</a> of a <a href="message_Mass_Quote_i.html" target="main">Mass Quote&nbsp;(i)</a> .
	/// </p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "117")]
	pub quote_id: Option<String>,
	/// Contains the <a href="tag_1867_OfferID.html" target="bottom">OfferID&nbsp;(1867)</a> of a single <a href="message_Quote_S.html" target="main">Quote&nbsp;(S)</a> .
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1166")]
	pub quote_msg_id: Option<String>,
	/// Required when responding to a <a href="message_QuoteResponse_AJ.html" target="main">Quote Response&nbsp;(AJ)</a> message.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "693")]
	pub quote_resp_id: Option<String>,
	/// If not specified, the default is an indicative quote
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "537")]
	pub quote_type: Option<QuoteType>,
	/// QuoteCancelType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "298")]
	pub quote_cancel_type: Option<QuoteCancelType>,
	/// Parties
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// Can be populated with the values provided on the associated <a href="message_Quote_Status_Request_a.html" target="main">QuoteStatusRequest&nbsp;(a)</a> .
	#[serde(flatten)]
	pub target_parties: Option<super::super::target_parties::TargetParties>,
	/// TradingSessionID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "336")]
	pub trading_session_id: Option<TradingSessionID>,
	/// TradingSessionSubID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "625")]
	pub trading_session_sub_id: Option<TradingSessionSubID>,
	/// Conditionally required when reporting status of a single security quote.
	#[serde(flatten)]
	pub instrument: Option<super::super::instrument::Instrument>,
	/// FinancingDetails
	#[serde(flatten)]
	pub financing_details: Option<super::super::financing_details::FinancingDetails>,
	/// UndInstrmtGrp
	#[serde(flatten)]
	pub und_instrmt_grp: Option<super::super::und_instrmt_grp::UndInstrmtGrp>,
	/// Side
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "54")]
	pub side: Option<Side>,
	/// Conditionally required for quotes of single instrument depending on the type of instrument when QuoteType(537)=1 (Tradeable).
	#[serde(flatten)]
	pub order_qty_data: Option<super::super::order_qty_data::OrderQtyData>,
	/// SettlType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "63")]
	pub settl_type: Option<SettlType>,
	/// Can be used with forex quotes to specify a specific "value date".
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "64")]
	pub settl_date: Option<fix_common::LocalMktDate>,
	/// (Deprecated in FIX.5.0) Can be used with <a href="tag_40_OrdType.html" target="bottom">OrdType&nbsp;(40)</a> = "Forex - Swap" to specify the "value date" for the future portion of a F/X swap.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "193")]
	pub settl_date_2: Option<fix_common::LocalMktDate>,
	/// (Deprecated in FIX.5.0) Can be used with <a href="tag_40_OrdType.html" target="bottom">OrdType&nbsp;(40)</a> = "Forex - Swap" to specify the order quantity for the future portion of a F/X swap.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "192")]
	pub order_qty_2: Option<f64>,
	/// Can be used to specify the currency of the quoted prices. May differ from the normal trading currency of the instrument being
	/// quoted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "15")]
	pub currency: Option<Currency>,
	/// Stipulations
	#[serde(flatten)]
	pub stipulations: Option<super::super::stipulations::Stipulations>,
	/// Account
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1")]
	pub account: Option<String>,
	/// AcctIDSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "660")]
	pub acct_id_source: Option<AcctIDSource>,
	/// AccountType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "581")]
	pub account_type: Option<AccountType>,
	/// Conditionally required for multileg quote status reports
	#[serde(flatten)]
	pub leg_quot_stat_grp: Option<super::super::leg_quot_stat_grp::LegQuotStatGrp>,
	/// QuotQualGrp
	#[serde(flatten)]
	pub quot_qual_grp: Option<super::super::quot_qual_grp::QuotQualGrp>,
	/// ExpireTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "126")]
	pub expire_time: Option<fix_common::UTCTimestamp>,
	/// Price
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "44")]
	pub price: Option<f64>,
	/// PriceType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "423")]
	pub price_type: Option<PriceType>,
	/// SpreadOrBenchmarkCurveData
	#[serde(flatten)]
	pub spread_or_benchmark_curve_data: Option<super::super::spread_or_benchmark_curve_data::SpreadOrBenchmarkCurveData>,
	/// YieldData
	#[serde(flatten)]
	pub yield_data: Option<super::super::yield_data::YieldData>,
	/// If F/X quote, should be the "all-in" rate (spot rate adjusted for forward points). Note that either BidPx, <a href="tag_133_OfferPx.html" target="bottom">OfferPx&nbsp;(133)</a> or both must be specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "132")]
	pub bid_px: Option<f64>,
	/// If F/X quote, should be the "all-in" rate (spot rate adjusted for forward points). Note that either BidPx, <a href="tag_133_OfferPx.html" target="bottom">OfferPx&nbsp;(133)</a> or both must be specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "133")]
	pub offer_px: Option<f64>,
	/// Can be used by markets that require showing the current best bid and offer
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "645")]
	pub mkt_bid_px: Option<f64>,
	/// Can be used by markets that require showing the current best bid and offer
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "646")]
	pub mkt_offer_px: Option<f64>,
	/// Used for markets that use a minimum and maximum bid size.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "647")]
	pub min_bid_size: Option<f64>,
	/// If <a href="tag_647_MinBidSize.html" target="bottom">MinBidSize&nbsp;(647)</a> is specified, <a href="tag_134_BidSize.html" target="bottom">BidSize&nbsp;(134)</a> is interpreted to contain the maximum bid size.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "134")]
	pub bid_size: Option<f64>,
	/// If <a href="tag_648_MinOfferSize.html" target="bottom">MinOfferSize&nbsp;(648)</a> is specified, <a href="tag_135_OfferSize.html" target="bottom">OfferSize&nbsp;(135)</a> is interpreted to contain the maximum offer size.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "648")]
	pub min_offer_size: Option<f64>,
	/// If <a href="tag_648_MinOfferSize.html" target="bottom">MinOfferSize&nbsp;(648)</a> is specified, <a href="tag_135_OfferSize.html" target="bottom">OfferSize&nbsp;(135)</a> is interpreted to contain the maximum offer size.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "135")]
	pub offer_size: Option<f64>,
	/// MinQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "110")]
	pub min_qty: Option<f64>,
	/// ValidUntilTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "62")]
	pub valid_until_time: Option<fix_common::UTCTimestamp>,
	/// BidSpotRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "188")]
	pub bid_spot_rate: Option<f64>,
	/// OfferSpotRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "190")]
	pub offer_spot_rate: Option<f64>,
	/// BidForwardPoints
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "189")]
	pub bid_forward_points: Option<f64>,
	/// OfferForwardPoints
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "191")]
	pub offer_forward_points: Option<f64>,
	/// MidPx
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "631")]
	pub mid_px: Option<f64>,
	/// BidYield
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "632")]
	pub bid_yield: Option<f32>,
	/// MidYield
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "633")]
	pub mid_yield: Option<f32>,
	/// OfferYield
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "634")]
	pub offer_yield: Option<f32>,
	/// TransactTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<fix_common::UTCTimestamp>,
	/// Can be used to specify the type of order the quote is for
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40")]
	pub ord_type: Option<OrdType>,
	/// (Deprecated in FIX.5.0) Bid F/X forward points of the future portion of a F/X swap quote added to spot rate. May be a negative
	/// value
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "642")]
	pub bid_forward_points_2: Option<f64>,
	/// (Deprecated in FIX.5.0)Offer F/X forward points of the future portion of a F/X swap quote added to spot rate. May be a negative
	/// value
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "643")]
	pub offer_forward_points_2: Option<f64>,
	/// Can be used when the quote is provided in a currency other than the instruments normal trading currency. Applies to all bid
	/// prices contained in this message
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "656")]
	pub settl_curr_bid_fx_rate: Option<f64>,
	/// Can be used when the quote is provided in a currency other than the instruments normal trading currency. Applies to all offer
	/// prices contained in this message
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "657")]
	pub settl_curr_offer_fx_rate: Option<f64>,
	/// Can be used when the quote is provided in a currency other than the instruments trading currency.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "156")]
	pub settl_curr_fx_rate_calc: Option<SettlCurrFxRateCalc>,
	/// CustOrderCapacity
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "582")]
	pub cust_order_capacity: Option<CustOrderCapacity>,
	/// Used when routing quotes to multiple markets
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "100")]
	pub ex_destination: Option<String>,
	/// ExDestinationIDSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1133")]
	pub ex_destination_id_source: Option<ExDestinationIDSource>,
	/// BookingType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "775")]
	pub booking_type: Option<BookingType>,
	/// OrderCapacity
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "528")]
	pub order_capacity: Option<OrderCapacity>,
	/// OrderRestrictions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "529")]
	pub order_restrictions: Option<fix_common::SeparatedValues<OrderRestrictions>>,
	/// QuoteStatus
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "297")]
	pub quote_status: Option<QuoteStatus>,
	/// QuoteRejectReason
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "300")]
	pub quote_reject_reason: Option<QuoteRejectReason>,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// EncodedTextLen
	#[serde(rename = "354")]
	/// EncodedText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
	/// ThrottleResponse
	#[serde(flatten)]
	pub throttle_response: Option<super::super::throttle_response::ThrottleResponse>,
	/// BidQuoteID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1747")]
	pub bid_quote_id: Option<String>,
	/// OfferQuoteID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1748")]
	pub offer_quote_id: Option<String>,
	/// BidMDEntryID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1745")]
	pub bid_md_entry_id: Option<String>,
	/// OfferMDEntryID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1746")]
	pub offer_md_entry_id: Option<String>,
	/// SecondaryQuoteID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1751")]
	pub secondary_quote_id: Option<String>,
	/// TotalBidSize
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1749")]
	pub total_bid_size: Option<f64>,
	/// TotalOfferSize
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1750")]
	pub total_offer_size: Option<f64>,
	/// Reason description for rejecting the quote
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1328")]
	pub reject_text: Option<String>,
	/// Must be set if <a href="tag_1665_EncodedRejectText.html" target="bottom">EncodedRejectTextLen(1665)&nbsp;(1665)</a> field is specified and must immediately precede it.
	#[serde(rename = "1664")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_1328_RejectText.html" target="bottom">RejectedText(1328)&nbsp;(1328)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding(347)&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "1665")]
	pub encoded_reject_text: Option<fix_common::EncodedText<1665>>,
	/// BidID(390) of a single Quote(35=S).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "390")]
	pub bid_id: Option<String>,
	/// OfferID(1867) of a single Quote(35=S).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1867")]
	pub offer_id: Option<String>,
	/// NegotiationMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2115")]
	pub negotiation_method: Option<NegotiationMethod>,
	/// Can be used to show the counterparty the commission associated with the transaction.
	#[serde(flatten)]
	pub commission_data: Option<super::super::commission_data::CommissionData>,
	/// Conditionally required when QuoteQual(695) = d (Deferred spot) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "443")]
	pub strike_time: Option<fix_common::UTCTimestamp>,
	/// PriceQualifierGrp
	#[serde(flatten)]
	pub price_qualifier_grp: Option<super::super::price_qualifier_grp::PriceQualifierGrp>,
	/// EventInitiatorType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2830")]
	pub event_initiator_type: Option<EventInitiatorType>,
	/// TerminationDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2878")]
	pub termination_date: Option<fix_common::LocalMktDate>,
	/// If specified, this should echo the value in the message this status message is in response to.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1937")]
	pub trade_continuation: Option<TradeContinuation>,
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
	/// RegulatoryReportType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1934")]
	pub regulatory_report_type: Option<RegulatoryReportType>,
	/// QuoteAttributeGrp
	#[serde(flatten)]
	pub quote_attribute_grp: Option<super::super::quote_attribute_grp::QuoteAttributeGrp>,
	/// TrdRegTimestamps
	#[serde(flatten)]
	pub trd_reg_timestamps: Option<super::super::trd_reg_timestamps::TrdRegTimestamps>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum QuoteType {
	/// Indicative
	#[serde(rename = "0")]
	Indicative,
	/// Tradeable
	#[serde(rename = "1")]
	Tradeable,
	/// Restricted Tradeable
	#[serde(rename = "2")]
	RestrictedTradeable,
	/// Counter (tradeable)
	#[serde(rename = "3")]
	Counter,
	/// Initially tradeable
	#[serde(rename = "4")]
	InitiallyTradeable,
}

impl Default for QuoteType {
	fn default() -> Self {
		QuoteType::Indicative
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum QuoteCancelType {
	/// Cancel for Symbol(s)
	#[serde(rename = "1")]
	CancelForSymbol,
	/// Cancel for Security Type(s)
	#[serde(rename = "2")]
	CancelForSecurityType,
	/// Cancel for Underlying Symbol
	#[serde(rename = "3")]
	CancelForUnderlyingSymbol,
	/// Cancel All Quotes
	#[serde(rename = "4")]
	CancelAllQuotes,
	/// Cancel quote specified in QuoteID
	#[serde(rename = "5")]
	CancelQuoteSpecifiedInQuoteId,
	/// Cancel by QuoteType(537)
	#[serde(rename = "6")]
	CancelByQuoteType,
	/// Cancel for Security Issuer
	#[serde(rename = "7")]
	CancelForSecurityIssuer,
	/// Cancel for Issuer of Underlying Security
	#[serde(rename = "8")]
	CancelForIssuerOfUnderlyingSecurity,
}

impl Default for QuoteCancelType {
	fn default() -> Self {
		QuoteCancelType::CancelForSymbol
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
	/// Holiday
	#[serde(rename = "7")]
	Holiday,
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

impl Default for TradingSessionSubID {
	fn default() -> Self {
		TradingSessionSubID::PreTrading
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
	/// Sell undisclosed
	#[serde(rename = "H")]
	SellUndisclosed,
}

impl Default for Side {
	fn default() -> Self {
		Side::Buy
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

impl Default for AccountType {
	fn default() -> Self {
		AccountType::AccountIsCarriedOnCustomerSideOfBooks
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
	StopOnBidOrOfferAtWhichPointTheStoppedOrderBecomesAMarketOrderAlsoKnownAsStopOnQuoteInSomeMarketsInTheUsEquitiesMarketItIsCommonToTriggerAStopOffTheNationalBestBidOrOffer,
	/// Stop Limit on Bid or Offer (A stop order that is triggered by a bid or offer price movement (quote) at which ponit the stopped
	/// order becomes a limit order, also known as "stop limit on quote" in some markets (e.g. US markets). In the US equities market
	/// it is common to trigger a stop off the National Best Bid or Offer (NBBO)
	#[serde(rename = "S")]
	StopLimitOnBidOrOfferAtWhichPonitTheStoppedOrderBecomesALimitOrderAlsoKnownAsStopLimitOnQuoteInSomeMarketsInTheUsEquitiesMarketItIsCommonToTriggerAStopOffTheNationalBestBidOrOffer,
}

impl Default for OrdType {
	fn default() -> Self {
		OrdType::Market
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

impl Default for CustOrderCapacity {
	fn default() -> Self {
		CustOrderCapacity::MemberTradingForTheirOwnAccount
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ExDestinationIDSource {
	/// BIC (Bank Identification Code) (ISO 9362)
	#[serde(rename = "B")]
	Bic,
	/// Generally accepted market participant identifier (e.g. NASD mnemonic)
	#[serde(rename = "C")]
	GenerallyAcceptedMarketParticipantIdentifier,
	/// Proprietary / Custom code
	#[serde(rename = "D")]
	ProprietaryCustomCode,
	/// ISO Country Code
	#[serde(rename = "E")]
	IsoCountryCode,
	/// MIC (ISO 10383 - Market Identifier Code)
	#[serde(rename = "G")]
	Mic,
}

impl Default for ExDestinationIDSource {
	fn default() -> Self {
		ExDestinationIDSource::Bic
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

impl Default for OrderRestrictions {
	fn default() -> Self {
		OrderRestrictions::ProgramTrade
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum QuoteStatus {
	/// Accepted
	#[serde(rename = "0")]
	Accepted,
	/// Canceled for Symbol(s)
	#[serde(rename = "1")]
	CanceledForSymbol,
	/// Canceled for Security Type(s)
	#[serde(rename = "2")]
	CanceledForSecurityType,
	/// Canceled for Underlying
	#[serde(rename = "3")]
	CanceledForUnderlying,
	/// Canceled All
	#[serde(rename = "4")]
	CanceledAll,
	/// Rejected
	#[serde(rename = "5")]
	Rejected,
	/// Removed from Market
	#[serde(rename = "6")]
	RemovedFromMarket,
	/// Expired
	#[serde(rename = "7")]
	Expired,
	/// Query
	#[serde(rename = "8")]
	Query,
	/// Quote Not Found
	#[serde(rename = "9")]
	QuoteNotFound,
	/// Pending
	#[serde(rename = "10")]
	Pending,
	/// Pass
	#[serde(rename = "11")]
	Pass,
	/// Locked Market Warning
	#[serde(rename = "12")]
	LockedMarketWarning,
	/// Cross Market Warning
	#[serde(rename = "13")]
	CrossMarketWarning,
	/// Canceled Due To Lock Market
	#[serde(rename = "14")]
	CanceledDueToLockMarket,
	/// Canceled due to crossed market
	#[serde(rename = "15")]
	CanceledDueToCrossedMarket,
	/// Active
	#[serde(rename = "16")]
	Active,
	/// Canceled
	#[serde(rename = "17")]
	Canceled,
	/// Unsolicited Quote Replenishment
	#[serde(rename = "18")]
	UnsolicitedQuoteReplenishment,
	/// Pending End Trade
	#[serde(rename = "19")]
	PendingEndTrade,
	/// Too Late to End
	#[serde(rename = "20")]
	TooLateToEnd,
	/// Traded
	#[serde(rename = "21")]
	Traded,
	/// Traded and removed
	#[serde(rename = "22")]
	TradedAndRemoved,
	/// Contract terminated
	#[serde(rename = "23")]
	ContractTerminated,
}

impl Default for QuoteStatus {
	fn default() -> Self {
		QuoteStatus::Accepted
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum QuoteRejectReason {
	/// Unknown symbol (security)
	#[serde(rename = "1")]
	N1,
	/// Exhcnage (security) closed
	#[serde(rename = "2")]
	N2,
	/// Quote exceeds limit
	#[serde(rename = "3")]
	N3,
	/// Too late to enter
	#[serde(rename = "4")]
	N4,
	/// Unknown quote
	#[serde(rename = "5")]
	N5,
	/// Duplicate quote
	#[serde(rename = "6")]
	N6,
	/// Invalid bid/ask spread
	#[serde(rename = "7")]
	N7,
	/// Invalid price
	#[serde(rename = "8")]
	N8,
	/// Not authorized to quote security
	#[serde(rename = "9")]
	N9,
	/// Price exceeds current price band
	#[serde(rename = "10")]
	N10,
	/// Quote Locked - Unable to Update/Cancel
	#[serde(rename = "11")]
	N11,
	/// Invalid or unknown Security Issuer
	#[serde(rename = "12")]
	N12,
	/// Invalid or unknown Issuer of Underlying Security
	#[serde(rename = "13")]
	N13,
	/// Other
	#[serde(rename = "99")]
	N99,
	/// Notional value exceeds threshold
	#[serde(rename = "14")]
	N14,
	/// Price exceeds current price band
	#[serde(rename = "15")]
	N15,
	/// Reference price not available
	#[serde(rename = "16")]
	N16,
	/// Insufficient credit limit
	#[serde(rename = "17")]
	N17,
	/// Exceeded clip size limit
	#[serde(rename = "18")]
	N18,
	/// Exceeded maximum notional order amount
	#[serde(rename = "19")]
	N19,
	/// Exceeded DV01/PV01 limit
	#[serde(rename = "20")]
	N20,
	/// Exceeded CS01 limit
	#[serde(rename = "21")]
	N21,
}

impl Default for QuoteRejectReason {
	fn default() -> Self {
		QuoteRejectReason::N1
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum NegotiationMethod {
	/// Auto spot
	#[serde(rename = "0")]
	AutoSpot,
	/// Negotiated spot
	#[serde(rename = "1")]
	NegotiatedSpot,
	/// The spot price for the reference or benchmark security is to be negotiated via phone or voice.
	#[serde(rename = "2")]
	TheSpotPriceForTheReferenceOrBenchmarkSecurityIsToBeNegotiatedViaPhoneOrVoice,
}

impl Default for NegotiationMethod {
	fn default() -> Self {
		NegotiationMethod::AutoSpot
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

impl Default for EventInitiatorType {
	fn default() -> Self {
		EventInitiatorType::CustomerOrClient
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
