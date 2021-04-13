
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegComplexEventCreditEventQualifierGrp {
	/// NoLegComplexEventCreditEventQualifiers
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41374")]
	pub leg_complex_event_credit_event_qualifiers: Option<fix_common::RepeatingValues<LegComplexEventCreditEventQualifier>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegComplexEventCreditEventQualifier {
	/// Required if NoLegComplexEventCreditEventQualifiers(41374) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41375")]
	pub leg_complex_event_credit_event_qualifier_item: Option<LegComplexEventCreditEventQualifierItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegComplexEventCreditEventQualifierItem {
	/// Retructuring - multiple holding obligations (In relation to a restructuring credit event, unless multiple holder obligation
	/// is not specified restructurings are limited to multiple holder obligations. A multiple holder obligation means an obligation
	/// that is held by more than three holders that are not affiliates of each other and where at least two thirds of the holders
	/// must agree to the event that constitutes the restructuring credit event. ISDA 2003 Term: Multiple Holder Obligation)
	#[serde(rename = "H")]
	RetructuringMultipleHoldingObligations,
	/// Restructuring - multiple credit event notices (Presence of this element and value set to 'true' indicates that Section 3.9
	/// of the 2003 Credit Derivatives Definitions shall apply. Absence of this element indicates that Section 3.9 shall not apply.
	/// NOTE: Not allowed under ISDA Credit 1999)
	#[serde(rename = "E")]
	RestructuringMultipleCreditEventNotices,
	/// Floating rate interest shortfall (Indicates compounding)
	#[serde(rename = "C")]
	FloatingRateInterestShortfall,
}

impl Default for LegComplexEventCreditEventQualifierItem {
	fn default() -> Self {
		LegComplexEventCreditEventQualifierItem::RetructuringMultipleHoldingObligations
	}
}
