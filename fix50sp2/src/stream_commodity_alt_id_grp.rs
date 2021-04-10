
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamCommodityAltIDGrp {
	/// NoStreamCommodityAltIDs
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41277")]
	pub stream_commodity_alt_i_ds: Option<crate::entities::RepeatingValues<StreamCommodityAltID>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamCommodityAltID {
	/// Required if NoStreamCommodityAltIDs(41277) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41278")]
	pub stream_commodity_alt_id: Option<String>,
	/// Required if NoStreamCommodityAltIDs(41277) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41279")]
	pub stream_commodity_alt_id_source: Option<String>,
}
