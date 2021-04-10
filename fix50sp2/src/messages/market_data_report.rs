
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Market {
	/// MsgType = DR
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// ApplicationSequenceControl
	#[serde(flatten)]
	pub application_sequence_control: Option<super::super::application_sequence_control::ApplicationSequenceControl>,
	/// Unique identifier for MarketDataReport(35=DR).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "963")]
	pub md_report_id: Option<i32>,
	/// MDReportEvent
	#[serde(rename = "2535")]
	pub md_report_event: MDReportEvent,
	/// MDReportCount
	#[serde(deserialize_with = "crate::entities::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "2536")]
	pub md_report_count: i32,
	/// TransactTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<crate::entities::UTCTimestamp>,
	/// TotNumReports
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "911")]
	pub tot_num_reports: Option<i32>,
	/// TotNoMarketSegmentReports
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2537")]
	pub tot_no_market_segment_reports: Option<i32>,
	/// TotNoInstrumentReports
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2538")]
	pub tot_no_instrument_reports: Option<i32>,
	/// TotNoPartyDetailReports
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2539")]
	pub tot_no_party_detail_reports: Option<i32>,
	/// TotNoEntitlementReports
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2540")]
	pub tot_no_entitlement_reports: Option<i32>,
	/// TotNoRiskLimitReports
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2541")]
	pub tot_no_risk_limit_reports: Option<i32>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MDReportEvent {
	/// Start of instrument reference data
	#[serde(rename = "1")]
	StartOfInstrumentReferenceData,
	/// End of instrument reference data
	#[serde(rename = "2")]
	EndOfInstrumentReferenceData,
	/// Start of off-market trades
	#[serde(rename = "3")]
	StartOfOffMarketTrades,
	/// End of off-market trades
	#[serde(rename = "4")]
	EndOfOffMarketTrades,
	/// Start of order book trades
	#[serde(rename = "5")]
	StartOfOrderBookTrades,
	/// End of order book trades
	#[serde(rename = "6")]
	EndOfOrderBookTrades,
	/// Start of open interest
	#[serde(rename = "7")]
	StartOfOpenInterest,
	/// End of open interest
	#[serde(rename = "8")]
	EndOfOpenInterest,
	/// Start of settlement prices
	#[serde(rename = "9")]
	StartOfSettlementPrices,
	/// End of settlement prices
	#[serde(rename = "10")]
	EndOfSettlementPrices,
	/// Start of statistics reference data
	#[serde(rename = "11")]
	StartOfStatisticsReferenceData,
	/// End of statistics reference data
	#[serde(rename = "12")]
	EndOfStatisticsReferenceData,
	/// Start of statistics
	#[serde(rename = "13")]
	StartOfStatistics,
	/// End of statistics
	#[serde(rename = "14")]
	EndOfStatistics,
}
