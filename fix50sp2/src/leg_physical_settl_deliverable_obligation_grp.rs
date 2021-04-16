
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPhysicalSettlDeliverableObligationGrp {
	/// NoLegPhysicalSettlDeliverableObligations
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41604")]
	pub leg_physical_settl_deliverable_obligations: Option<fix_common::RepeatingValues<LegPhysicalSettlDeliverableObligation>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPhysicalSettlDeliverableObligation {
	/// Required if NoLegPhysicalSettlDeliverableObligations(41604) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41605")]
	pub leg_physical_settl_deliverable_obligation_type: Option<String>,
	/// LegPhysicalSettlDeliverableObligationValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41606")]
	pub leg_physical_settl_deliverable_obligation_value: Option<String>,
}
