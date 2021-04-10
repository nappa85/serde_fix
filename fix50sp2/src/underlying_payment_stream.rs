
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStream {
	/// UnderlyingPaymentStreamType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40568")]
	pub underlying_payment_stream_type: Option<i32>,
	/// UnderlyingPaymentStreamMarketRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40569")]
	pub underlying_payment_stream_market_rate: Option<i32>,
	/// UnderlyingPaymentStreamDelayIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40570")]
	pub underlying_payment_stream_delay_indicator: Option<crate::entities::Boolean>,
	/// UnderlyingPaymentStreamSettlCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40571")]
	pub underlying_payment_stream_settl_currency: Option<String>,
	/// UnderlyingPaymentStreamDayCount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40572")]
	pub underlying_payment_stream_day_count: Option<i32>,
	/// UnderlyingPaymentStreamAccrualDays
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40573")]
	pub underlying_payment_stream_accrual_days: Option<i32>,
	/// UnderlyingPaymentStreamDiscountType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40574")]
	pub underlying_payment_stream_discount_type: Option<i32>,
	/// UnderlyingPaymentStreamDiscountRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40575")]
	pub underlying_payment_stream_discount_rate: Option<f32>,
	/// UnderlyingPaymentStreamDiscountRateDayCount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40576")]
	pub underlying_payment_stream_discount_rate_day_count: Option<i32>,
	/// UnderlyingPaymentStreamCompoundingMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40577")]
	pub underlying_payment_stream_compounding_method: Option<i32>,
	/// UnderlyingPaymentStreamInitialPrincipalExchangeIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40578")]
	pub underlying_payment_stream_initial_principal_exchange_indicator: Option<crate::entities::Boolean>,
	/// UnderlyingPaymentStreamInterimPrincipalExchangeIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40579")]
	pub underlying_payment_stream_interim_principal_exchange_indicator: Option<crate::entities::Boolean>,
	/// UnderlyingPaymentStreamFinalPrincipalExchangeIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40580")]
	pub underlying_payment_stream_final_principal_exchange_indicator: Option<crate::entities::Boolean>,
	/// UnderlyingPaymentStreamPaymentDates
	#[serde(flatten)]
	pub underlying_payment_stream_payment_dates: Option<super::underlying_payment_stream_payment_dates::UnderlyingPaymentStreamPaymentDates>,
	/// UnderlyingPaymentStreamResetDates
	#[serde(flatten)]
	pub underlying_payment_stream_reset_dates: Option<super::underlying_payment_stream_reset_dates::UnderlyingPaymentStreamResetDates>,
	/// UnderlyingPaymentStreamFixedRate
	#[serde(flatten)]
	pub underlying_payment_stream_fixed_rate: Option<super::underlying_payment_stream_fixed_rate::UnderlyingPaymentStreamFixedRate>,
	/// UnderlyingPaymentStreamFloatingRate
	#[serde(flatten)]
	pub underlying_payment_stream_floating_rate: Option<super::underlying_payment_stream_floating_rate::UnderlyingPaymentStreamFloatingRate>,
	/// UnderlyingPaymentStreamNonDeliverableSettlTerms
	#[serde(flatten)]
	pub underlying_payment_stream_non_deliverable_settl_terms: Option<super::underlying_payment_stream_non_deliverable_settl_terms::UnderlyingPaymentStreamNonDeliverableSettlTerms>,
	/// UnderlyingPaymentStreamFlatRateIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41897")]
	pub underlying_payment_stream_flat_rate_indicator: Option<crate::entities::Boolean>,
	/// UnderlyingPaymentStreamFlatRateAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41898")]
	pub underlying_payment_stream_flat_rate_amount: Option<f64>,
	/// UnderlyingPaymentStreamFlatRateCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41899")]
	pub underlying_payment_stream_flat_rate_currency: Option<String>,
	/// UnderlyingPaymentStreamMaximumPaymentAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41900")]
	pub underlying_payment_stream_maximum_payment_amount: Option<f64>,
	/// UnderlyingPaymentStreamMaximumPaymentCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41901")]
	pub underlying_payment_stream_maximum_payment_currency: Option<String>,
	/// UnderlyingPaymentStreamMaximumTransactionAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41902")]
	pub underlying_payment_stream_maximum_transaction_amount: Option<f64>,
	/// UnderlyingPaymentStreamMaximumTransactionCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41903")]
	pub underlying_payment_stream_maximum_transaction_currency: Option<String>,
	/// UnderlyingPaymentStreamCashSettlIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42895")]
	pub underlying_payment_stream_cash_settl_indicator: Option<crate::entities::Boolean>,
	/// Mutually exclusive with UnderlyingPaymentStreamCompoundingFixedRate(42900) or the UnderlyingPaymentStreamCompoundingFloatingRate
	/// component.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42896")]
	pub underlying_payment_stream_compounding_xid_ref: Option<String>,
	/// UnderlyingPaymentStreamCompoundingSpread
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42897")]
	pub underlying_payment_stream_compounding_spread: Option<f64>,
	/// UnderlyingPaymentStreamInterpolationMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42898")]
	pub underlying_payment_stream_interpolation_method: Option<i32>,
	/// UnderlyingPaymentStreamInterpolationPeriod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42899")]
	pub underlying_payment_stream_interpolation_period: Option<i32>,
	/// Mutually exclusive with UnderlyingPaymentStreamCompoundingXIDRef(42896) or the UnderlyingPaymentStreamCompoundingFloatingRate
	/// component.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42900")]
	pub underlying_payment_stream_compounding_fixed_rate: Option<f64>,
	/// Mutually exclusive with UnderlyingPaymentStreamCompoundingFixedRate(42900) or the UnderlyingPaymentStreamCompoundingXIDRef(42896).
	#[serde(flatten)]
	pub underlying_payment_stream_compounding_floating_rate: Option<super::underlying_payment_stream_compounding_floating_rate::UnderlyingPaymentStreamCompoundingFloatingRate>,
	/// UnderlyingPaymentStreamCompoundingDates
	#[serde(flatten)]
	pub underlying_payment_stream_compounding_dates: Option<super::underlying_payment_stream_compounding_dates::UnderlyingPaymentStreamCompoundingDates>,
	/// May be used to specify a count method not listed in UnderlyingPaymentStreamDayCount(40572).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43107")]
	pub underlying_payment_stream_other_day_count: Option<String>,
}
