
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ValueChecksGrp {
	/// NoValueChecks
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1868")]
	pub value_checks: Option<fix_common::RepeatingValues<ValueCheck>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ValueCheck {
	/// Required if NoValueChecks &gt; 0. Must be first field in repeating group.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1869")]
	pub value_check_type: Option<ValueCheckType>,
	/// Required if NoValueChecks &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1870")]
	pub value_check_action: Option<ValueCheckAction>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ValueCheckType {
	/// Price check
	#[serde(rename = "1")]
	PriceCheck,
	/// Notional value check
	#[serde(rename = "2")]
	NotionalValueCheck,
	/// Quantity check
	#[serde(rename = "3")]
	QuantityCheck,
}

impl Default for ValueCheckType {
	fn default() -> Self {
		ValueCheckType::PriceCheck
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ValueCheckAction {
	/// Do not check (checks will not be done for the specified ValueCheckType (1869))
	#[serde(rename = "0")]
	DoNotCheck,
	/// Check (checks will be vdone for the specificed ValueCheckType (1869))
	#[serde(rename = "1")]
	Check,
	/// Best effort check (the market may or may not check the specified ValueCheckType (1869) depending on availability of reference
	/// data)
	#[serde(rename = "2")]
	BestEffortCheckDependingOnAvailabilityOfReferenceData,
}

impl Default for ValueCheckAction {
	fn default() -> Self {
		ValueCheckAction::DoNotCheck
	}
}
