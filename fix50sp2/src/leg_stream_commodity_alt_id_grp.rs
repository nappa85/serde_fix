
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegStreamCommodityAltIDGrp {
	/// NoLegStreamCommodityAltIDs
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41674")]
	pub leg_stream_commodity_alt_i_ds: Option<crate::entities::RepeatingValues<LegStreamCommodityAltID>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegStreamCommodityAltID {
	/// Required if NoLegStreamCommodityAltIDs(41674) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41675")]
	pub leg_stream_commodity_alt_id: Option<String>,
	/// Required if NoLegStreamCommodityAltIDs(41674) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41676")]
	pub leg_stream_commodity_alt_id_source: Option<String>,
}
