
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OptionExerciseDateGrp {
	/// NoOptionExerciseDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41137")]
	pub option_exercise_dates: Option<fix_common::RepeatingValues<OptionExerciseDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OptionExerciseDate {
	/// Required if NoOptionExerciseDates(41137) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41138")]
	pub option_exercise_date: Option<fix_common::LocalMktDate>,
	/// OptionExerciseDateType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41139")]
	pub option_exercise_date_type: Option<OptionExerciseDateType>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum OptionExerciseDateType {
	/// Unadjusted
	#[serde(rename = "0")]
	Unadjusted,
	/// Adjusted
	#[serde(rename = "1")]
	Adjusted,
}

impl Default for OptionExerciseDateType {
	fn default() -> Self {
		OptionExerciseDateType::Unadjusted
	}
}
