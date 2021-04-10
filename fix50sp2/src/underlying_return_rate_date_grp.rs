
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingReturnRateDateGrp {
	/// NoUnderlyingReturnRateDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43008")]
	pub underlying_return_rate_dates: Option<crate::entities::RepeatingValues<UnderlyingReturnRateDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingReturnRateDate {
	/// Required if NoUnderlyingReturnRateDates(43008) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43009")]
	pub underlying_return_rate_date_mode: Option<i32>,
	/// UnderlyingReturnRateValuationDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43010")]
	pub underlying_return_rate_valuation_date_relative_to: Option<i32>,
	/// Conditionally required when UnderlyingReturnRateValuationDateOffsetUnit(43012) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43011")]
	pub underlying_return_rate_valuation_date_offset_period: Option<i32>,
	/// Conditionally required when UnderlyingReturnRateValuationDateOffsetPeriod(43011) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43012")]
	pub underlying_return_rate_valuation_date_offset_unit: Option<String>,
	/// UnderlyingReturnRateValuationDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43013")]
	pub underlying_return_rate_valuation_date_offset_day_type: Option<i32>,
	/// UnderlyingReturnRateValuationStartDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43014")]
	pub underlying_return_rate_valuation_start_date_unadjusted: Option<crate::entities::LocalMktDate>,
	/// UnderlyingReturnRateValuationStartDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43015")]
	pub underlying_return_rate_valuation_start_date_relative_to: Option<i32>,
	/// Conditionally required when UnderlyingReturnRateValuationStartDateOffsetUnit(43017) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43016")]
	pub underlying_return_rate_valuation_start_date_offset_period: Option<i32>,
	/// Conditionally required when UnderlyingReturnRateValuationStartDateOffsetPeriod(43016) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43017")]
	pub underlying_return_rate_valuation_start_date_offset_unit: Option<String>,
	/// UnderlyingReturnRateValuationStartDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43018")]
	pub underlying_return_rate_valuation_start_date_offset_day_type: Option<i32>,
	/// UnderlyingReturnRateValuationStartDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43019")]
	pub underlying_return_rate_valuation_start_date_adjusted: Option<crate::entities::LocalMktDate>,
	/// UnderlyingReturnRateValuationEndDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43020")]
	pub underlying_return_rate_valuation_end_date_unadjusted: Option<crate::entities::LocalMktDate>,
	/// UnderlyingReturnRateValuationEndDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43021")]
	pub underlying_return_rate_valuation_end_date_relative_to: Option<i32>,
	/// Conditionally required when UnderlyingReturnRateValuationEndDateOffsetUnit(43023) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43022")]
	pub underlying_return_rate_valuation_end_date_offset_period: Option<i32>,
	/// Conditionally required when UnderlyingReturnRateValuationEndDateOffsetPeriod(43022) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43023")]
	pub underlying_return_rate_valuation_end_date_offset_unit: Option<String>,
	/// UnderlyingReturnRateValuationEndDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43024")]
	pub underlying_return_rate_valuation_end_date_offset_day_type: Option<i32>,
	/// UnderlyingReturnRateValuationEndDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43025")]
	pub underlying_return_rate_valuation_end_date_adjusted: Option<crate::entities::LocalMktDate>,
	/// Conditionally required when UnderlyingReturnRateValuationFrequencyUnit(43027) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43026")]
	pub underlying_return_rate_valuation_frequency_period: Option<i32>,
	/// Conditionally required when UnderlyingReturnRateValuationFrequencyPeriod(43026) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43027")]
	pub underlying_return_rate_valuation_frequency_unit: Option<String>,
	/// When specified, this overrides the date roll convention defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified values would be specific to this instance of the return rate dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43028")]
	pub underlying_return_rate_valuation_frequency_roll_convention: Option<String>,
	/// When specified, this overrides the business day convention defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified value would be specific to payment stream return rate valuation dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43029")]
	pub underlying_return_rate_valuation_date_business_day_convention: Option<i32>,
}
