
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegReturnRateDateGrp {
	/// NoLegReturnRateDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42508")]
	pub leg_return_rate_dates: Option<crate::entities::RepeatingValues<LegReturnRateDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegReturnRateDate {
	/// Required if NoLegReturnRateDates(42508) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42509")]
	pub leg_return_rate_date_mode: Option<i32>,
	/// LegReturnRateValuationDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42510")]
	pub leg_return_rate_valuation_date_relative_to: Option<i32>,
	/// Conditionally required when LegReturnRateValuationDateOffsetUnit(42512) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42511")]
	pub leg_return_rate_valuation_date_offset_period: Option<i32>,
	/// Conditionally required when LegReturnRateValuationDateOffsetPeriod(42511) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42512")]
	pub leg_return_rate_valuation_date_offset_unit: Option<String>,
	/// LegReturnRateValuationDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42513")]
	pub leg_return_rate_valuation_date_offset_day_type: Option<i32>,
	/// LegReturnRateValuationStartDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42514")]
	pub leg_return_rate_valuation_start_date_unadjusted: Option<crate::entities::LocalMktDate>,
	/// LegReturnRateValuationStartDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42515")]
	pub leg_return_rate_valuation_start_date_relative_to: Option<i32>,
	/// Conditionally required when LegReturnRateValuationStartDateOffsetUnit(42517) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42516")]
	pub leg_return_rate_valuation_start_date_offset_period: Option<i32>,
	/// Conditionally required when LegReturnRateValuationStartDateOffsetPeriod(42516) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42517")]
	pub leg_return_rate_valuation_start_date_offset_unit: Option<String>,
	/// LegReturnRateValuationStartDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42518")]
	pub leg_return_rate_valuation_start_date_offset_day_type: Option<i32>,
	/// LegReturnRateValuationStartDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42519")]
	pub leg_return_rate_valuation_start_date_adjusted: Option<crate::entities::LocalMktDate>,
	/// LegReturnRateValuationEndDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42520")]
	pub leg_return_rate_valuation_end_date_unadjusted: Option<crate::entities::LocalMktDate>,
	/// LegReturnRateValuationEndDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42521")]
	pub leg_return_rate_valuation_end_date_relative_to: Option<i32>,
	/// Conditionally required when LegReturnRateValuationEndDateOffsetUnit(42523) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42522")]
	pub leg_return_rate_valuation_end_date_offset_period: Option<i32>,
	/// Conditionally required when LegReturnRateValuationEndDateOffsetPeriod(42522) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42523")]
	pub leg_return_rate_valuation_end_date_offset_unit: Option<String>,
	/// LegReturnRateValuationEndDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42524")]
	pub leg_return_rate_valuation_end_date_offset_day_type: Option<i32>,
	/// LegReturnRateValuationEndDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42525")]
	pub leg_return_rate_valuation_end_date_adjusted: Option<crate::entities::LocalMktDate>,
	/// Conditionally required when LegReturnRateValuationFrequencyUnit(42527) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42526")]
	pub leg_return_rate_valuation_frequency_period: Option<i32>,
	/// Conditionally required when LegReturnRateValuationFrequencyPeriod(42526) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42527")]
	pub leg_return_rate_valuation_frequency_unit: Option<String>,
	/// When specified, this overrides the date roll convention defined in the LegDateAdjustment component in InstrumentLeg. The specified
	/// values would be specific to this instance of return rate valuation dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42528")]
	pub leg_return_rate_valuation_frequency_roll_convention: Option<String>,
	/// When specified, this overrides the business day convention defined in the LegDateAdjustment component in InstrumentLeg. The
	/// specified value would be specific to payment stream return rate valuation dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42529")]
	pub leg_return_rate_valuation_date_business_day_convention: Option<i32>,
}
