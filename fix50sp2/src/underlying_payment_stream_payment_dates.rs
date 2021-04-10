
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamPaymentDates {
	/// When specified, this overrides the business day convention defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified value would be specific to this instance of the underlying instrument's payment stream's payment dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40581")]
	pub underlying_payment_stream_payment_date_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified values would be specific to this instance of the underlying instrument's payment stream's payment dates.
	#[serde(flatten)]
	pub underlying_payment_stream_payment_date_business_center_grp: Option<super::underlying_payment_stream_payment_date_business_center_grp::UnderlyingPaymentStreamPaymentDateBusinessCenterGrp>,
	/// Conditionally required when UnderlyingPaymentStreamPaymentFrequencyUnit(40584) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40583")]
	pub underlying_payment_stream_payment_frequency_period: Option<i32>,
	/// Conditionally required when UnderlyingPaymentStreamPaymentFrequencyPeriod(40583) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40584")]
	pub underlying_payment_stream_payment_frequency_unit: Option<String>,
	/// UnderlyingPaymentStreamPaymentRollConvention
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40585")]
	pub underlying_payment_stream_payment_roll_convention: Option<String>,
	/// UnderlyingPaymentStreamFirstPaymentDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40586")]
	pub underlying_payment_stream_first_payment_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// UnderlyingPaymentStreamLastRegularPaymentDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40587")]
	pub underlying_payment_stream_last_regular_payment_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// UnderlyingPaymentStreamPaymentDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40588")]
	pub underlying_payment_stream_payment_date_relative_to: Option<i32>,
	/// Conditionally required when UnderlyingPaymentStreamPaymentDateOffsetUnit(40590) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40589")]
	pub underlying_payment_stream_payment_date_offset_period: Option<i32>,
	/// Conditionally required when UnderlyingPaymentStreamPaymentDateOffsetPeriod(40589) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40590")]
	pub underlying_payment_stream_payment_date_offset_unit: Option<String>,
	/// UnderlyingPaymentStreamPaymentDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40591")]
	pub underlying_payment_stream_payment_date_offset_day_type: Option<i32>,
	/// UnderlyingPaymentStreamPaymentDateGrp
	#[serde(flatten)]
	pub underlying_payment_stream_payment_date_grp: Option<super::underlying_payment_stream_payment_date_grp::UnderlyingPaymentStreamPaymentDateGrp>,
	/// UnderlyingPaymentStreamMasterAgreementPaymentDatesIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41940")]
	pub underlying_payment_stream_master_agreement_payment_dates_indicator: Option<fix_common::Boolean>,
	/// UnderlyingPaymentStreamFinalPricePaymentDate
	#[serde(flatten)]
	pub underlying_payment_stream_final_price_payment_date: Option<super::underlying_payment_stream_final_price_payment_date::UnderlyingPaymentStreamFinalPricePaymentDate>,
}
