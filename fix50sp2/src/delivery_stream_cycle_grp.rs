
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeliveryStreamCycleGrp {
	/// NoDeliveryStreamCycles
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41081")]
	pub delivery_stream_cycles: Option<crate::entities::RepeatingValues<DeliveryStreamCycle>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeliveryStreamCycle {
	/// Required if NoDeliveryStreamCycles(41081) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41082")]
	pub delivery_stream_cycle_desc: Option<String>,
	/// Must be set if EncodedDeliveryStreamCycleDesc(41084) field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41083")]
	pub encoded_delivery_stream_cycle_desc_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the DeliveryStreamCycleDesc(41082) field in the encoded format specified
	/// via the MessageEncoding(347) field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41084")]
	pub encoded_delivery_stream_cycle_desc: Option<String>,
}
