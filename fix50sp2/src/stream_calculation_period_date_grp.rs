
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamCalculationPeriodDateGrp {
	/// NoStreamCalculationPeriodDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41241")]
	pub stream_calculation_period_dates: Option<fix_common::RepeatingValues<StreamCalculationPeriodDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamCalculationPeriodDate {
	/// Required if NoStreamCalculationPeriodDates(41241) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41242")]
	pub stream_calculation_period_date: Option<fix_common::LocalMktDate>,
	/// StreamCalculationPeriodDateType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41243")]
	pub stream_calculation_period_date_type: Option<StreamCalculationPeriodDateType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StreamCalculationPeriodDateType {
	/// Unadjusted
	#[serde(rename = "0")]
	Unadjusted,
	/// Adjusted
	#[serde(rename = "1")]
	Adjusted,
}

impl Default for StreamCalculationPeriodDateType {
	fn default() -> Self {
		StreamCalculationPeriodDateType::Unadjusted
	}
}
