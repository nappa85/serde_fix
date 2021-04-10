
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StrikeRules {
	/// Number of strike rule entries. This block specifies the rules for determining how new strikes should be listed within the
	/// stated price range of the underlying instrument.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1201")]
	pub strike_rules: Option<crate::entities::RepeatingValues<StrikeRule>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StrikeRule {
	/// Allows strike rule to be referenced via an identifier so that rules do not need to be explicitly enumerated.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1223")]
	pub strike_rule_id: Option<String>,
	/// Starting price for the range to which the StrikeIncrement applies. Price refers to the price of the underlying.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1202")]
	pub start_strike_px_range: Option<f64>,
	/// Ending price of the range to which the StrikeIncrement applies. Price refers to the price of the underlying.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1203")]
	pub end_strike_px_range: Option<f64>,
	/// Value by which strike price should be incremented within the specified price.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1204")]
	pub strike_increment: Option<f64>,
	/// Enumeration that represents the exercise style for a class of options. Same values as ExerciseStyle.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1304")]
	pub strike_exercise_style: Option<StrikeExerciseStyle>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StrikeExerciseStyle {
	/// European
	#[serde(rename = "0")]
	European,
	/// American
	#[serde(rename = "1")]
	American,
	/// Bermuda
	#[serde(rename = "2")]
	Bermuda,
	/// Other
	#[serde(rename = "99")]
	Other,
}
