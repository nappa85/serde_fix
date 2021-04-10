
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExpirationQty {
	/// NoExpiration
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "981")]
	pub expiration: Option<fix_common::RepeatingValues<Expiratio>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Expiratio {
	/// Required if <a href="tag_981_NoExpiration.html" target="bottom">NoExpiration&nbsp;(981)</a> &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "982")]
	pub expiration_qty_type: Option<ExpirationQtyType>,
	/// ExpQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "983")]
	pub exp_qty: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExpirationQtyType {
	/// Auto Exercise
	#[serde(rename = "1")]
	AutoExercise,
	/// Non Auto Exercise
	#[serde(rename = "2")]
	NonAutoExercise,
	/// Final Will Be Exercised
	#[serde(rename = "3")]
	FinalWillBeExercised,
	/// Contrary Intention
	#[serde(rename = "4")]
	ContraryIntention,
	/// Difference
	#[serde(rename = "5")]
	Difference,
}
