
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegSettlMethodElectionDate {
	/// LegSettlMethodElectionDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42574")]
	pub leg_settl_method_election_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the LegDateAdjustment component in InstrumentLeg. The
	/// specified value would be specific to LegOptionExercise.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42575")]
	pub leg_settl_method_election_date_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the LegDateAdjustment component in InstrumentLeg. The specified
	/// values would be specific to LegOptionExercise.
	#[serde(flatten)]
	pub leg_settl_method_election_date_business_center_grp: Option<super::leg_settl_method_election_date_business_center_grp::LegSettlMethodElectionDateBusinessCenterGrp>,
	/// LegSettlMethodElectionDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42576")]
	pub leg_settl_method_election_date_relative_to: Option<i32>,
	/// Conditionally required when LegSettlMethodElectionDateOffsetUnit(42578) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42577")]
	pub leg_settl_method_election_date_offset_period: Option<i32>,
	/// Conditionally required when LegSettlMethodElectionDateOffsetPeriod(42577) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42578")]
	pub leg_settl_method_election_date_offset_unit: Option<String>,
	/// LegSettlMethodElectionDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42579")]
	pub leg_settl_method_election_date_offset_day_type: Option<i32>,
	/// LegSettlMethodElectionDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42580")]
	pub leg_settl_method_election_date_adjusted: Option<fix_common::LocalMktDate>,
}
