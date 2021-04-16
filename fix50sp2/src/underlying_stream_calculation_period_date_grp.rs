
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingStreamCalculationPeriodDateGrp {
	/// NoUnderlyingStreamCalculationPeriodDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41954")]
	pub underlying_stream_calculation_period_dates: Option<fix_common::RepeatingValues<UnderlyingStreamCalculationPeriodDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingStreamCalculationPeriodDate {
	/// Required if NoUnderlyingStreamCalculationPeriodDates(41954) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41955")]
	pub underlying_stream_calculation_period_date: Option<fix_common::LocalMktDate>,
	/// UnderlyingStreamCalculationPeriodDateType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41956")]
	pub underlying_stream_calculation_period_date_type: Option<UnderlyingStreamCalculationPeriodDateType>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingStreamCalculationPeriodDateType {
	/// Unadjusted
	#[serde(rename = "0")]
	Unadjusted,
	/// Adjusted
	#[serde(rename = "1")]
	Adjusted,
}

impl Default for UnderlyingStreamCalculationPeriodDateType {
	fn default() -> Self {
		UnderlyingStreamCalculationPeriodDateType::Unadjusted
	}
}
