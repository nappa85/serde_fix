
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProtectionTermGrp {
	/// NoLegProtectionTerms
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41616")]
	pub no_leg_protection_terms: Option<usize>,
	/// Required if NoLegProtectionTerms(41616) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41618")]
	pub leg_protection_term_notional: Option<f64>,
	/// LegProtectionTermCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41619")]
	pub leg_protection_term_currency: Option<String>,
	/// LegProtectionTermSellerNotifies
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41620")]
	pub leg_protection_term_seller_notifies: Option<fix_common::Boolean>,
	/// LegProtectionTermBuyerNotifies
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41621")]
	pub leg_protection_term_buyer_notifies: Option<fix_common::Boolean>,
	/// LegProtectionTermEventBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41622")]
	pub leg_protection_term_event_business_center: Option<String>,
	/// LegProtectionTermStandardSources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41623")]
	pub leg_protection_term_standard_sources: Option<fix_common::Boolean>,
	/// LegProtectionTermEventMinimumSources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41624")]
	pub leg_protection_term_event_minimum_sources: Option<i32>,
	/// LegProtectionTermEventNewsSourceGrp
	#[serde(flatten)]
	pub leg_protection_term_event_news_source_grp: Option<super::leg_protection_term_event_news_source_grp::LegProtectionTermEventNewsSourceGrp>,
	/// LegProtectionTermEventGrp
	#[serde(flatten)]
	pub leg_protection_term_event_grp: Option<super::leg_protection_term_event_grp::LegProtectionTermEventGrp>,
	/// LegProtectionTermObligationGrp
	#[serde(flatten)]
	pub leg_protection_term_obligation_grp: Option<super::leg_protection_term_obligation_grp::LegProtectionTermObligationGrp>,
	/// LegProtectionTermXID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41617")]
	pub leg_protection_term_xid: Option<String>,
}
