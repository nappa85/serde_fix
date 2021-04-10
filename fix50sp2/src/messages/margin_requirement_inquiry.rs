
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Margin {
	/// MsgType = CH
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// Unique identifier for this message
	#[serde(rename = "1635")]
	pub margin_reqmt_inq_id: String,
	/// Type of margin requirement inquiry
	#[serde(flatten)]
	pub margin_reqmt_inq_qual_grp: super::super::margin_reqmt_inq_qual_grp::MarginReqmtInqQualGrp,
	/// Used to subscribe/unsubscribe for margin requirement reports. If the field is absent, the default will be snapshot request
	/// only - no subscription
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "263")]
	pub subscription_request_type: Option<SubscriptionRequestType>,
	/// Ability to specify whether the response to the request should be delivered inband or via pre-arranged out-ofband transport
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "725")]
	pub response_transport_type: Option<ResponseTransportType>,
	/// URI destination name. Used if ResponseTransportType is out-of-band.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "726")]
	pub response_destination: Option<String>,
	/// Parties
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// Indicates the date for which the margin is to be calculated
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "715")]
	pub clearing_business_date: Option<fix_common::LocalMktDate>,
	/// Indicates the settlement session for which the margin is to be calculated - End Of Day or Intraday
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "716")]
	pub settl_sess_id: Option<SettlSessID>,
	/// SettlSessSubID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "717")]
	pub settl_sess_sub_id: Option<String>,
	/// Used to identify a group of instruments with similar risk profile
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1639")]
	pub margin_class: Option<String>,
	/// Instrument
	#[serde(flatten)]
	pub instrument: Option<super::super::instrument::Instrument>,
	/// Represents the time the inquiry was submitted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<fix_common::UTCTimestamp>,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if EncodedText(355) field is specified and must immediately precede it
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "354")]
	pub encoded_text_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "355")]
	pub encoded_text: Option<String>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum ResponseTransportType {
	/// In-band (default)
	#[serde(rename = "0")]
	InBand,
	/// Out of band
	#[serde(rename = "1")]
	OutOfBand,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
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
