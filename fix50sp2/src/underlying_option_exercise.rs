
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingOptionExercise {
	/// UnderlyingExerciseDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41810")]
	pub underlying_exercise_desc: Option<String>,
	/// Must be set if EncodedUnderlyingExerciseDesc(41812) field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41811")]
	pub encoded_underlying_exercise_desc_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the UnderlyingExerciseDesc(41810) field in the encoded format specified via
	/// the MessageEncoding(347) field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41812")]
	pub encoded_underlying_exercise_desc: Option<String>,
	/// UnderlyingAutomaticExerciseIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41813")]
	pub underlying_automatic_exercise_indicator: Option<crate::entities::Boolean>,
	/// UnderlyingAutomaticExerciseThresholdRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41814")]
	pub underlying_automatic_exercise_threshold_rate: Option<f64>,
	/// UnderlyingExerciseConfirmationMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41815")]
	pub underlying_exercise_confirmation_method: Option<UnderlyingExerciseConfirmationMethod>,
	/// UnderlyingManualNoticeBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41816")]
	pub underlying_manual_notice_business_center: Option<String>,
	/// UnderlyingFallbackExerciseIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41817")]
	pub underlying_fallback_exercise_indicator: Option<crate::entities::Boolean>,
	/// UnderlyingLimitedRightToConfirmIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41818")]
	pub underlying_limited_right_to_confirm_indicator: Option<crate::entities::Boolean>,
	/// UnderlyingExerciseSplitTicketIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41819")]
	pub underlying_exercise_split_ticket_indicator: Option<crate::entities::Boolean>,
	/// UnderlyingOptionExerciseDates
	#[serde(flatten)]
	pub underlying_option_exercise_dates: Option<super::underlying_option_exercise_dates::UnderlyingOptionExerciseDates>,
	/// UnderlyingOptionExerciseExpiration
	#[serde(flatten)]
	pub underlying_option_exercise_expiration: Option<super::underlying_option_exercise_expiration::UnderlyingOptionExerciseExpiration>,
	/// UnderlyingSettlMethodElectingPartySide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42887")]
	pub underlying_settl_method_electing_party_side: Option<i32>,
	/// UnderlyingSettlMethodElectionDate
	#[serde(flatten)]
	pub underlying_settl_method_election_date: Option<super::underlying_settl_method_election_date::UnderlyingSettlMethodElectionDate>,
	/// UnderlyingOptionExerciseMakeWholeProvision
	#[serde(flatten)]
	pub underlying_option_exercise_make_whole_provision: Option<super::underlying_option_exercise_make_whole_provision::UnderlyingOptionExerciseMakeWholeProvision>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingExerciseConfirmationMethod {
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
