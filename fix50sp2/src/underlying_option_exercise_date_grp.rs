
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingOptionExerciseDateGrp {
	/// NoUnderlyingOptionExerciseDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41841")]
	pub underlying_option_exercise_dates: Option<fix_common::RepeatingValues<UnderlyingOptionExerciseDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingOptionExerciseDate {
	/// Required if NoUnderlyingOptionExerciseDates(41841) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41842")]
	pub underlying_option_exercise_date: Option<fix_common::LocalMktDate>,
	/// UnderlyingOptionExerciseDateType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41843")]
	pub underlying_option_exercise_date_type: Option<UnderlyingOptionExerciseDateType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingOptionExerciseDateType {
	/// Unadjusted
	#[serde(rename = "0")]
	Unadjusted,
	/// Adjusted
	#[serde(rename = "1")]
	Adjusted,
}
