
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProvisionOptionExerciseFixedDateGrp {
	/// NoProvisionOptionExerciseFixedDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40142")]
	pub provision_option_exercise_fixed_dates: Option<fix_common::RepeatingValues<ProvisionOptionExerciseFixedDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProvisionOptionExerciseFixedDate {
	/// Required if NoProvisionOptionExerciseFixedDates (40142) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40143")]
	pub provision_option_exercise_fixed_date: Option<fix_common::LocalMktDate>,
	/// ProvisionOptionExerciseFixedDateType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40144")]
	pub provision_option_exercise_fixed_date_type: Option<ProvisionOptionExerciseFixedDateType>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ProvisionOptionExerciseFixedDateType {
	/// Unadjusted
	#[serde(rename = "0")]
	Unadjusted,
	/// Adjusted
	#[serde(rename = "1")]
	Adjusted,
}

impl Default for ProvisionOptionExerciseFixedDateType {
	fn default() -> Self {
		ProvisionOptionExerciseFixedDateType::Unadjusted
	}
}
