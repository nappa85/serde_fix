
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OptionExercise {
	/// ExerciseDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41106")]
	pub exercise_desc: Option<String>,
	/// Must be set if EncodedExerciseDesc(41108) field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41107")]
	pub encoded_exercise_desc_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the ExerciseDesc(41106) field in the encoded format specified via the MessageEncoding(347)
	/// field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41108")]
	pub encoded_exercise_desc: Option<String>,
	/// AutomaticExerciseIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41109")]
	pub automatic_exercise_indicator: Option<fix_common::Boolean>,
	/// AutomaticExerciseThresholdRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41110")]
	pub automatic_exercise_threshold_rate: Option<f64>,
	/// ExerciseConfirmationMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41111")]
	pub exercise_confirmation_method: Option<ExerciseConfirmationMethod>,
	/// ManualNoticeBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41112")]
	pub manual_notice_business_center: Option<String>,
	/// FallbackExerciseIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41113")]
	pub fallback_exercise_indicator: Option<fix_common::Boolean>,
	/// LimitedRightToConfirmIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41114")]
	pub limited_right_to_confirm_indicator: Option<fix_common::Boolean>,
	/// ExerciseSplitTicketIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41115")]
	pub exercise_split_ticket_indicator: Option<fix_common::Boolean>,
	/// OptionExerciseDates
	#[serde(flatten)]
	pub option_exercise_dates: Option<super::option_exercise_dates::OptionExerciseDates>,
	/// OptionExerciseExpiration
	#[serde(flatten)]
	pub option_exercise_expiration: Option<super::option_exercise_expiration::OptionExerciseExpiration>,
	/// SettlMethodElectingPartySide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42590")]
	pub settl_method_electing_party_side: Option<i32>,
	/// SettlMethodElectionDate
	#[serde(flatten)]
	pub settl_method_election_date: Option<super::settl_method_election_date::SettlMethodElectionDate>,
	/// OptionExerciseMakeWholeProvision
	#[serde(flatten)]
	pub option_exercise_make_whole_provision: Option<super::option_exercise_make_whole_provision::OptionExerciseMakeWholeProvision>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExerciseConfirmationMethod {
	/// Not required
	#[serde(rename = "0")]
	NotRequired,
	/// Non-electronic
	#[serde(rename = "1")]
	NonElectronic,
	/// Electronic
	#[serde(rename = "2")]
	Electronic,
	/// Unknown at time of report
	#[serde(rename = "3")]
	UnknownAtTimeOfReport,
}
