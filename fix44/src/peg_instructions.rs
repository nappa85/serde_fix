
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PegInstructions {
	/// Amount (signed) added to the peg for a pegged order in the context of the <a href="tag_836_PegOffsetType.html" target="bottom">PegOffsetType&nbsp;(836)</a>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "211")]
	pub peg_offset_value: Option<f64>,
	/// Describes whether peg is static/fixed or floats
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "835")]
	pub peg_move_type: Option<PegMoveType>,
	/// Type of Peg Offset (e.g. price offset, tick offset etc)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "836")]
	pub peg_offset_type: Option<PegOffsetType>,
	/// Specifies nature of resulting pegged price (e.g. or better limit, strict limit etc)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "837")]
	pub peg_limit_type: Option<PegLimitType>,
	/// If the calculated peg price is not a valid tick price, specifies how to round the price (e.g. be more or less aggressive)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "838")]
	pub peg_round_direction: Option<PegRoundDirection>,
	/// The scope of the "related to" price of the peg (e.g. local, global etc)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "840")]
	pub peg_scope: Option<PegScope>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PegMoveType {
	/// Floating (default)
	#[serde(rename = "0")]
	Floating,
	/// Fixed
	#[serde(rename = "1")]
	Fixed,
}

impl Default for PegMoveType {
	fn default() -> Self {
		PegMoveType::Floating
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PegOffsetType {
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

impl Default for PegOffsetType {
	fn default() -> Self {
		PegOffsetType::Price
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PegLimitType {
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

impl Default for PegLimitType {
	fn default() -> Self {
		PegLimitType::OrBetterPriceImprovementAllowed
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PegRoundDirection {
	/// More aggressive - on a buy order round the price up round up to the nearest tick, on a sell round down to the nearest tick
	#[serde(rename = "1")]
	MoreAggressiveOnABuyOrderRoundThePriceUpRoundUpToTheNearestTickOnASellRoundDownToTheNearestTick,
	/// More passive - on a buy order round down to nearest tick on a sell order round up to nearest tick
	#[serde(rename = "2")]
	MorePassiveOnABuyOrderRoundDownToNearestTickOnASellOrderRoundUpToNearestTick,
}

impl Default for PegRoundDirection {
	fn default() -> Self {
		PegRoundDirection::MoreAggressiveOnABuyOrderRoundThePriceUpRoundUpToTheNearestTickOnASellRoundDownToTheNearestTick
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PegScope {
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

impl Default for PegScope {
	fn default() -> Self {
		PegScope::Local
	}
}
