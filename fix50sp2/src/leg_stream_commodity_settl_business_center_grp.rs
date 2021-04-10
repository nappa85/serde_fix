
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegStreamCommoditySettlBusinessCenterGrp {
	/// NoLegStreamCommoditySettlBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41646")]
	pub leg_stream_commodity_settl_business_centers: Option<crate::entities::RepeatingValues<LegStreamCommoditySettlBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegStreamCommoditySettlBusinessCenter {
	/// Required if NoLegStreamCommoditySettlementBusinessCenters(41646) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41647")]
	pub leg_stream_commodity_settl_business_center: Option<String>,
}
