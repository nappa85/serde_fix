
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegStreamCalculationPeriodDateGrp {
	/// NoLegStreamCalculationPeriodDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41638")]
	pub leg_stream_calculation_period_dates: Option<fix_common::RepeatingValues<LegStreamCalculationPeriodDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegStreamCalculationPeriodDate {
	/// Required if NoLegStreamCalculationPeriodDates(41638) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41639")]
	pub leg_stream_calculation_period_date: Option<fix_common::LocalMktDate>,
	/// LegStreamCalculationPeriodDateType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41640")]
	pub leg_stream_calculation_period_date_type: Option<LegStreamCalculationPeriodDateType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegStreamCalculationPeriodDateType {
	/// Unadjusted
	#[serde(rename = "0")]
	Unadjusted,
	/// Adjusted
	#[serde(rename = "1")]
	Adjusted,
}

impl Default for LegStreamCalculationPeriodDateType {
	fn default() -> Self {
		LegStreamCalculationPeriodDateType::Unadjusted
	}
}
