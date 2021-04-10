
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegOptionExerciseExpirationDateBusinessCenterGrp {
	/// NoLegOptionExerciseExpirationDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41515")]
	pub leg_option_exercise_expiration_date_business_centers: Option<crate::entities::RepeatingValues<LegOptionExerciseExpirationDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegOptionExerciseExpirationDateBusinessCenter {
	/// Required if NoLegOptionExerciseExpirationDateBusinessCenters(41515) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41516")]
	pub leg_option_exercise_expiration_date_business_center: Option<String>,
}
