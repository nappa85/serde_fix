
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProvisionOptionExerciseBusinessCenterGrp {
	/// NoProvisionOptionExerciseBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40954")]
	pub provision_option_exercise_business_centers: Option<crate::entities::RepeatingValues<ProvisionOptionExerciseBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProvisionOptionExerciseBusinessCenter {
	/// Required if NoProvisionOptionExerciseBusinessCenters(40954) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40124")]
	pub provision_option_exercise_business_center: Option<String>,
}
