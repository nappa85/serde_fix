
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamPaymentDates {
	/// When specified, this overrides the business day convention defined in the DateAdjustment component in Instrument. The specified
	/// value would be specific to this instance of the payment stream's payment dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40751")]
	pub payment_stream_payment_date_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the DateAdjustment component in Instrument. The specified values
	/// would be specific to this instance of the payment stream's payment dates.
	#[serde(flatten)]
	pub payment_stream_payment_date_business_center_grp: Option<super::payment_stream_payment_date_business_center_grp::PaymentStreamPaymentDateBusinessCenterGrp>,
	/// Conditionally required when PaymentStreamPaymentFrequencyUnit(40754) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40753")]
	pub payment_stream_payment_frequency_period: Option<i32>,
	/// Conditionally required when PaymentStreamPaymentFrequencyPeriod(40753) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40754")]
	pub payment_stream_payment_frequency_unit: Option<PaymentStreamPaymentFrequencyUnit>,
	/// PaymentStreamPaymentRollConvention
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40755")]
	pub payment_stream_payment_roll_convention: Option<String>,
	/// PaymentStreamFirstPaymentDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40756")]
	pub payment_stream_first_payment_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// PaymentStreamLastRegularPaymentDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40757")]
	pub payment_stream_last_regular_payment_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// PaymentStreamPaymentDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40758")]
	pub payment_stream_payment_date_relative_to: Option<i32>,
	/// Conditionally required when PaymentStreamPaymentDateOffsetUnit(40760) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40759")]
	pub payment_stream_payment_date_offset_period: Option<i32>,
	/// Conditionally required when PaymentStreamPaymentDateOffsetPeriod(40759) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40760")]
	pub payment_stream_payment_date_offset_unit: Option<PaymentStreamPaymentDateOffsetUnit>,
	/// PaymentStreamPaymentDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40920")]
	pub payment_stream_payment_date_offset_day_type: Option<PaymentStreamPaymentDateOffsetDayType>,
	/// PaymentStreamPaymentDateGrp
	#[serde(flatten)]
	pub payment_stream_payment_date_grp: Option<super::payment_stream_payment_date_grp::PaymentStreamPaymentDateGrp>,
	/// PaymentStreamMasterAgreementPaymentDatesIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41223")]
	pub payment_stream_master_agreement_payment_dates_indicator: Option<fix_common::Boolean>,
	/// PaymentStreamFinalPricePaymentDate
	#[serde(flatten)]
	pub payment_stream_final_price_payment_date: Option<super::payment_stream_final_price_payment_date::PaymentStreamFinalPricePaymentDate>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PaymentStreamPaymentFrequencyUnit {
	/// Day
	#[serde(rename = "D")]
	Day,
	/// Week
	#[serde(rename = "Wk")]
	Week,
	/// Month
	#[serde(rename = "Mo")]
	Month,
	/// Year
	#[serde(rename = "Yr")]
	Year,
	/// Term
	#[serde(rename = "T")]
	Term,
}

impl Default for PaymentStreamPaymentFrequencyUnit {
	fn default() -> Self {
		PaymentStreamPaymentFrequencyUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PaymentStreamPaymentDateOffsetUnit {
	/// Day
	#[serde(rename = "D")]
	Day,
	/// Week
	#[serde(rename = "Wk")]
	Week,
	/// Month
	#[serde(rename = "Mo")]
	Month,
	/// Year
	#[serde(rename = "Yr")]
	Year,
}

impl Default for PaymentStreamPaymentDateOffsetUnit {
	fn default() -> Self {
		PaymentStreamPaymentDateOffsetUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PaymentStreamPaymentDateOffsetDayType {
	/// Business
	#[serde(rename = "0")]
	Business,
	/// Calendar
	#[serde(rename = "1")]
	Calendar,
	/// Commodity business
	#[serde(rename = "2")]
	CommodityBusiness,
	/// Currency business
	#[serde(rename = "3")]
	CurrencyBusiness,
	/// Exchange business
	#[serde(rename = "4")]
	ExchangeBusiness,
	/// Scheduled trading day
	#[serde(rename = "5")]
	ScheduledTradingDay,
}

impl Default for PaymentStreamPaymentDateOffsetDayType {
	fn default() -> Self {
		PaymentStreamPaymentDateOffsetDayType::Business
	}
}
