
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamAssetAttributeGrp {
	/// NoStreamAssetAttributes
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41237")]
	pub no_stream_asset_attributes: Option<NoStreamAssetAttributes>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NoStreamAssetAttribute {
	/// Required if NoStreamAssetAttributes(41237) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41238")]
	pub stream_asset_attribute_type: Option<String>,
	/// StreamAssetAttributeValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41239")]
	pub stream_asset_attribute_value: Option<String>,
	/// StreamAssetAttributeLimit
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41240")]
	pub stream_asset_attribute_limit: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum NoStreamAssetAttributes {
	/// Day
	#[serde(rename = "D")]
	Day,
}

impl Default for NoStreamAssetAttributes {
	fn default() -> Self {
		NoStreamAssetAttributes::Day
	}
}
