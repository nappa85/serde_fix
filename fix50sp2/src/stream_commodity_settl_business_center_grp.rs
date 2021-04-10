
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamCommoditySettlBusinessCenterGrp {
	/// NoStreamCommoditySettlBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41249")]
	pub stream_commodity_settl_business_centers: Option<crate::entities::RepeatingValues<StreamCommoditySettlBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamCommoditySettlBusinessCenter {
	/// Required if NoStreamCommoditySettlBusinessCenters(41249) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41250")]
	pub stream_commodity_settl_business_center: Option<String>,
}
