
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamNonDeliverableFixingDateGrp {
	/// NoLegNonDeliverableFixingDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40367")]
	pub leg_non_deliverable_fixing_dates: Option<fix_common::RepeatingValues<LegNonDeliverableFixingDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegNonDeliverableFixingDate {
	/// Required if NoLegNonDeliverableFixingDates(40367) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40368")]
	pub leg_non_deliverable_fixing_date: Option<fix_common::LocalMktDate>,
	/// LegNonDeliverableFixingDateType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40369")]
	pub leg_non_deliverable_fixing_date_type: Option<LegNonDeliverableFixingDateType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegNonDeliverableFixingDateType {
	/// Unadjusted
	#[serde(rename = "0")]
	Unadjusted,
	/// Adjusted
	#[serde(rename = "1")]
	Adjusted,
}

impl Default for LegNonDeliverableFixingDateType {
	fn default() -> Self {
		LegNonDeliverableFixingDateType::Unadjusted
	}
}
