
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingDeliveryStreamCycleGrp {
	/// NoUnderlyingDeliveryStreamCycles
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41804")]
	pub underlying_delivery_stream_cycles: Option<fix_common::RepeatingValues<UnderlyingDeliveryStreamCycle>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingDeliveryStreamCycle {
	/// Required if NoUnderlyingDeliveryStreamCycles(41804) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41805")]
	pub underlying_delivery_stream_cycle_desc: Option<String>,
	/// Must be set if EncodedUnderlyingDeliveryStreamCycleDesc(41807) field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41806")]
	pub encoded_underlying_delivery_stream_cycle_desc_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the UnderlyingDeliverySreamCycleDesc(41805) field in the encoded format specified
	/// via the MessageEncoding(347) field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41807")]
	pub encoded_underlying_delivery_stream_cycle_desc: Option<String>,
}
