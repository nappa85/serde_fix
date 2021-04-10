
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPhysicalSettlDeliverableObligationGrp {
	/// NoUnderlyingPhysicalSettlDeliverableObligations
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42065")]
	pub underlying_physical_settl_deliverable_obligations: Option<fix_common::RepeatingValues<UnderlyingPhysicalSettlDeliverableObligation>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPhysicalSettlDeliverableObligation {
	/// Required if NoUnderlyingPhysicalSettlDeliverableObligations(42065) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42066")]
	pub underlying_physical_settl_deliverable_obligation_type: Option<String>,
	/// UnderlyingPhysicalSettlDeliverableObligationValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42067")]
	pub underlying_physical_settl_deliverable_obligation_value: Option<String>,
}
