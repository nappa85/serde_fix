
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegDeliveryStreamCommoditySourceGrp {
	/// NoLegDeliveryStreamCommoditySources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41460")]
	pub leg_delivery_stream_commodity_sources: Option<crate::entities::RepeatingValues<LegDeliveryStreamCommoditySource>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegDeliveryStreamCommoditySource {
	/// Required if NoLegDeliveryStreamCommoditySources(41460) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41461")]
	pub leg_delivery_stream_commodity_source: Option<String>,
}
