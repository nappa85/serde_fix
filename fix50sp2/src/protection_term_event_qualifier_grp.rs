
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProtectionTermEventQualifierGrp {
	/// NoProtectionTermEventQualifiers
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40199")]
	pub protection_term_event_qualifiers: Option<fix_common::RepeatingValues<ProtectionTermEventQualifier>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProtectionTermEventQualifier {
	/// Required if NoProtectionTermEventQualifiers(40199) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40200")]
	pub protection_term_event_qualifier_item: Option<ProtectionTermEventQualifierItem>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ProtectionTermEventQualifierItem {
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

impl Default for ProtectionTermEventQualifierItem {
	fn default() -> Self {
		ProtectionTermEventQualifierItem::RetructuringMultipleHoldingObligations
	}
}
