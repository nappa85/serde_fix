
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegDividendFXTriggerDate {
	/// LegDividendFXTriggerDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42357")]
	pub leg_dividend_fx_trigger_date_relative_to: Option<i32>,
	/// Conditionally required when LegDividendFXTriggerDateOffsetUnit(42359) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42358")]
	pub leg_dividend_fx_trigger_date_offset_period: Option<i32>,
	/// Conditionally required when LegDividendFXTriggerDateOffsetPeriod(42358) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42359")]
	pub leg_dividend_fx_trigger_date_offset_unit: Option<String>,
	/// LegDividendFXTriggerDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42360")]
	pub leg_dividend_fx_trigger_date_offset_day_type: Option<i32>,
	/// LegDividendFXTriggerDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42361")]
	pub leg_dividend_fx_trigger_date_unadjusted: Option<crate::entities::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the LegDateAdjustment component in InstrumentLeg. The
	/// value would be specific to this instance of LegDividendFXTriggerDate.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42362")]
	pub leg_dividend_fx_trigger_date_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the LegDateAdjustment component in InstrumentLeg. The values
	/// would be specific to this instance of LegDividendFXTriggerDate.
	#[serde(flatten)]
	pub leg_dividend_fx_trigger_date_business_center_grp: Option<super::leg_dividend_fx_trigger_date_business_center_grp::LegDividendFXTriggerDateBusinessCenterGrp>,
	/// LegDividendFXTriggerDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42363")]
	pub leg_dividend_fx_trigger_date_adjusted: Option<crate::entities::LocalMktDate>,
}
