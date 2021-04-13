
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegQuotStatGrp {
	/// NoLegs
	#[serde(rename = "555")]
	pub legs: fix_common::RepeatingValues<Leg>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Leg {
	/// Deprecated in FIX 5.0 SP1. The LegQty(687) field is deprecated. The use of LegOrderQty(685) is recommended instead
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "687")]
	pub leg_qty: Option<f64>,
	/// Quantity ordered for this leg as provided during order entry.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "685")]
	pub leg_order_qty: Option<f64>,
	/// LegSwapType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "690")]
	pub leg_swap_type: Option<LegSwapType>,
	/// LegSettlType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "587")]
	pub leg_settl_type: Option<LegSettlType>,
	/// LegSettlDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "588")]
	pub leg_settl_date: Option<fix_common::LocalMktDate>,
	/// LegMidPx
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2346")]
	pub leg_mid_px: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegSwapType {
	/// Par For Par
	#[serde(rename = "1")]
	ParForPar,
	/// Modified Duration
	#[serde(rename = "2")]
	ModifiedDuration,
	/// Risk
	#[serde(rename = "4")]
	Risk,
	/// Proceeds
	#[serde(rename = "5")]
	Proceeds,
}

impl Default for LegSwapType {
	fn default() -> Self {
		LegSwapType::ParForPar
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegSettlType {
	/// Regular / FX Spot settlement (T+1 or T+2 depending on currency)
	#[serde(rename = "0")]
	RegularFxSpotSettlement,
	/// Cash (TOD / T+0)
	#[serde(rename = "1")]
	Cash,
	/// Next Day (TOM / T+1)
	#[serde(rename = "2")]
	NextDay,
	/// T+2
	#[serde(rename = "3")]
	T2,
	/// T+3
	#[serde(rename = "4")]
	T3,
	/// T+4
	#[serde(rename = "5")]
	T4,
	/// Future
	#[serde(rename = "6")]
	Future,
	/// When And If Issued
	#[serde(rename = "7")]
	WhenAndIfIssued,
	/// Sellers Option
	#[serde(rename = "8")]
	SellersOption,
	/// T+5
	#[serde(rename = "9")]
	T5,
	/// Broken date - for FX expressing non-standard tenor, <a href="tag_64_SettlDate.html" target="bottom">SettlDate&nbsp;(64)</a> must be specified
	#[serde(rename = "B")]
	BrokenDateForFxExpressingNonStandardTenorAHrefTag64SettlDateHtmlTargetBottomSettlDateNbspAMustBeSpecified,
	/// FX Spot Next settlement (Spot+1, aka next day)
	#[serde(rename = "C")]
	FxSpotNextSettlement,
}

impl Default for LegSettlType {
	fn default() -> Self {
		LegSettlType::RegularFxSpotSettlement
	}
}
