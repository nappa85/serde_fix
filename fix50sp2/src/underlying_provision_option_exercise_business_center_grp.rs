
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingProvisionOptionExerciseBusinessCenterGrp {
	/// NoUnderlyingProvisionOptionExerciseBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42184")]
	pub underlying_provision_option_exercise_business_centers: Option<fix_common::RepeatingValues<UnderlyingProvisionOptionExerciseBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingProvisionOptionExerciseBusinessCenter {
	/// Required if NoUnderlyingProvisionOptionExerciseBusinessCenters(42184) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42185")]
	pub underlying_provision_option_exercise_business_center: Option<String>,
}
