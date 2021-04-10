
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingStreamTerminationDate {
	/// UnderlyingStreamTerminationDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40548")]
	pub underlying_stream_termination_date_unadjusted: Option<crate::entities::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified value would be specific to this instance of the underlying instrument's termination date of the stream.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40549")]
	pub underlying_stream_termination_date_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified values would be specific to this instance of the underlying instrument's termination date of the stream.
	#[serde(flatten)]
	pub underlying_stream_termination_date_business_center_grp: Option<super::underlying_stream_termination_date_business_center_grp::UnderlyingStreamTerminationDateBusinessCenterGrp>,
	/// UnderlyingStreamTerminationDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40551")]
	pub underlying_stream_termination_date_relative_to: Option<i32>,
	/// Conditionally required when UnderlyingStreamTerminationDateOffsetUnit(40553) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40552")]
	pub underlying_stream_termination_date_offset_period: Option<i32>,
	/// Conditionally required when UnderlyingPaymentTerminationDateOffsetPeriod(40552) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40553")]
	pub underlying_stream_termination_date_offset_unit: Option<String>,
	/// UnderlyingStreamTerminationDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40554")]
	pub underlying_stream_termination_date_offset_day_type: Option<i32>,
	/// UnderlyingStreamTerminationDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40555")]
	pub underlying_stream_termination_date_adjusted: Option<crate::entities::LocalMktDate>,
}
