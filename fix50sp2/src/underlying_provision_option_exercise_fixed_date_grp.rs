
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingProvisionOptionExerciseFixedDateGrp {
	/// NoUnderlyingProvisionOptionExerciseFixedDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42112")]
	pub underlying_provision_option_exercise_fixed_dates: Option<fix_common::RepeatingValues<UnderlyingProvisionOptionExerciseFixedDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingProvisionOptionExerciseFixedDate {
	/// Required if NoUnderlyingProvisionOptionExerciseFixedDates(42112) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42113")]
	pub underlying_provision_option_exercise_fixed_date: Option<fix_common::LocalMktDate>,
	/// UnderlyingProvisionOptionExerciseFixedDateType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42114")]
	pub underlying_provision_option_exercise_fixed_date_type: Option<UnderlyingProvisionOptionExerciseFixedDateType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingProvisionOptionExerciseFixedDateType {
	/// Unadjusted
	#[serde(rename = "0")]
	Unadjusted,
	/// Adjusted
	#[serde(rename = "1")]
	Adjusted,
}

impl Default for UnderlyingProvisionOptionExerciseFixedDateType {
	fn default() -> Self {
		UnderlyingProvisionOptionExerciseFixedDateType::Unadjusted
	}
}
