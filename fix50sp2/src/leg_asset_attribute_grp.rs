
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegAssetAttributeGrp {
	/// NoLegAssetAttributes
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2308")]
	pub leg_asset_attributes: Option<crate::entities::RepeatingValues<LegAssetAttribute>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegAssetAttribute {
	/// Required if NoLegAssetAttributes(2308) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2309")]
	pub leg_asset_attribute_type: Option<String>,
	/// LegAssetAttributeValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2310")]
	pub leg_asset_attribute_value: Option<String>,
	/// LegAssetAttributeLimit
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2311")]
	pub leg_asset_attribute_limit: Option<String>,
}
