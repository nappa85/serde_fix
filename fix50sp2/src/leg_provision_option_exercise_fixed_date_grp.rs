
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProvisionOptionExerciseFixedDateGrp {
	/// NoLegProvisionOptionExerciseFixedDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40495")]
	pub leg_provision_option_exercise_fixed_dates: Option<fix_common::RepeatingValues<LegProvisionOptionExerciseFixedDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProvisionOptionExerciseFixedDate {
	/// Required if NoLegProvisionOptionExerciseFixedDates(40495) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40496")]
	pub leg_provision_option_exercise_fixed_date: Option<fix_common::LocalMktDate>,
	/// LegProvisionOptionExerciseFixedDateType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40497")]
	pub leg_provision_option_exercise_fixed_date_type: Option<LegProvisionOptionExerciseFixedDateType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegProvisionOptionExerciseFixedDateType {
	/// Unadjusted
	#[serde(rename = "0")]
	Unadjusted,
	/// Adjusted
	#[serde(rename = "1")]
	Adjusted,
}

impl Default for LegProvisionOptionExerciseFixedDateType {
	fn default() -> Self {
		LegProvisionOptionExerciseFixedDateType::Unadjusted
	}
}
