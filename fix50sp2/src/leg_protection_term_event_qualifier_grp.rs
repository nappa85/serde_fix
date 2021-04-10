
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProtectionTermEventQualifierGrp {
	/// NoLegProtectionTermEventQualifiers
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41633")]
	pub leg_protection_term_event_qualifiers: Option<crate::entities::RepeatingValues<LegProtectionTermEventQualifier>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProtectionTermEventQualifier {
	/// Required if NoLegProtectionTermEventQualifiers(41633) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41634")]
	pub leg_protection_term_event_qualifier_item: Option<LegProtectionTermEventQualifierItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegProtectionTermEventQualifierItem {
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
