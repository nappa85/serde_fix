
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamNonDeliverableSettlTerms {
	/// PaymentStreamNonDeliverableRefCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40817")]
	pub payment_stream_non_deliverable_ref_currency: Option<String>,
	/// When specified, this overrides the business day convention defined in the DateAdjustment component in Instrument. The specified
	/// value would be specific to this instance of the payment stream's non-deliverable fixing dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40818")]
	pub payment_stream_non_deliverable_fixing_dates_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the DateAdjustment component in Instrument. The specified values
	/// would be specific to this instance of the payment stream's non-deliverable fixing dates.
	#[serde(flatten)]
	pub payment_stream_non_deliverable_fixing_dates_business_center_grp: Option<super::payment_stream_non_deliverable_fixing_dates_business_center_grp::PaymentStreamNonDeliverableFixingDatesBusinessCenterGrp>,
	/// PaymentStreamNonDeliverableFixingDatesRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40820")]
	pub payment_stream_non_deliverable_fixing_dates_relative_to: Option<i32>,
	/// Conditionally required when PaymentStreamNonDeliverableFixingDatesOffsetUnit(40822) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40821")]
	pub payment_stream_non_deliverable_fixing_dates_offset_period: Option<i32>,
	/// Conditionally required when PaymentStreamNonDeliverableFixingDatesOffsetPeriod(40821) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40822")]
	pub payment_stream_non_deliverable_fixing_dates_offset_unit: Option<String>,
	/// PaymentStreamNonDeliverableFixingDatesOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40823")]
	pub payment_stream_non_deliverable_fixing_dates_offset_day_type: Option<i32>,
	/// PaymentStreamNonDeliverableSettlRateSource
	#[serde(flatten)]
	pub payment_stream_non_deliverable_settl_rate_source: Option<super::payment_stream_non_deliverable_settl_rate_source::PaymentStreamNonDeliverableSettlRateSource>,
	/// PaymentStreamNonDeliverableFixingDateGrp
	#[serde(flatten)]
	pub payment_stream_non_deliverable_fixing_date_grp: Option<super::payment_stream_non_deliverable_fixing_date_grp::PaymentStreamNonDeliverableFixingDateGrp>,
	/// SettlRateDisruptionFallbackGrp
	#[serde(flatten)]
	pub settl_rate_disruption_fallback_grp: Option<super::settl_rate_disruption_fallback_grp::SettlRateDisruptionFallbackGrp>,
}
