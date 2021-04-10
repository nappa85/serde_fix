
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SettlMethodElectionDate {
	/// SettlMethodElectionDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42777")]
	pub settl_method_election_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the DateAdjustment component in Instrument. The specified
	/// value would be specific to OptionExercise.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42778")]
	pub settl_method_election_date_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the DateAdjustment component in Instrument. The specified values
	/// would be specific to OptionExercise.
	#[serde(flatten)]
	pub settl_method_election_date_business_center_grp: Option<super::settl_method_election_date_business_center_grp::SettlMethodElectionDateBusinessCenterGrp>,
	/// SettlMethodElectionDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42779")]
	pub settl_method_election_date_relative_to: Option<i32>,
	/// Conditionally required when SettlMethodElectionDateOffsetUnit(42781) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42780")]
	pub settl_method_election_date_offset_period: Option<i32>,
	/// Conditionally required when SettlMethodElectionDateOffsetPeriod(42780) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42781")]
	pub settl_method_election_date_offset_unit: Option<String>,
	/// SettlMethodElectionDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42782")]
	pub settl_method_election_date_offset_day_type: Option<i32>,
	/// SettlMethodElectionDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42783")]
	pub settl_method_election_date_adjusted: Option<fix_common::LocalMktDate>,
}
