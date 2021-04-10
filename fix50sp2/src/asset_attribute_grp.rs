
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AssetAttributeGrp {
	/// NoAssetAttributes
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2304")]
	pub asset_attributes: Option<crate::entities::RepeatingValues<AssetAttribute>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AssetAttribute {
	/// Required if NoAssetAttributes(2304) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2305")]
	pub asset_attribute_type: Option<String>,
	/// AssetAttributeValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2306")]
	pub asset_attribute_value: Option<String>,
	/// AssetAttributeLimit
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2307")]
	pub asset_attribute_limit: Option<String>,
}
