
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OptionExerciseBusinessCenterGrp {
	/// NoOptionExerciseBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41116")]
	pub option_exercise_business_centers: Option<crate::entities::RepeatingValues<OptionExerciseBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OptionExerciseBusinessCenter {
	/// Required if NoOptionExerciseBusinessCenters(41116) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41117")]
	pub option_exercise_business_center: Option<String>,
}
