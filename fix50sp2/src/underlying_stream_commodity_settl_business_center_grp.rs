
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingStreamCommoditySettlBusinessCenterGrp {
	/// NoUnderlyingStreamCommoditySettlBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41962")]
	pub underlying_stream_commodity_settl_business_centers: Option<crate::entities::RepeatingValues<UnderlyingStreamCommoditySettlBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingStreamCommoditySettlBusinessCenter {
	/// Required if NoUnderlyingStreamCommoditySettlBusinessCenters(41962) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41963")]
	pub underlying_stream_commodity_settl_business_center: Option<String>,
}
