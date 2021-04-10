
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingSettlMethodElectionDate {
	/// UnderlyingSettlMethodElectionDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43076")]
	pub underlying_settl_method_election_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified value would be specific to UnderlyingOptionExercise.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43077")]
	pub underlying_settl_method_election_date_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified values would be specific to UnderlyingOptionExercise.
	#[serde(flatten)]
	pub underlying_settl_method_election_date_business_center_grp: Option<super::underlying_settl_method_election_date_business_center_grp::UnderlyingSettlMethodElectionDateBusinessCenterGrp>,
	/// UnderlyingSettlMethodElectionDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43078")]
	pub underlying_settl_method_election_date_relative_to: Option<i32>,
	/// Conditionally required when UnderlyingSettlMethodElectionDateOffsetUnit(43080) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43079")]
	pub underlying_settl_method_election_date_offset_period: Option<i32>,
	/// Conditionally required when UnderlyingSettlMethodElectionDateOffsetPeriod(43079) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43080")]
	pub underlying_settl_method_election_date_offset_unit: Option<String>,
	/// UnderlyingSettlMethodElectionDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43081")]
	pub underlying_settl_method_election_date_offset_day_type: Option<i32>,
	/// UnderlyingSettlMethodElectionDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43082")]
	pub underlying_settl_method_election_date_adjusted: Option<fix_common::LocalMktDate>,
}
