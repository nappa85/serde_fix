
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegDeliveryStreamCycleGrp {
	/// NoLegDeliveryStreamCycles
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41456")]
	pub leg_delivery_stream_cycles: Option<crate::entities::RepeatingValues<LegDeliveryStreamCycle>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegDeliveryStreamCycle {
	/// Required if NoLegDeliveryStreamCycles(41456) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41457")]
	pub leg_delivery_stream_cycle_desc: Option<String>,
	/// Must be set if EncodedLegDeliveryStreamCycleDesc(41459) field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41458")]
	pub encoded_leg_delivery_stream_cycle_desc_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the LegDeliveryStreamCycleDesc(41457) field in the encoded format specified
	/// via the MessageEncoding(347) field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41459")]
	pub encoded_leg_delivery_stream_cycle_desc: Option<String>,
}
