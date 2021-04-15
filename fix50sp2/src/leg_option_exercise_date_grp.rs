
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegOptionExerciseDateGrp {
	/// NoLegOptionExerciseDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41512")]
	pub leg_option_exercise_dates: Option<fix_common::RepeatingValues<LegOptionExerciseDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegOptionExerciseDate {
	/// Required if NoLegOptionExerciseDates(41512) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41513")]
	pub leg_option_exercise_date: Option<fix_common::LocalMktDate>,
	/// LegOptionExerciseDateType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41514")]
	pub leg_option_exercise_date_type: Option<LegOptionExerciseDateType>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegOptionExerciseDateType {
	/// Unadjusted
	#[serde(rename = "0")]
	Unadjusted,
	/// Adjusted
	#[serde(rename = "1")]
	Adjusted,
}

impl Default for LegOptionExerciseDateType {
	fn default() -> Self {
		LegOptionExerciseDateType::Unadjusted
	}
}
