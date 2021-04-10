
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Contrary {
	/// MsgType = BO
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// ApplicationSequenceControl
	#[serde(flatten)]
	pub application_sequence_control: Option<super::super::application_sequence_control::ApplicationSequenceControl>,
	/// Unique identifier for the Contrary Intention report
	#[serde(rename = "977")]
	pub cont_int_rpt_id: String,
	/// Time the contrary intention was received by clearing organization.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<fix_common::UTCTimestamp>,
	/// Indicates if the contrary intention was received after the exchange imposed cutoff time
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "978")]
	pub late_indicator: Option<fix_common::Boolean>,
	/// Source of the contrary intention
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "979")]
	pub input_source: Option<String>,
	/// Business date of contrary intention
	#[serde(rename = "715")]
	pub clearing_business_date: fix_common::LocalMktDate,
	/// Clearing Organization Clearing Firm Position Account
	#[serde(flatten)]
	pub parties: super::super::parties::Parties,
	/// Expiration Quantities
	#[serde(flatten)]
	pub expiration_qty: super::super::expiration_qty::ExpirationQty,
	/// Instrument
	#[serde(flatten)]
	pub instrument: super::super::instrument::Instrument,
	/// UndInstrmtGrp
	#[serde(flatten)]
	pub und_instrmt_grp: Option<super::super::und_instrmt_grp::UndInstrmtGrp>,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if <a href="tag_355_EncodedText.html" target="bottom">EncodedText&nbsp;(355)</a> field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "354")]
	pub encoded_text_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the <a href="tag_58_Text.html" target="bottom">Text&nbsp;(58)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "355")]
	pub encoded_text: Option<String>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}
