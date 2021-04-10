
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReturnRateDateGrp {
	/// NoReturnRateDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42709")]
	pub return_rate_dates: Option<fix_common::RepeatingValues<ReturnRateDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReturnRateDate {
	/// Required if NoReturnRateDates(42709) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42710")]
	pub return_rate_date_mode: Option<ReturnRateDateMode>,
	/// ReturnRateValuationDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42711")]
	pub return_rate_valuation_date_relative_to: Option<i32>,
	/// Conditionally required when ReturnRateValuationDateOffsetUnit(42713) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42712")]
	pub return_rate_valuation_date_offset_period: Option<i32>,
	/// Conditionally required when ReturnRateValuationDateOffsetPeriod(42712) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42713")]
	pub return_rate_valuation_date_offset_unit: Option<String>,
	/// ReturnRateValuationDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42714")]
	pub return_rate_valuation_date_offset_day_type: Option<i32>,
	/// ReturnRateValuationStartDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42715")]
	pub return_rate_valuation_start_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// ReturnRateValuationStartDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42716")]
	pub return_rate_valuation_start_date_relative_to: Option<i32>,
	/// Conditionally required when ReturnRateValuationStartDateOffsetUnit(42718) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42717")]
	pub return_rate_valuation_start_date_offset_period: Option<i32>,
	/// Conditionally required when ReturnRateValuationStartDateOffsetPeriod(42717) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42718")]
	pub return_rate_valuation_start_date_offset_unit: Option<String>,
	/// ReturnRateValuationStartDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42719")]
	pub return_rate_valuation_start_date_offset_day_type: Option<i32>,
	/// ReturnRateValuationStartDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42720")]
	pub return_rate_valuation_start_date_adjusted: Option<fix_common::LocalMktDate>,
	/// ReturnRateValuationEndDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42721")]
	pub return_rate_valuation_end_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// ReturnRateValuationEndDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42722")]
	pub return_rate_valuation_end_date_relative_to: Option<i32>,
	/// Conditionally required when ReturnRateValuationEndDateOffsetUnit(42724) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42723")]
	pub return_rate_valuation_end_date_offset_period: Option<i32>,
	/// Conditionally required when ReturnRateValuationEndDateOffsetPeriod(42723) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42724")]
	pub return_rate_valuation_end_date_offset_unit: Option<String>,
	/// ReturnRateValuationEndDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42725")]
	pub return_rate_valuation_end_date_offset_day_type: Option<i32>,
	/// ReturnRateValuationEndDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42726")]
	pub return_rate_valuation_end_date_adjusted: Option<fix_common::LocalMktDate>,
	/// Conditionally required when ReturnRateValuationFrequencyUnit(42728) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42727")]
	pub return_rate_valuation_frequency_period: Option<i32>,
	/// Conditionally required when ReturnRateValuationFrequencyPeriod(42727) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42728")]
	pub return_rate_valuation_frequency_unit: Option<String>,
	/// When specified, this overrides the date roll convention defined in the DateAdjustment component in Instrument. The specified
	/// values would be specific to this instance of the payment stream return rate valuation dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42729")]
	pub return_rate_valuation_frequency_roll_convention: Option<String>,
	/// When specified, this overrides the business day convention defined in the DateAdjustment component in Instrument. The specified
	/// value would be specific to payment stream return rate valuation dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42730")]
	pub return_rate_valuation_date_business_day_convention: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ReturnRateDateMode {
	/// Price valuation
	#[serde(rename = "0")]
	PriceValuation,
	/// Dividend valuation
	#[serde(rename = "1")]
	DividendValuation,
}
