
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingStreamCommodityAltIDGrp {
	/// NoUnderlyingStreamCommodityAltIDs
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41990")]
	pub underlying_stream_commodity_alt_i_ds: Option<crate::entities::RepeatingValues<UnderlyingStreamCommodityAltID>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingStreamCommodityAltID {
	/// Required if NoUnderlyingStreamCommodityAltIDs(41990) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41991")]
	pub underlying_stream_commodity_alt_id: Option<String>,
	/// Required if NoUnderlyingStreamCommodityAltIDs(41990) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41992")]
	pub underlying_stream_commodity_alt_id_source: Option<String>,
}
