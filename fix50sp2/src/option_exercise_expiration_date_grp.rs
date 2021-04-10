
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OptionExerciseExpirationDateGrp {
	/// NoOptionExerciseExpirationDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41152")]
	pub option_exercise_expiration_dates: Option<crate::entities::RepeatingValues<OptionExerciseExpirationDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OptionExerciseExpirationDate {
	/// Required if NoOptionExpirationDates(41152) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41153")]
	pub option_exercise_expiration_date: Option<crate::entities::LocalMktDate>,
	/// OptionExerciseExpirationDateType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41154")]
	pub option_exercise_expiration_date_type: Option<OptionExerciseExpirationDateType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OptionExerciseExpirationDateType {
	/// Unadjusted
	#[serde(rename = "0")]
	Unadjusted,
	/// Adjusted
	#[serde(rename = "1")]
	Adjusted,
}
