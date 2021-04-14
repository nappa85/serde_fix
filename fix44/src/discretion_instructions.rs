
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DiscretionInstructions {
	/// What the discretionary price is related to (e.g. primary price, display price etc)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "388")]
	pub discretion_inst: Option<DiscretionInst>,
	/// Amount (signed) added to the "related to" price specified via <a href="tag_388_DiscretionInst.html" target="bottom">DiscretionInst&nbsp;(388)</a> , in the context of <a href="tag_842_DiscretionOffsetType.html" target="bottom">DiscretionOffsetType&nbsp;(842)</a>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "389")]
	pub discretion_offset_value: Option<f64>,
	/// Describes whether discretion price is static/fixed or floats
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "841")]
	pub discretion_move_type: Option<DiscretionMoveType>,
	/// Type of Discretion Offset (e.g. price offset, tick offset etc)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "842")]
	pub discretion_offset_type: Option<DiscretionOffsetType>,
	/// Specifies the nature of the resulting discretion price (e.g. or better limit, strict limit etc)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "843")]
	pub discretion_limit_type: Option<DiscretionLimitType>,
	/// If the calculated discretion price is not a valid tick price, specifies how to round the price (e.g. to be more or less aggressive)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "844")]
	pub discretion_round_direction: Option<DiscretionRoundDirection>,
	/// The scope of "related to" price of the discretion (e.g. local, global etc)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "846")]
	pub discretion_scope: Option<DiscretionScope>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DiscretionInst {
	/// Related to displayed price
	#[serde(rename = "0")]
	RelatedToDisplayedPrice,
	/// Related to market price
	#[serde(rename = "1")]
	RelatedToMarketPrice,
	/// Related to primary price
	#[serde(rename = "2")]
	RelatedToPrimaryPrice,
	/// Related to local primary price
	#[serde(rename = "3")]
	RelatedToLocalPrimaryPrice,
	/// Related to midpoint price
	#[serde(rename = "4")]
	RelatedToMidpointPrice,
	/// Related to last trade price
	#[serde(rename = "5")]
	RelatedToLastTradePrice,
	/// Related to VWAP
	#[serde(rename = "6")]
	RelatedToVwap,
}

impl Default for DiscretionInst {
	fn default() -> Self {
		DiscretionInst::RelatedToDisplayedPrice
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DiscretionMoveType {
	/// Floating (default)
	#[serde(rename = "0")]
	Floating,
	/// Fixed
	#[serde(rename = "1")]
	Fixed,
}

impl Default for DiscretionMoveType {
	fn default() -> Self {
		DiscretionMoveType::Floating
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DiscretionOffsetType {
	/// Price (default)
	#[serde(rename = "0")]
	Price,
	/// Basis Points
	#[serde(rename = "1")]
	BasisPoints,
	/// Ticks
	#[serde(rename = "2")]
	Ticks,
	/// Price Tier / Level
	#[serde(rename = "3")]
	PriceTierLevel,
}

impl Default for DiscretionOffsetType {
	fn default() -> Self {
		DiscretionOffsetType::Price
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DiscretionLimitType {
	/// Or better (default) - price improvement allowed
	#[serde(rename = "0")]
	OrBetterPriceImprovementAllowed,
	/// Strict - limit is a strict limit
	#[serde(rename = "1")]
	StrictLimitIsAStrictLimit,
	/// Or worse - for a buy the peg limit is a minimum and for a sell the peg limit is a maximum (for use for orders which have a
	/// price range)
	#[serde(rename = "2")]
	OrWorseForABuyThePegLimitIsAMinimumAndForASellThePegLimitIsAMaximum,
}

impl Default for DiscretionLimitType {
	fn default() -> Self {
		DiscretionLimitType::OrBetterPriceImprovementAllowed
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DiscretionRoundDirection {
	/// More aggressive - on a buy order round the price up round up to the nearest tick, on a sell round down to the nearest tick
	#[serde(rename = "1")]
	MoreAggressiveOnABuyOrderRoundThePriceUpRoundUpToTheNearestTickOnASellRoundDownToTheNearestTick,
	/// More passive - on a buy order round down to nearest tick on a sell order round up to nearest tick
	#[serde(rename = "2")]
	MorePassiveOnABuyOrderRoundDownToNearestTickOnASellOrderRoundUpToNearestTick,
}

impl Default for DiscretionRoundDirection {
	fn default() -> Self {
		DiscretionRoundDirection::MoreAggressiveOnABuyOrderRoundThePriceUpRoundUpToTheNearestTickOnASellRoundDownToTheNearestTick
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DiscretionScope {
	/// Local (Exchange, ECN, ATS)
	#[serde(rename = "1")]
	Local,
	/// National
	#[serde(rename = "2")]
	National,
	/// Global
	#[serde(rename = "3")]
	Global,
	/// National excluding local
	#[serde(rename = "4")]
	NationalExcludingLocal,
}

impl Default for DiscretionScope {
	fn default() -> Self {
		DiscretionScope::Local
	}
}
