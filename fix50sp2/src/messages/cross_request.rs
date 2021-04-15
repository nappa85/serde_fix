
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CrossRequest {
	/// MsgType = DS
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'D', 'S'>,
	/// Unique identifier for cross request message.
	#[serde(rename = "2672")]
	pub cross_request_id: String,
	/// MarketID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1301")]
	pub market_id: Option<String>,
	/// MarketSegmentID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1300")]
	pub market_segment_id: Option<String>,
	/// Instrument
	#[serde(flatten)]
	pub instrument: super::super::instrument::Instrument,
	/// Can be used to announce a maximum quantity that is subject to crossing.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "38")]
	pub order_qty: Option<f64>,
	/// ComplianceID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "376")]
	pub compliance_id: Option<String>,
	/// ComplianceText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2404")]
	pub compliance_text: Option<String>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}
