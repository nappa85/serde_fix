
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingStreamCalculationPeriodDates {
	/// When specified, this overrides the business day convention defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified value would be specific to this instance of the underlying instrument's calculation period dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40556")]
	pub underlying_stream_calculation_period_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified values would be specific to this instance of the underlying instrument's calculation period dates.
	#[serde(flatten)]
	pub underlying_stream_calculation_period_business_center_grp: Option<super::underlying_stream_calculation_period_business_center_grp::UnderlyingStreamCalculationPeriodBusinessCenterGrp>,
	/// UnderlyingStreamFirstPeriodStartDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40558")]
	pub underlying_stream_first_period_start_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified value would be specific to this instance of the underlying instrument's calculation period dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40559")]
	pub underlying_stream_first_period_start_date_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified values would be specific to this instance of the underlying instrument's calculation period dates.
	#[serde(flatten)]
	pub underlying_stream_first_period_start_date_business_center_grp: Option<super::underlying_stream_first_period_start_date_business_center_grp::UnderlyingStreamFirstPeriodStartDateBusinessCenterGrp>,
	/// UnderlyingStreamFirstPeriodStartDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40561")]
	pub underlying_stream_first_period_start_date_adjusted: Option<fix_common::LocalMktDate>,
	/// UnderlyingStreamFirstRegularPeriodStartDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40562")]
	pub underlying_stream_first_regular_period_start_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// UnderlyingStreamFirstCompoundingPeriodEndDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40563")]
	pub underlying_stream_first_compounding_period_end_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// UnderlyingStreamLastRegularPeriodEndDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40564")]
	pub underlying_stream_last_regular_period_end_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// Conditionally required when UnderyingStreamCalculationFrequencyUnit(40566) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40565")]
	pub underlying_stream_calculation_frequency_period: Option<i32>,
	/// Conditionally required when UnderlyingStreamCalculationFrequencyPeriod(40565) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40566")]
	pub underlying_stream_calculation_frequency_unit: Option<String>,
	/// UnderlyingStreamCalculationRollConvention
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40567")]
	pub underlying_stream_calculation_roll_convention: Option<String>,
	/// UnderlyingStreamCalculationPeriodDatesXID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41957")]
	pub underlying_stream_calculation_period_dates_xid: Option<String>,
	/// UnderlyingStreamCalculationPeriodDatesXIDRef
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41958")]
	pub underlying_stream_calculation_period_dates_xid_ref: Option<String>,
	/// UnderlyingStreamCalculationPeriodDateGrp
	#[serde(flatten)]
	pub underlying_stream_calculation_period_date_grp: Option<super::underlying_stream_calculation_period_date_grp::UnderlyingStreamCalculationPeriodDateGrp>,
	/// UnderlyingStreamCalculationBalanceOfFirstPeriod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41959")]
	pub underlying_stream_calculation_balance_of_first_period: Option<fix_common::Boolean>,
	/// Conditionally required when UnderlyingStreamCalculationCorrectionUnit(41961) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41960")]
	pub underlying_stream_calculation_correction_period: Option<i32>,
	/// Conditionally required when UnderlyingStreamCalculationCorrectionPeriod(41960) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41961")]
	pub underlying_stream_calculation_correction_unit: Option<UnderlyingStreamCalculationCorrectionUnit>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingStreamCalculationCorrectionUnit {
	/// Day
	#[serde(rename = "D")]
	Day,
	/// Week
	#[serde(rename = "Wk")]
	Week,
	/// Month
	#[serde(rename = "Mo")]
	Month,
	/// Year
	#[serde(rename = "Yr")]
	Year,
}

impl Default for UnderlyingStreamCalculationCorrectionUnit {
	fn default() -> Self {
		UnderlyingStreamCalculationCorrectionUnit::Day
	}
}
