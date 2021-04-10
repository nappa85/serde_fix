
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingDeliveryStreamCommoditySourceGrp {
	/// NoUnderlyingDeliveryStreamCommoditySources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41808")]
	pub underlying_delivery_stream_commodity_sources: Option<fix_common::RepeatingValues<UnderlyingDeliveryStreamCommoditySource>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingDeliveryStreamCommoditySource {
	/// Required if NoUnderlyingDeliveryStreamCommoditySources(41808) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41809")]
	pub underlying_delivery_stream_commodity_source: Option<String>,
}
