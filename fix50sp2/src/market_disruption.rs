
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MarketDisruption {
	/// MarketDisruptionProvision
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41087")]
	pub market_disruption_provision: Option<MarketDisruptionProvision>,
	/// MarketDisruptionEventGrp
	#[serde(flatten)]
	pub market_disruption_event_grp: Option<super::market_disruption_event_grp::MarketDisruptionEventGrp>,
	/// MarketDisruptionFallbackProvision
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41088")]
	pub market_disruption_fallback_provision: Option<MarketDisruptionFallbackProvision>,
	/// MarketDisruptionFallbackGrp
	#[serde(flatten)]
	pub market_disruption_fallback_grp: Option<super::market_disruption_fallback_grp::MarketDisruptionFallbackGrp>,
	/// MarketDisruptionFallbackReferencePriceGrp
	#[serde(flatten)]
	pub market_disruption_fallback_reference_price_grp: Option<super::market_disruption_fallback_reference_price_grp::MarketDisruptionFallbackReferencePriceGrp>,
	/// MarketDisruptionMaximumDays
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41089")]
	pub market_disruption_maximum_days: Option<i32>,
	/// If specified, the disruption event should be specified in MarketDisruptionEventGrp.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41090")]
	pub market_disruption_materiality_percentage: Option<f32>,
	/// Applicable only when MarketDisruptionEvent(41093)='DeMinimisTrading'.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41091")]
	pub market_disruption_minimum_futures_contracts: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MarketDisruptionProvision {
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MarketDisruptionFallbackProvision {
	/// As specified in master agreement
	#[serde(rename = "0")]
	AsSpecifiedInMasterAgreement,
	/// As specified in confirmation
	#[serde(rename = "1")]
	AsSpecifiedInConfirmation,
}
