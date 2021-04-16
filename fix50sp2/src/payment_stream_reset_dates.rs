
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamResetDates {
	/// PaymentStreamResetDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40761")]
	pub payment_stream_reset_date_relative_to: Option<i32>,
	/// When specified, this overrides the business day convention defined in the DateAdjustment component in Instrument. The specified
	/// value would be specific to this instance of the payment stream's reset dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40762")]
	pub payment_stream_reset_date_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the DateAdjustment component in Instrument. The specified values
	/// would be specific to this instance of the payment stream's reset dates.
	#[serde(flatten)]
	pub payment_stream_reset_date_business_center_grp: Option<super::payment_stream_reset_date_business_center_grp::PaymentStreamResetDateBusinessCenterGrp>,
	/// Conditionally required when PaymentStreamResetFrequencyUnit(40765) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40764")]
	pub payment_stream_reset_frequency_period: Option<i32>,
	/// Conditionally required when PaymentStreamResetFrequencyPeriod(40764) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40765")]
	pub payment_stream_reset_frequency_unit: Option<String>,
	/// PaymentStreamResetWeeklyRollConvention
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40766")]
	pub payment_stream_reset_weekly_roll_convention: Option<PaymentStreamResetWeeklyRollConvention>,
	/// PaymentStreamInitialFixingDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40767")]
	pub payment_stream_initial_fixing_date_relative_to: Option<i32>,
	/// When specified, this overrides the business day convention defined in the DateAdjustment component in Instrument. The specified
	/// value would be specific to this instance of the payment stream's reset dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40768")]
	pub payment_stream_initial_fixing_date_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the DateAdjustment component in Instrument. The specified values
	/// would be specific to this instance of the payment stream's reset dates.
	#[serde(flatten)]
	pub payment_stream_initial_fixing_date_business_center_grp: Option<super::payment_stream_initial_fixing_date_business_center_grp::PaymentStreamInitialFixingDateBusinessCenterGrp>,
	/// Conditionally required when PaymentStreamInitialFixingDateOffsetUnit(40771) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40770")]
	pub payment_stream_initial_fixing_date_offset_period: Option<i32>,
	/// Conditionally required when PaymentStreamInitialFixingDateOffsetPeriod(40770) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40771")]
	pub payment_stream_initial_fixing_date_offset_unit: Option<String>,
	/// PaymentStreamInitialFixingDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40772")]
	pub payment_stream_initial_fixing_date_offset_day_type: Option<i32>,
	/// PaymentStreamInitialFixingDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40773")]
	pub payment_stream_initial_fixing_date_adjusted: Option<fix_common::LocalMktDate>,
	/// PaymentStreamFixingDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40774")]
	pub payment_stream_fixing_date_relative_to: Option<i32>,
	/// When specified, this overrides the business day convention defined in the DateAdjustment component in Instrument. The specified
	/// value would be specific to this instance of the payment stream's reset dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40775")]
	pub payment_stream_fixing_date_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the DateAdjustment component in Instrument. The specified values
	/// would be specific to this instance of the payment stream's reset dates.
	#[serde(flatten)]
	pub payment_stream_fixing_date_business_center_grp: Option<super::payment_stream_fixing_date_business_center_grp::PaymentStreamFixingDateBusinessCenterGrp>,
	/// Conditionally required when PaymentStreamFixingDateOffsetUnit(40778) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40777")]
	pub payment_stream_fixing_date_offset_period: Option<i32>,
	/// Conditionally required when PaymentStreamFixingDateOffsetPeriod(40777) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40778")]
	pub payment_stream_fixing_date_offset_unit: Option<String>,
	/// PaymentStreamFixingDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40779")]
	pub payment_stream_fixing_date_offset_day_type: Option<i32>,
	/// PaymentStreamFixingDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40780")]
	pub payment_stream_fixing_date_adjusted: Option<fix_common::LocalMktDate>,
	/// Conditionally required when PaymentStreamRateCutoffDateOffsetUnit(40782) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40781")]
	pub payment_stream_rate_cutoff_date_offset_period: Option<i32>,
	/// Conditionally required when PaymentStreamRateCutoffDateOffsetPeriod(40783) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40782")]
	pub payment_stream_rate_cutoff_date_offset_unit: Option<String>,
	/// PaymentStreamRateCutoffDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40783")]
	pub payment_stream_rate_cutoff_date_offset_day_type: Option<i32>,
	/// PaymentStreamFixingDateGrp
	#[serde(flatten)]
	pub payment_stream_fixing_date_grp: Option<super::payment_stream_fixing_date_grp::PaymentStreamFixingDateGrp>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PaymentStreamResetWeeklyRollConvention {
	/// Monday
	#[serde(rename = "MON")]
	Monday,
	/// Tuesday
	#[serde(rename = "TUE")]
	Tuesday,
	/// Wednesday
	#[serde(rename = "WED")]
	Wednesday,
	/// Thursday
	#[serde(rename = "THU")]
	Thursday,
	/// Friday
	#[serde(rename = "FRI")]
	Friday,
	/// Saturday
	#[serde(rename = "SAT")]
	Saturday,
	/// Sunday
	#[serde(rename = "SUN")]
	Sunday,
}

impl Default for PaymentStreamResetWeeklyRollConvention {
	fn default() -> Self {
		PaymentStreamResetWeeklyRollConvention::Monday
	}
}
