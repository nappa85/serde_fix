
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamEffectiveDate {
	/// StreamEffectiveDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40907")]
	pub stream_effective_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the DateAdjustment component in Instrument. The specified
	/// value would be specific to this instance of the effective date of the stream.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40908")]
	pub stream_effective_date_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the DateAdjustment component in Instrument. The specified values
	/// would be specific to this instance of the effective date of the stream.
	#[serde(flatten)]
	pub stream_effective_business_center_grp: Option<super::stream_effective_business_center_grp::StreamEffectiveBusinessCenterGrp>,
	/// StreamEffectiveDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40910")]
	pub stream_effective_date_relative_to: Option<StreamEffectiveDateRelativeTo>,
	/// Conditionally required when StreamEffectiveDateOffsetUnit(40912) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40911")]
	pub stream_effective_date_offset_period: Option<i32>,
	/// Conditionally required when StreamEffectiveDateOffsetPeriod(40911) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40912")]
	pub stream_effective_date_offset_unit: Option<String>,
	/// StreamEffectiveDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40913")]
	pub stream_effective_date_offset_day_type: Option<i32>,
	/// StreamEffectiveDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40914")]
	pub stream_effective_date_adjusted: Option<fix_common::LocalMktDate>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StreamEffectiveDateRelativeTo {
	/// Trade date
	#[serde(rename = "0")]
	TradeDate,
	/// Settlement date
	#[serde(rename = "1")]
	SettlementDate,
	/// Effective date
	#[serde(rename = "2")]
	EffectiveDate,
	/// Calculation period start date
	#[serde(rename = "3")]
	CalculationPeriodStartDate,
	/// Calculation period end date
	#[serde(rename = "4")]
	CalculationPeriodEndDate,
	/// Reset date
	#[serde(rename = "5")]
	ResetDate,
	/// Last pricing date
	#[serde(rename = "6")]
	LastPricingDate,
	/// Valuation date
	#[serde(rename = "7")]
	ValuationDate,
	/// Cash settlement date
	#[serde(rename = "8")]
	CashSettlementDate,
	/// Option exercise start date
	#[serde(rename = "9")]
	OptionExerciseStartDate,
}
