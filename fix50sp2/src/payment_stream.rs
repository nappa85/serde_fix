
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStream {
	/// PaymentStreamType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40738")]
	pub payment_stream_type: Option<PaymentStreamType>,
	/// PaymentStreamMarketRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40739")]
	pub payment_stream_market_rate: Option<i32>,
	/// PaymentStreamDelayIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40740")]
	pub payment_stream_delay_indicator: Option<crate::entities::Boolean>,
	/// PaymentStreamSettlCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40741")]
	pub payment_stream_settl_currency: Option<String>,
	/// PaymentStreamDayCount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40742")]
	pub payment_stream_day_count: Option<i32>,
	/// PaymentStreamAccrualDays
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40743")]
	pub payment_stream_accrual_days: Option<i32>,
	/// PaymentStreamDiscountType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40744")]
	pub payment_stream_discount_type: Option<PaymentStreamDiscountType>,
	/// PaymentStreamDiscountRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40745")]
	pub payment_stream_discount_rate: Option<f32>,
	/// PaymentStreamDiscountRateDayCount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40746")]
	pub payment_stream_discount_rate_day_count: Option<i32>,
	/// PaymentStreamCompoundingMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40747")]
	pub payment_stream_compounding_method: Option<PaymentStreamCompoundingMethod>,
	/// PaymentStreamInitialPrincipalExchangeIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40748")]
	pub payment_stream_initial_principal_exchange_indicator: Option<crate::entities::Boolean>,
	/// PaymentStreamInterimPrincipalExchangeIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40749")]
	pub payment_stream_interim_principal_exchange_indicator: Option<crate::entities::Boolean>,
	/// PaymentStreamFinalPrincipalExchangeIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40750")]
	pub payment_stream_final_principal_exchange_indicator: Option<crate::entities::Boolean>,
	/// PaymentStreamPaymentDates
	#[serde(flatten)]
	pub payment_stream_payment_dates: Option<super::payment_stream_payment_dates::PaymentStreamPaymentDates>,
	/// PaymentStreamResetDates
	#[serde(flatten)]
	pub payment_stream_reset_dates: Option<super::payment_stream_reset_dates::PaymentStreamResetDates>,
	/// PaymentStreamFixedRate
	#[serde(flatten)]
	pub payment_stream_fixed_rate: Option<super::payment_stream_fixed_rate::PaymentStreamFixedRate>,
	/// PaymentStreamFloatingRate
	#[serde(flatten)]
	pub payment_stream_floating_rate: Option<super::payment_stream_floating_rate::PaymentStreamFloatingRate>,
	/// PaymentStreamNonDeliverableSettlTerms
	#[serde(flatten)]
	pub payment_stream_non_deliverable_settl_terms: Option<super::payment_stream_non_deliverable_settl_terms::PaymentStreamNonDeliverableSettlTerms>,
	/// PaymentStreamFlatRateIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41180")]
	pub payment_stream_flat_rate_indicator: Option<crate::entities::Boolean>,
	/// PaymentStreamFlatRateAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41181")]
	pub payment_stream_flat_rate_amount: Option<f64>,
	/// PaymentStreamFlatRateCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41182")]
	pub payment_stream_flat_rate_currency: Option<String>,
	/// PaymentStreamMaximumPaymentAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41183")]
	pub payment_stream_maximum_payment_amount: Option<f64>,
	/// PaymentStreamMaximumPaymentCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41184")]
	pub payment_stream_maximum_payment_currency: Option<String>,
	/// PaymentStreamMaximumTransactionAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41185")]
	pub payment_stream_maximum_transaction_amount: Option<f64>,
	/// PaymentStreamMaximumTransactionCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41186")]
	pub payment_stream_maximum_transaction_currency: Option<String>,
	/// PaymentStreamCashSettlIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42600")]
	pub payment_stream_cash_settl_indicator: Option<crate::entities::Boolean>,
	/// Mutually exclusive with PaymentStreamCompoundingFixedRate(42605) or the PaymentStreamCompoundingFloatingRate component.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42601")]
	pub payment_stream_compounding_xid_ref: Option<String>,
	/// PaymentStreamCompoundingSpread
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42602")]
	pub payment_stream_compounding_spread: Option<f64>,
	/// PaymentStreamInterpolationMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42603")]
	pub payment_stream_interpolation_method: Option<i32>,
	/// PaymentStreamInterpolationPeriod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42604")]
	pub payment_stream_interpolation_period: Option<PaymentStreamInterpolationPeriod>,
	/// Mutually exclusive with PaymentStreamCompoundingXIDRef(42601) or the PaymentStreamCompoundingFloatingRate component.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42605")]
	pub payment_stream_compounding_fixed_rate: Option<f64>,
	/// Mutually exclusive with PaymentStreamCompoundingFixedRate(42605) or the PaymentStreamCompoundingXIDRef(42601).
	#[serde(flatten)]
	pub payment_stream_compounding_floating_rate: Option<super::payment_stream_compounding_floating_rate::PaymentStreamCompoundingFloatingRate>,
	/// PaymentStreamCompoundingDates
	#[serde(flatten)]
	pub payment_stream_compounding_dates: Option<super::payment_stream_compounding_dates::PaymentStreamCompoundingDates>,
	/// May be used to specify a count method not listed in PaymentStreamDayCount(40742).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43106")]
	pub payment_stream_other_day_count: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PaymentStreamType {
	/// Periodic (default)
	#[serde(rename = "0")]
	Periodic,
	/// Initial
	#[serde(rename = "1")]
	Initial,
	/// Single
	#[serde(rename = "2")]
	Single,
	/// Dividend
	#[serde(rename = "3")]
	Dividend,
	/// Interest
	#[serde(rename = "4")]
	Interest,
	/// Dividend return
	#[serde(rename = "5")]
	DividendReturn,
	/// Price return
	#[serde(rename = "6")]
	PriceReturn,
	/// Total return
	#[serde(rename = "7")]
	TotalReturn,
	/// Variance
	#[serde(rename = "8")]
	Variance,
	/// Correlation
	#[serde(rename = "9")]
	Correlation,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PaymentStreamDiscountType {
	/// Standard
	#[serde(rename = "0")]
	Standard,
	/// Forward Rate Agreement (FRA)
	#[serde(rename = "1")]
	ForwardRateAgreement,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PaymentStreamCompoundingMethod {
	/// None
	#[serde(rename = "0")]
	None,
	/// Flat
	#[serde(rename = "1")]
	Flat,
	/// Straight
	#[serde(rename = "2")]
	Straight,
	/// Spread exclusive
	#[serde(rename = "3")]
	SpreadExclusive,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PaymentStreamInterpolationPeriod {
	/// Initial
	#[serde(rename = "0")]
	Initial,
	/// Initial and final
	#[serde(rename = "1")]
	InitialAndFinal,
	/// Final
	#[serde(rename = "2")]
	Final,
	/// Any period
	#[serde(rename = "3")]
	AnyPeriod,
}
