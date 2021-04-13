
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamNonDeliverableFixingDateGrp {
	/// NoNonDeliverableFixingDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40825")]
	pub non_deliverable_fixing_dates: Option<fix_common::RepeatingValues<NonDeliverableFixingDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NonDeliverableFixingDate {
	/// Required if NoNonDeliverableFixingDates(40825) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40826")]
	pub non_deliverable_fixing_date: Option<fix_common::LocalMktDate>,
	/// NonDeliverableFixingDateType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40827")]
	pub non_deliverable_fixing_date_type: Option<NonDeliverableFixingDateType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NonDeliverableFixingDateType {
	/// Unadjusted
	#[serde(rename = "0")]
	Unadjusted,
	/// Adjusted
	#[serde(rename = "1")]
	Adjusted,
}

impl Default for NonDeliverableFixingDateType {
	fn default() -> Self {
		NonDeliverableFixingDateType::Unadjusted
	}
}
