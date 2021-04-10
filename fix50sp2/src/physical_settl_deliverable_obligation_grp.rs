
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PhysicalSettlDeliverableObligationGrp {
	/// NoPhysicalSettlDeliverableObligations
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40209")]
	pub physical_settl_deliverable_obligations: Option<crate::entities::RepeatingValues<PhysicalSettlDeliverableObligation>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PhysicalSettlDeliverableObligation {
	/// Required if NoPhysicalSettlDeliverableObligations (40209) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40210")]
	pub physical_settl_deliverable_obligation_type: Option<String>,
	/// PhysicalSettlDeliverableObligationValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40211")]
	pub physical_settl_deliverable_obligation_value: Option<String>,
}
