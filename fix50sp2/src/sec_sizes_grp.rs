
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecSizesGrp {
	/// Number of entries following. Conditionally required when <a href="tag_279_MDUpdateAction.html" target="bottom">MDUpdateAction&nbsp;(279)</a> = New(0) and <a href="tag_269_MDEntryType.html" target="bottom">MDEntryType&nbsp;(269)</a> = Bid(0) or Offer(1).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1177")]
	pub of_sec_sizes: Option<fix_common::RepeatingValues<OfSecSize>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OfSecSize {
	/// Defines the type of secondary size specified in <a href="tag_1179_MDSecSize.html" target="bottom">MDSecSize(1179)&nbsp;(1179)</a> . Must be first field in this repeating group
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1178")]
	pub md_sec_size_type: Option<MDSecSizeType>,
	/// MDSecSize
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1179")]
	pub md_sec_size: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MDSecSizeType {
	/// Customer
	#[serde(rename = "1")]
	Customer,
	/// Customer professional (Quantity of high-volume investors acting similar to broker-dealers)
	#[serde(rename = "2")]
	CustomerProfessional,
	/// Do not trade through (Quantity that cannot trade through the away markets)
	#[serde(rename = "3")]
	DoNotTradeThrough,
}

impl Default for MDSecSizeType {
	fn default() -> Self {
		MDSecSizeType::Customer
	}
}
