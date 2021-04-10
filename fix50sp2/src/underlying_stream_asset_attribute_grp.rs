
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingStreamAssetAttributeGrp {
	/// NoUnderlyingStreamAssetAttributes
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41800")]
	pub underlying_stream_asset_attributes: Option<fix_common::RepeatingValues<UnderlyingStreamAssetAttribute>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingStreamAssetAttribute {
	/// Required if NoUnderlyingStreamAssetAttributes(41800) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41801")]
	pub underlying_stream_asset_attribute_type: Option<String>,
	/// UnderlyingStreamAssetAttributeValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41802")]
	pub underlying_stream_asset_attribute_value: Option<String>,
	/// UnderlyingStreamAssetAttributeLimit
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41803")]
	pub underlying_stream_asset_attribute_limit: Option<String>,
}
