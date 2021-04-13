
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TradeAggregationReport {
	/// MsgType = DX
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// Unique identifier for the report message.
	#[serde(rename = "2792")]
	pub trade_aggregation_report_id: String,
	/// TradeAggregationRequest(35=DW) message being responded to.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2786")]
	pub trade_aggregation_request_id: Option<String>,
	/// TradeAggregationRequestStatus
	#[serde(rename = "2790")]
	pub trade_aggregation_request_status: TradeAggregationRequestStatus,
	/// Conditionally required when TradeAggregationRequestStatus(2790)=0 (Accepted). The trade identifier for the group of aggregated
	/// trades. This would be used by the sell-side to provide an aggregated trade identifier if the request was accepted.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1003")]
	pub trade_id: Option<String>,
	/// TradeAggregationRejectReason
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2791")]
	pub trade_aggregation_reject_reason: Option<TradeAggregationRejectReason>,
	/// Conditionally required when TradeAggregationRequestStatus(2790)=0 (Accepted).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2789")]
	pub aggregated_qty: Option<f64>,
	/// Conditionally required when TradeAggregationRequestStatus(2790)=0 (Accepted). The all-in average rate for the aggregated trades.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "6")]
	pub avg_px: Option<f64>,
	/// AvgSpotRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2793")]
	pub avg_spot_rate: Option<f64>,
	/// AvgForwardPoints
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2794")]
	pub avg_forward_points: Option<f64>,
	/// SettlDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "64")]
	pub settl_date: Option<fix_common::LocalMktDate>,
	/// Conditionally required when TradeAggregationRequestStatus(2790)=0 (Accepted).
	#[serde(flatten)]
	pub instrument: Option<super::super::instrument::Instrument>,
	/// Conditionally required when TradeAggregationRequestStatus(2790)=0 (Accepted).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "54")]
	pub side: Option<Side>,
	/// Optionally used to provide a text narrative for rejecting the request.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1328")]
	pub reject_text: Option<String>,
	/// Must be set if EncodedRejectText(1665) field is specified and must immediately precede it.
	#[serde(rename = "1664")]
	/// Encoded (non-ASCII characters) representation of the RejectText(1328) field in the encoded format specified via the MessageEncoding(347)
	/// field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "1665")]
	pub encoded_reject_text: Option<fix_common::EncodedText<1665>>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TradeAggregationRequestStatus {
	/// Accepted
	#[serde(rename = "0")]
	Accepted,
	/// Rejected
	#[serde(rename = "1")]
	Rejected,
}

impl Default for TradeAggregationRequestStatus {
	fn default() -> Self {
		TradeAggregationRequestStatus::Accepted
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TradeAggregationRejectReason {
	/// Unknown order(s)
	#[serde(rename = "0")]
	UnknownOrder,
	/// Unknown execution/fill(s)
	#[serde(rename = "1")]
	UnknownExecutionFill,
	/// Other
	#[serde(rename = "99")]
	Other,
}

impl Default for TradeAggregationRejectReason {
	fn default() -> Self {
		TradeAggregationRejectReason::UnknownOrder
	}
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
