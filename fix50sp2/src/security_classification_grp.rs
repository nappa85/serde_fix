
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecurityClassificationGrp {
	/// NoSecurityClassifications
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1582")]
	pub security_classifications: Option<crate::entities::RepeatingValues<SecurityClassification>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecurityClassification {
	/// Conditionally required if NoSecurityClassifications (1582) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1583")]
	pub security_classification_reason: Option<SecurityClassificationReason>,
	/// SecurityClassificationValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1584")]
	pub security_classification_value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SecurityClassificationReason {
	/// Fee
	#[serde(rename = "0")]
	Fee,
	/// Credit Controls
	#[serde(rename = "1")]
	CreditControls,
	/// Margin
	#[serde(rename = "2")]
	Margin,
	/// Entitlement / Eligibility
	#[serde(rename = "3")]
	EntitlementEligibility,
	/// Market Data
	#[serde(rename = "4")]
	MarketData,
	/// Account Selection
	#[serde(rename = "5")]
	AccountSelection,
	/// Delivery Process
	#[serde(rename = "6")]
	DeliveryProcess,
	/// Sector
	#[serde(rename = "7")]
	Sector,
}
