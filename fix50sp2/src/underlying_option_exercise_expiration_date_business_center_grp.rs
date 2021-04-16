
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingOptionExerciseExpirationDateBusinessCenterGrp {
	/// NoUnderlyingOptionExerciseExpirationDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41844")]
	pub underlying_option_exercise_expiration_date_business_centers: Option<fix_common::RepeatingValues<UnderlyingOptionExerciseExpirationDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingOptionExerciseExpirationDateBusinessCenter {
	/// Required if NoUnderlyingOptionExerciseExpirationDateBusinessCenters(41844) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41845")]
	pub underlying_option_exercise_expiration_date_business_center: Option<String>,
}
