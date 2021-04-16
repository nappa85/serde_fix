
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingAssetAttributeGrp {
	/// NoUnderlyingAssetAttributes
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2312")]
	pub underlying_asset_attributes: Option<fix_common::RepeatingValues<UnderlyingAssetAttribute>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingAssetAttribute {
	/// Required if NoUnderlyingAssetAttributes(2312) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2313")]
	pub underlying_asset_attribute_type: Option<String>,
	/// UnderlyingAssetAttributeValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2314")]
	pub underlying_asset_attribute_value: Option<String>,
	/// UnderlyingAssetAttributeLimit
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2315")]
	pub underlying_asset_attribute_limit: Option<String>,
}
