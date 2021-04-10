
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingOptionExerciseBusinessCenterGrp {
	/// NoUnderlyingOptionExerciseBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41820")]
	pub underlying_option_exercise_business_centers: Option<fix_common::RepeatingValues<UnderlyingOptionExerciseBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingOptionExerciseBusinessCenter {
	/// Required if NoUnderlyingOptionExerciseBusinessCenters(41820) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41821")]
	pub underlying_option_exercise_business_center: Option<String>,
}
