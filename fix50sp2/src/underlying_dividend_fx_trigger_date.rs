
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingDividendFXTriggerDate {
	/// UnderlyingDividendFXTriggerDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42846")]
	pub underlying_dividend_fx_trigger_date_relative_to: Option<i32>,
	/// Conditionally required when UnderlyingDividendFXTriggerDateOffsetUnit(42848) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42847")]
	pub underlying_dividend_fx_trigger_date_offset_period: Option<i32>,
	/// Conditionally required when UnderlyingDividendFXTriggerDateOffsetPeriod(42847) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42848")]
	pub underlying_dividend_fx_trigger_date_offset_unit: Option<String>,
	/// UnderlyingDividendFXTriggerDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42849")]
	pub underlying_dividend_fx_trigger_date_offset_day_type: Option<i32>,
	/// UnderlyingDividendFXTriggerDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42850")]
	pub underlying_dividend_fx_trigger_date_unadjusted: Option<crate::entities::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The value would be specific to this instance of UnderlyingDividendFXTriggerDate.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42851")]
	pub underlying_dividend_fx_trigger_date_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The values would be specific to this instance of UnderlyingDividendFXTriggerDate.
	#[serde(flatten)]
	pub underlying_dividend_fx_trigger_date_business_center_grp: Option<super::underlying_dividend_fx_trigger_date_business_center_grp::UnderlyingDividendFXTriggerDateBusinessCenterGrp>,
	/// UnderlyingDividendFXTriggerDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42852")]
	pub underlying_dividend_fx_trigger_date_adjusted: Option<crate::entities::LocalMktDate>,
}
