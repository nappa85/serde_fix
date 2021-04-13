
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingMarketDisruption {
	/// UnderlyingMarketDisruptionProvision
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41859")]
	pub underlying_market_disruption_provision: Option<UnderlyingMarketDisruptionProvision>,
	/// UnderlyingMarketDisruptionEventGrp
	#[serde(flatten)]
	pub underlying_market_disruption_event_grp: Option<super::underlying_market_disruption_event_grp::UnderlyingMarketDisruptionEventGrp>,
	/// UnderlyingMarketDisruptionFallbackProvision
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41860")]
	pub underlying_market_disruption_fallback_provision: Option<UnderlyingMarketDisruptionFallbackProvision>,
	/// UnderlyingMarketDisruptionFallbackGrp
	#[serde(flatten)]
	pub underlying_market_disruption_fallback_grp: Option<super::underlying_market_disruption_fallback_grp::UnderlyingMarketDisruptionFallbackGrp>,
	/// UnderlyingMarketDisruptionFallbackReferencePriceGrp
	#[serde(flatten)]
	pub underlying_market_disruption_fallback_reference_price_grp: Option<super::underlying_market_disruption_fallback_reference_price_grp::UnderlyingMarketDisruptionFallbackReferencePriceGrp>,
	/// UnderlyingMarketDisruptionMaximumDays
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41861")]
	pub underlying_market_disruption_maximum_days: Option<i32>,
	/// If specified, the disruption event should be specified in UnderlyingMarketDisruptionEventGrp.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41862")]
	pub underlying_market_disruption_materiality_percentage: Option<f32>,
	/// Applicable only when UnderlyingMarketDisruptionEvent(41865)='DeMinimisTrading'.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41863")]
	pub underlying_market_disruption_minimum_futures_contracts: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingMarketDisruptionProvision {
	/// Not applicable
	#[serde(rename = "0")]
	NotApplicable,
	/// Applicable
	#[serde(rename = "1")]
	Applicable,
	/// As specified in master agreement
	#[serde(rename = "2")]
	AsSpecifiedInMasterAgreement,
	/// As specified in confirmation
	#[serde(rename = "3")]
	AsSpecifiedInConfirmation,
}

impl Default for UnderlyingMarketDisruptionProvision {
	fn default() -> Self {
		UnderlyingMarketDisruptionProvision::NotApplicable
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingMarketDisruptionFallbackProvision {
	/// As specified in master agreement
	#[serde(rename = "0")]
	AsSpecifiedInMasterAgreement,
	/// As specified in confirmation
	#[serde(rename = "1")]
	AsSpecifiedInConfirmation,
}

impl Default for UnderlyingMarketDisruptionFallbackProvision {
	fn default() -> Self {
		UnderlyingMarketDisruptionFallbackProvision::AsSpecifiedInMasterAgreement
	}
}
