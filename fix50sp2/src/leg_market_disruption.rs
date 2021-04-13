
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegMarketDisruption {
	/// LegMarketDisruptionProvision
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41462")]
	pub leg_market_disruption_provision: Option<LegMarketDisruptionProvision>,
	/// LegMarketDisruptionEventGrp
	#[serde(flatten)]
	pub leg_market_disruption_event_grp: Option<super::leg_market_disruption_event_grp::LegMarketDisruptionEventGrp>,
	/// LegMarketDisruptionFallbackProvision
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41463")]
	pub leg_market_disruption_fallback_provision: Option<LegMarketDisruptionFallbackProvision>,
	/// LegMarketDisruptionFallbackGrp
	#[serde(flatten)]
	pub leg_market_disruption_fallback_grp: Option<super::leg_market_disruption_fallback_grp::LegMarketDisruptionFallbackGrp>,
	/// LegMarketDisruptionFallbackReferencePriceGrp
	#[serde(flatten)]
	pub leg_market_disruption_fallback_reference_price_grp: Option<super::leg_market_disruption_fallback_reference_price_grp::LegMarketDisruptionFallbackReferencePriceGrp>,
	/// LegMarketDisruptionMaximumDays
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41464")]
	pub leg_market_disruption_maximum_days: Option<i32>,
	/// If specified, the disruption event should be specified in LegMarketDisruptionEventGrp.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41465")]
	pub leg_market_disruption_materiality_percentage: Option<f32>,
	/// Applicable only when LegMarketDisruptionEvent(41468)='DeMinimisTrading'.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41466")]
	pub leg_market_disruption_minimum_futures_contracts: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegMarketDisruptionProvision {
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

impl Default for LegMarketDisruptionProvision {
	fn default() -> Self {
		LegMarketDisruptionProvision::NotApplicable
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegMarketDisruptionFallbackProvision {
	/// As specified in master agreement
	#[serde(rename = "0")]
	AsSpecifiedInMasterAgreement,
	/// As specified in confirmation
	#[serde(rename = "1")]
	AsSpecifiedInConfirmation,
}

impl Default for LegMarketDisruptionFallbackProvision {
	fn default() -> Self {
		LegMarketDisruptionFallbackProvision::AsSpecifiedInMasterAgreement
	}
}
