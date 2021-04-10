
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegOptionExerciseBusinessCenterGrp {
	/// NoLegOptionExerciseBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41491")]
	pub leg_option_exercise_business_centers: Option<crate::entities::RepeatingValues<LegOptionExerciseBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegOptionExerciseBusinessCenter {
	/// Required if NoLegOptionExerciseBusinessCenters(41491) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41492")]
	pub leg_option_exercise_business_center: Option<String>,
}
