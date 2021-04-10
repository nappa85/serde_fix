
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProtectionTermGrp {
	/// NoProtectionTerms
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40181")]
	pub protection_terms: Option<fix_common::RepeatingValues<ProtectionTerm>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProtectionTerm {
	/// Required if NoProtectionTerms(40181) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40182")]
	pub protection_term_notional: Option<f64>,
	/// ProtectionTermCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40183")]
	pub protection_term_currency: Option<String>,
	/// ProtectionTermSellerNotifies
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40184")]
	pub protection_term_seller_notifies: Option<fix_common::Boolean>,
	/// ProtectionTermBuyerNotifies
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40185")]
	pub protection_term_buyer_notifies: Option<fix_common::Boolean>,
	/// ProtectionTermEventBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40186")]
	pub protection_term_event_business_center: Option<String>,
	/// ProtectionTermStandardSources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40187")]
	pub protection_term_standard_sources: Option<fix_common::Boolean>,
	/// ProtectionTermEventMinimumSources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40188")]
	pub protection_term_event_minimum_sources: Option<i32>,
	/// ProtectionTermXID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40190")]
	pub protection_term_xid: Option<String>,
}
