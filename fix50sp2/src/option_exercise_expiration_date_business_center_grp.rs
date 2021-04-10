
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OptionExerciseExpirationDateBusinessCenterGrp {
	/// NoOptionExerciseExpirationDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41140")]
	pub option_exercise_expiration_date_business_centers: Option<fix_common::RepeatingValues<OptionExerciseExpirationDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OptionExerciseExpirationDateBusinessCenter {
	/// Required if NoOptionExerciseExpirationDateBusinessCenters(41140) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41141")]
	pub option_exercise_expiration_date_business_center: Option<String>,
}
