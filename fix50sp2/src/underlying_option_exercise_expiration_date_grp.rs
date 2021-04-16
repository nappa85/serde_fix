
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingOptionExerciseExpirationDateGrp {
	/// NoUnderlyingOptionExerciseExpirationDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41856")]
	pub underlying_option_exercise_expiration_dates: Option<fix_common::RepeatingValues<UnderlyingOptionExerciseExpirationDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingOptionExerciseExpirationDate {
	/// Required if NoUnderlyingOptionExpirationDates(41856) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41857")]
	pub underlying_option_exercise_expiration_date: Option<fix_common::LocalMktDate>,
	/// UnderlyingOptionExerciseExpirationDateType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41858")]
	pub underlying_option_exercise_expiration_date_type: Option<UnderlyingOptionExerciseExpirationDateType>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingOptionExerciseExpirationDateType {
	/// Unadjusted
	#[serde(rename = "0")]
	Unadjusted,
	/// Adjusted
	#[serde(rename = "1")]
	Adjusted,
}

impl Default for UnderlyingOptionExerciseExpirationDateType {
	fn default() -> Self {
		UnderlyingOptionExerciseExpirationDateType::Unadjusted
	}
}
