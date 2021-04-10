
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MaturityRules {
	/// Number of maturity rule entries. This block specifies the rules for determining how new strikes should be listed within the
	/// stated price range of the underlying instrument.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1236")]
	pub maturity_rules: Option<fix_common::RepeatingValues<MaturityRule>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MaturityRule {
	/// Allows maturity rule to be referenced via an identifier so that rules do not need to be explicitly enumerated.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1222")]
	pub maturity_rule_id: Option<String>,
	/// Format used to generate the MMY for each option contract:
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1303")]
	pub maturity_month_year_format: Option<MaturityMonthYearFormat>,
	/// enumeration specifying the increment unit:.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1302")]
	pub maturity_month_year_increment_units: Option<MaturityMonthYearIncrementUnits>,
	/// Starting maturity for the range to which the StrikeIncrement applies. Price refers to the price of the underlying.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1241")]
	pub start_maturity_month_year: Option<fix_common::MonthYear>,
	/// Ending maturity monthy year to which the StrikeIncrement applies. Price refers to the price of the underlying.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1226")]
	pub end_maturity_month_year: Option<fix_common::MonthYear>,
	/// Value by which maturity month year should be incremented within the specified price range.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1229")]
	pub maturity_month_year_increment: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MaturityMonthYearFormat {
	/// YearMonth Only (default)
	#[serde(rename = "0")]
	YearMonthOnly,
	/// YearMonthDay
	#[serde(rename = "1")]
	YearMonthDay,
	/// YearMonthWeek
	#[serde(rename = "2")]
	YearMonthWeek,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MaturityMonthYearIncrementUnits {
	/// Months
	#[serde(rename = "0")]
	Months,
	/// Days
	#[serde(rename = "1")]
	Days,
	/// Weeks
	#[serde(rename = "2")]
	Weeks,
	/// Years
	#[serde(rename = "3")]
	Years,
}
