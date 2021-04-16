
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeliveryStreamCycleGrp {
	/// NoDeliveryStreamCycles
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41081")]
	pub delivery_stream_cycles: Option<fix_common::RepeatingValues<DeliveryStreamCycle>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeliveryStreamCycle {
	/// Required if NoDeliveryStreamCycles(41081) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41082")]
	pub delivery_stream_cycle_desc: Option<String>,
	/// Must be set if EncodedDeliveryStreamCycleDesc(41084) field is specified and must immediately precede it.
	#[serde(rename = "41083")]
	/// Encoded (non-ASCII characters) representation of the DeliveryStreamCycleDesc(41082) field in the encoded format specified
	/// via the MessageEncoding(347) field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "41084")]
	pub encoded_delivery_stream_cycle_desc: Option<fix_common::EncodedText<41084>>,
}
