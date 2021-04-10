
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegStreamAssetAttributeGrp {
	/// NoLegStreamAssetAttributes
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41452")]
	pub leg_stream_asset_attributes: Option<crate::entities::RepeatingValues<LegStreamAssetAttribute>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegStreamAssetAttribute {
	/// Required if NoLegStreamAssetAttributes(41452) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41453")]
	pub leg_stream_asset_attribute_type: Option<String>,
	/// LegStreamAssetAttributeValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41454")]
	pub leg_stream_asset_attribute_value: Option<String>,
	/// LegStreamAssetAttributeLimit
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41455")]
	pub leg_stream_asset_attribute_limit: Option<String>,
}
