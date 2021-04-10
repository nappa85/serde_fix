
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MiscFeesSubGrp {
	/// NoMiscFeeSubTypes
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2633")]
	pub misc_fee_sub_types: Option<crate::entities::RepeatingValues<MiscFeeSubType>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MiscFeeSubType {
	/// Required if NoMiscFeeSubTypes(2633) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2634")]
	pub misc_fee_sub_type: Option<String>,
	/// MiscFeeSubTypeAmt
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2635")]
	pub misc_fee_sub_type_amt: Option<f64>,
	/// MiscFeeSubTypeDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2636")]
	pub misc_fee_sub_type_desc: Option<String>,
	/// Must be set if EncodedMiscFeeSubTypeDesc(2638) field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2637")]
	pub encoded_misc_fee_sub_type_desc_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the MiscFeeSubTypeDesc(2636) field in the encoded format specified via the
	/// MessageEncoding(347) field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2638")]
	pub encoded_misc_fee_sub_type_desc: Option<String>,
}
