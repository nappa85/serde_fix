
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegOptionExercise {
	/// LegExerciseDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41481")]
	pub leg_exercise_desc: Option<String>,
	/// Must be set if EncodedLegExerciseDesc (41483) field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41482")]
	pub encoded_leg_exercise_desc_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the LegExerciseDesc(41481) field in the encoded format specified via the
	/// MessageEncoding(347) field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41483")]
	pub encoded_leg_exercise_desc: Option<String>,
	/// LegAutomaticExerciseIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41484")]
	pub leg_automatic_exercise_indicator: Option<crate::entities::Boolean>,
	/// LegAutomaticExerciseThresholdRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41485")]
	pub leg_automatic_exercise_threshold_rate: Option<f64>,
	/// LegExerciseConfirmationMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41486")]
	pub leg_exercise_confirmation_method: Option<LegExerciseConfirmationMethod>,
	/// LegManualNoticeBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41487")]
	pub leg_manual_notice_business_center: Option<String>,
	/// LegFallbackExerciseIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41488")]
	pub leg_fallback_exercise_indicator: Option<crate::entities::Boolean>,
	/// LegLimitRightToConfirmIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41489")]
	pub leg_limit_right_to_confirm_indicator: Option<crate::entities::Boolean>,
	/// LegExerciseSplitTicketIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41490")]
	pub leg_exercise_split_ticket_indicator: Option<crate::entities::Boolean>,
	/// LegOptionExerciseDates
	#[serde(flatten)]
	pub leg_option_exercise_dates: Option<super::leg_option_exercise_dates::LegOptionExerciseDates>,
	/// LegOptionExerciseExpiration
	#[serde(flatten)]
	pub leg_option_exercise_expiration: Option<super::leg_option_exercise_expiration::LegOptionExerciseExpiration>,
	/// LegSettlMethodElectingPartySide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42391")]
	pub leg_settl_method_electing_party_side: Option<i32>,
	/// LegSettlMethodElectionDate
	#[serde(flatten)]
	pub leg_settl_method_election_date: Option<super::leg_settl_method_election_date::LegSettlMethodElectionDate>,
	/// LegOptionExerciseMakeWholeProvision
	#[serde(flatten)]
	pub leg_option_exercise_make_whole_provision: Option<super::leg_option_exercise_make_whole_provision::LegOptionExerciseMakeWholeProvision>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegExerciseConfirmationMethod {
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
