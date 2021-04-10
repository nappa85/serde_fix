
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeliveryStreamCommoditySourceGrp {
	/// NoDeliveryStreamCommoditySources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41085")]
	pub delivery_stream_commodity_sources: Option<crate::entities::RepeatingValues<DeliveryStreamCommoditySource>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeliveryStreamCommoditySource {
	/// Required if NoDeliveryStreamCommoditySources(41085) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41086")]
	pub delivery_stream_commodity_source: Option<String>,
}
