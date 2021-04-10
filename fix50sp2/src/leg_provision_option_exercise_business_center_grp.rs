
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProvisionOptionExerciseBusinessCenterGrp {
	/// NoLegProvisionOptionExerciseBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40936")]
	pub leg_provision_option_exercise_business_centers: Option<crate::entities::RepeatingValues<LegProvisionOptionExerciseBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProvisionOptionExerciseBusinessCenter {
	/// Required if NoLegProvisionOptionExerciseBusinessCenters(40936) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40477")]
	pub leg_provision_option_exercise_business_center: Option<String>,
}
