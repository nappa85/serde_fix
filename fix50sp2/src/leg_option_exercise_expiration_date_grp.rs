
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegOptionExerciseExpirationDateGrp {
	/// NoLegOptionExerciseExpirationDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41527")]
	pub leg_option_exercise_expiration_dates: Option<fix_common::RepeatingValues<LegOptionExerciseExpirationDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegOptionExerciseExpirationDate {
	/// Required if NoLegOptionExerciseExpirationDates(41527) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41528")]
	pub leg_option_exercise_expiration_date: Option<fix_common::LocalMktDate>,
	/// LegOptionExerciseExpirationDateType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41529")]
	pub leg_option_exercise_expiration_date_type: Option<LegOptionExerciseExpirationDateType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegOptionExerciseExpirationDateType {
	/// Unadjusted
	#[serde(rename = "0")]
	Unadjusted,
	/// Adjusted
	#[serde(rename = "1")]
	Adjusted,
}
