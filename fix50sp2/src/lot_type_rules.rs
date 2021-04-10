
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LotTypeRules {
	/// Number of Lot Types
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1234")]
	pub lot_type_rules: Option<fix_common::RepeatingValues<LotTypeRule>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LotTypeRule {
	/// Defines the lot type assigned to the order. Use as an alternate to <a href="tag_561_RoundLot.html" target="bottom">RoundLot&nbsp;(561)</a> . To be used with <a href="tag_1231_MinLotSize.html" target="bottom">MinLotSize&nbsp;(1231)</a> . <a href="tag_1093_LotType.html" target="bottom">LotType&nbsp;(1093)</a> + <a href="tag_1231_MinLotSize.html" target="bottom">MinLotSize&nbsp;(1231)</a> ( max is next level minus 1)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1093")]
	pub lot_type: Option<LotType>,
	/// Minimum lot size allowed based on lot type specified in <a href="tag_1093_LotType.html" target="bottom">LotType&nbsp;(1093)</a>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1231")]
	pub min_lot_size: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LotType {
	/// Odd Lot
	#[serde(rename = "1")]
	OddLot,
	/// Round Lot
	#[serde(rename = "2")]
	RoundLot,
	/// Block Lot
	#[serde(rename = "3")]
	BlockLot,
	/// Round lot based upon <a href="tag_996_UnitOfMeasure.html" target="bottom">UnitOfMeasure&nbsp;(996)</a>
	#[serde(rename = "4")]
	RoundLotBasedUponAHrefTag996UnitOfMeasureHtmlTargetBottomUnitOfMeasureNbspA,
}
