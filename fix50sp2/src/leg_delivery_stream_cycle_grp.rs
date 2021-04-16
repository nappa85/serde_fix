
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegDeliveryStreamCycleGrp {
	/// NoLegDeliveryStreamCycles
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41456")]
	pub leg_delivery_stream_cycles: Option<fix_common::RepeatingValues<LegDeliveryStreamCycle>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegDeliveryStreamCycle {
	/// Required if NoLegDeliveryStreamCycles(41456) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41457")]
	pub leg_delivery_stream_cycle_desc: Option<String>,
	/// Must be set if EncodedLegDeliveryStreamCycleDesc(41459) field is specified and must immediately precede it.
	#[serde(rename = "41458")]
	/// Encoded (non-ASCII characters) representation of the LegDeliveryStreamCycleDesc(41457) field in the encoded format specified
	/// via the MessageEncoding(347) field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "41459")]
	pub encoded_leg_delivery_stream_cycle_desc: Option<fix_common::EncodedText<41459>>,
}
