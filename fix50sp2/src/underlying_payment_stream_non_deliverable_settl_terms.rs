
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamNonDeliverableSettlTerms {
	/// UnderlyingPaymentStreamNonDeliverableRefCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40648")]
	pub underlying_payment_stream_non_deliverable_ref_currency: Option<String>,
	/// When specified, this overrides the business day convention defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified value would be specific to this instance of the underlying instrument's non-deliverable settlement terms.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40649")]
	pub underlying_payment_stream_non_deliverable_fixing_dates_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified values would be specific to this instance of the underlying instrument's non-deliverable settlement terms.
	#[serde(flatten)]
	pub underlying_payment_stream_non_deliverable_fixing_dates_business_center_grp: Option<super::underlying_payment_stream_non_deliverable_fixing_dates_business_center_grp::UnderlyingPaymentStreamNonDeliverableFixingDatesBusinessCenterGrp>,
	/// UnderlyingPaymentStreamNonDeliverableFixingDatesRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40651")]
	pub underlying_payment_stream_non_deliverable_fixing_dates_relative_to: Option<i32>,
	/// Conditionally required when UnderlyingPaymentStreamNonDeliverableFixingDatesOffsetUnit(40653) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40652")]
	pub underlying_payment_stream_non_deliverable_fixing_dates_offset_period: Option<i32>,
	/// Conditionally required when UnderlyingPaymentStreamNonDeliverableFixingDatesOffsetPeriod(40652) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40653")]
	pub underlying_payment_stream_non_deliverable_fixing_dates_offset_unit: Option<String>,
	/// UnderlyingPaymentStreamNonDeliverableFixingDatesOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40654")]
	pub underlying_payment_stream_non_deliverable_fixing_dates_offset_day_type: Option<i32>,
	/// UnderlyingPaymentStreamNonDeliverableSettlRateSource
	#[serde(flatten)]
	pub underlying_payment_stream_non_deliverable_settl_rate_source: Option<super::underlying_payment_stream_non_deliverable_settl_rate_source::UnderlyingPaymentStreamNonDeliverableSettlRateSource>,
	/// UnderlyingPaymentStreamNonDeliverableFixingDateGrp
	#[serde(flatten)]
	pub underlying_payment_stream_non_deliverable_fixing_date_grp: Option<super::underlying_payment_stream_non_deliverable_fixing_date_grp::UnderlyingPaymentStreamNonDeliverableFixingDateGrp>,
	/// UnderlyingSettlRateDisruptionFallbackGrp
	#[serde(flatten)]
	pub underlying_settl_rate_disruption_fallback_grp: Option<super::underlying_settl_rate_disruption_fallback_grp::UnderlyingSettlRateDisruptionFallbackGrp>,
}
