
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStream {
	/// LegPaymentStreamType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40279")]
	pub leg_payment_stream_type: Option<LegPaymentStreamType>,
	/// LegPaymentStreamMarketRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40280")]
	pub leg_payment_stream_market_rate: Option<i32>,
	/// LegPaymentStreamDelayIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40281")]
	pub leg_payment_stream_delay_indicator: Option<fix_common::Boolean>,
	/// LegPaymentStreamSettlCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40282")]
	pub leg_payment_stream_settl_currency: Option<LegPaymentStreamSettlCurrency>,
	/// LegPaymentStreamDayCount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40283")]
	pub leg_payment_stream_day_count: Option<i32>,
	/// LegPaymentStreamAccrualDays
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40284")]
	pub leg_payment_stream_accrual_days: Option<i32>,
	/// LegPaymentStreamDiscountType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40285")]
	pub leg_payment_stream_discount_type: Option<LegPaymentStreamDiscountType>,
	/// LegPaymentStreamDiscountRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40286")]
	pub leg_payment_stream_discount_rate: Option<f32>,
	/// LegPaymentStreamDiscountRateDayCount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40287")]
	pub leg_payment_stream_discount_rate_day_count: Option<LegPaymentStreamDiscountRateDayCount>,
	/// LegPaymentStreamCompoundingMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40288")]
	pub leg_payment_stream_compounding_method: Option<LegPaymentStreamCompoundingMethod>,
	/// LegPaymentStreamInitialPrincipalExchangeIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40289")]
	pub leg_payment_stream_initial_principal_exchange_indicator: Option<fix_common::Boolean>,
	/// LegPaymentStreamInterimPrincipalExchangeIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40290")]
	pub leg_payment_stream_interim_principal_exchange_indicator: Option<fix_common::Boolean>,
	/// LegPaymentStreamFinalPrincipalExchangeIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40291")]
	pub leg_payment_stream_final_principal_exchange_indicator: Option<fix_common::Boolean>,
	/// LegPaymentStreamPaymentDates
	#[serde(flatten)]
	pub leg_payment_stream_payment_dates: Option<super::leg_payment_stream_payment_dates::LegPaymentStreamPaymentDates>,
	/// LegPaymentStreamResetDates
	#[serde(flatten)]
	pub leg_payment_stream_reset_dates: Option<super::leg_payment_stream_reset_dates::LegPaymentStreamResetDates>,
	/// LegPaymentStreamFixedRate
	#[serde(flatten)]
	pub leg_payment_stream_fixed_rate: Option<super::leg_payment_stream_fixed_rate::LegPaymentStreamFixedRate>,
	/// LegPaymentStreamFloatingRate
	#[serde(flatten)]
	pub leg_payment_stream_floating_rate: Option<super::leg_payment_stream_floating_rate::LegPaymentStreamFloatingRate>,
	/// LegPaymentStreamNonDeliverableSettlTerms
	#[serde(flatten)]
	pub leg_payment_stream_non_deliverable_settl_terms: Option<super::leg_payment_stream_non_deliverable_settl_terms::LegPaymentStreamNonDeliverableSettlTerms>,
	/// LegPaymentStreamFlatRateIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41549")]
	pub leg_payment_stream_flat_rate_indicator: Option<fix_common::Boolean>,
	/// LegPaymentStreamFlatRateAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41550")]
	pub leg_payment_stream_flat_rate_amount: Option<f64>,
	/// LegPaymentStreamFlatRateCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41551")]
	pub leg_payment_stream_flat_rate_currency: Option<String>,
	/// LegStreamMaximumPaymentAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41552")]
	pub leg_stream_maximum_payment_amount: Option<f64>,
	/// LegStreamMaximumPaymentCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41553")]
	pub leg_stream_maximum_payment_currency: Option<String>,
	/// LegStreamMaximumTransactionAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41554")]
	pub leg_stream_maximum_transaction_amount: Option<f64>,
	/// LegStreamMaximumTransactionCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41555")]
	pub leg_stream_maximum_transaction_currency: Option<String>,
	/// LegPaymentStreamCashSettlIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42399")]
	pub leg_payment_stream_cash_settl_indicator: Option<fix_common::Boolean>,
	/// Mutually exclusive with LegPaymentStreamCompoundingFixedRate(42404) or the LegPaymentStreamCompoundingFloatingRate component.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42400")]
	pub leg_payment_stream_compounding_xid_ref: Option<String>,
	/// LegPaymentStreamCompoundingSpread
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42401")]
	pub leg_payment_stream_compounding_spread: Option<f64>,
	/// LegPaymentStreamInterpolationMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42402")]
	pub leg_payment_stream_interpolation_method: Option<i32>,
	/// LegPaymentStreamInterpolationPeriod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42403")]
	pub leg_payment_stream_interpolation_period: Option<i32>,
	/// Mutually exclusive with LegPaymentStreamCompoundingXIDRef(42400) or the LegPaymentStreamCompoundingFloatingRate component.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42404")]
	pub leg_payment_stream_compounding_fixed_rate: Option<f64>,
	/// Mutually exclusive with LegPaymentStreamCompoundingFixedRate(42404) or the LegPaymentStreamCompoundingXIDRef(42400).
	#[serde(flatten)]
	pub leg_payment_stream_compounding_floating_rate: Option<super::leg_payment_stream_compounding_floating_rate::LegPaymentStreamCompoundingFloatingRate>,
	/// LegPaymentStreamCompoundingDates
	#[serde(flatten)]
	pub leg_payment_stream_compounding_dates: Option<super::leg_payment_stream_compounding_dates::LegPaymentStreamCompoundingDates>,
	/// May be used to specify a count method not listed in LegPaymentStreamDayCount(40283).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43108")]
	pub leg_payment_stream_other_day_count: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegPaymentStreamType {
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

impl Default for LegPaymentStreamType {
	fn default() -> Self {
		LegPaymentStreamType::Periodic
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegPaymentStreamSettlCurrency {
	/// 1/1 (If parties specify the Day Count Fraction to be 1/1 then in calculating the applicable amount, 1 is simply input into
	/// the calculation as the relevant Day Count Fraction.)
	#[serde(rename = "0")]
	N0,
	/// 30/360 (30U/360 or Bond Basis) (Mainly used in the US with the following date adjustment rules: (1) If the investment is End-Of-Month
	/// and Date1 is the last day of February and Date2 is the last day of February, then change Date2 to 30; (2) If the investment
	/// is End-Of-Month and Date1 is the last day of February, then change Date1 to 30; (3) If Date2 is 31 and Date1 is 30 or 31,
	/// then change Date2 to 30; (4) If Date1 is 31, then change Date1 to 30.)
	#[serde(rename = "1")]
	N1,
	/// 30/360 (SIA) (A variant of "30/360" - when Date1 and Date2 are both Feb. 28th or 29th convert them to 30th using the same
	/// logic in the conversion of 31st to 30th.)
	#[serde(rename = "2")]
	N2,
	/// 30/360M (Commonly used day count convention for US mortgage backed securities. Feb 28th (or 29th in a leap year) is always
	/// considered as a 30th for a start date. As a comparison, in the regular 30/360 day count as used by most US agency and corporate
	/// bonds, a start date of Feb 28th (or 29th in a leap year) is still considered as the 28th (or 29th) day of a month of 30 days.)
	#[serde(rename = "3")]
	N3,
	/// 30E/360 (Eurobond Basis) (Also known as 30/360.ISMA, 30S/360, or Special German. Date adjustment rules are: (1) If Date1 falls
	/// on the 31st, then change it to the 30th; (2) If Date2 falls on the 31st, then change it to the 30th.)
	#[serde(rename = "4")]
	N4,
	/// 30E/360 (ISDA) (Date adjustment rules are: (1) if Date1 is the last day of the month, then change Date1 to 30; (2) if D2 is
	/// the last day of the month (unless Date2 is the maturity date and Date2 is in February), then change Date2 to 30.)
	#[serde(rename = "5")]
	N5,
	/// Act/360 (The actual number of days between Date1 and Date2, divided by 360.)
	#[serde(rename = "6")]
	N6,
	/// Act/365 (FIXED) (The actual number of days between Date1 and Date2, divided by 365.)
	#[serde(rename = "7")]
	N7,
	/// Act/Act (AFB) (The actual number of days between Date1 and Date2, the denominator is either 365 (if the calculation period
	/// does not contain the 29th February) or 366 (if the calculation period includes 29th February).)
	#[serde(rename = "8")]
	N8,
	/// Act/Act ICMA (The denominator is the actual number of days in the coupon period multiplied by the number of coupon periods
	/// in the year. Assumes that regular coupons alwaysfall on the same day of the month where possible.)
	#[serde(rename = "9")]
	N9,
	/// Act/Act (ICMA Ultimo) (The Act/Act (ICMA Ultimo) differs from Act/Act (ICMA) method only that it assumes that regular coupons
	/// always fall on the last day of the month.)
	#[serde(rename = "10")]
	N10,
	/// Act/Act ISDA (The denominator varies depending on whether a portion of the relevant calculation period falls within a leap
	/// year. For the portion of the calculation period falling in a leap year, the denominator is 366 and for the portion falling
	/// outside a leap year, the denominator is 365.)
	#[serde(rename = "11")]
	N11,
	/// BUS/252 (Used for Brazilian Real swaps which is based on business days instead of calendar days. The number of business days
	/// divied by 252)
	#[serde(rename = "12")]
	N12,
	/// 30E+/360 (Variation on 30E/360. Date adjustment rules: (1) If Date1 falls on the 31st, then change it to the 30th; (2) If
	/// Date2 falls on the 31st, then change it to 1 and increase Month2 by one, i.e. next month)
	#[serde(rename = "13")]
	N13,
	/// Act/365L (The number of days in a period equal to the actual number of days. The number of days in a year is 365, or if the
	/// period ends in a leap year 366. Used for Sterling floating rate notes. May also be referred to as ISMA-Year.)
	#[serde(rename = "14")]
	N14,
	/// NL365 (The number of days in a period equal to the actual number of days, with the exception of leap days (29th February)
	/// which are ignored. The number of days in a year is 365, even in a leap year.)
	#[serde(rename = "15")]
	N15,
	/// NL360 (This is the same as Act/360, with the exception of leap days (29th February) which are ignored.)
	#[serde(rename = "16")]
	N16,
	/// Act/364 (The actual number of days between Date1 and Date2, divided by 364.)
	#[serde(rename = "17")]
	N17,
	/// 30/365
	#[serde(rename = "18")]
	N18,
	/// 30/Actual
	#[serde(rename = "19")]
	N19,
	/// 30/360 (ICMA or basis rule)
	#[serde(rename = "20")]
	N20,
	/// 30E2/360 (Eurobond basis model two)
	#[serde(rename = "21")]
	N21,
	/// 30E3/360 (Eurobond basis model three)
	#[serde(rename = "22")]
	N22,
	/// Other
	#[serde(rename = "99")]
	N99,
}

impl Default for LegPaymentStreamSettlCurrency {
	fn default() -> Self {
		LegPaymentStreamSettlCurrency::N0
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegPaymentStreamDiscountType {
	/// Standard
	#[serde(rename = "0")]
	Standard,
	/// Forward Rate Agreement (FRA)
	#[serde(rename = "1")]
	ForwardRateAgreement,
}

impl Default for LegPaymentStreamDiscountType {
	fn default() -> Self {
		LegPaymentStreamDiscountType::Standard
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegPaymentStreamDiscountRateDayCount {
	/// 1/1 (If parties specify the Day Count Fraction to be 1/1 then in calculating the applicable amount, 1 is simply input into
	/// the calculation as the relevant Day Count Fraction.)
	#[serde(rename = "0")]
	N0,
	/// 30/360 (30U/360 or Bond Basis) (Mainly used in the US with the following date adjustment rules: (1) If the investment is End-Of-Month
	/// and Date1 is the last day of February and Date2 is the last day of February, then change Date2 to 30; (2) If the investment
	/// is End-Of-Month and Date1 is the last day of February, then change Date1 to 30; (3) If Date2 is 31 and Date1 is 30 or 31,
	/// then change Date2 to 30; (4) If Date1 is 31, then change Date1 to 30.)
	#[serde(rename = "1")]
	N1,
	/// 30/360 (SIA) (A variant of "30/360" - when Date1 and Date2 are both Feb. 28th or 29th convert them to 30th using the same
	/// logic in the conversion of 31st to 30th.)
	#[serde(rename = "2")]
	N2,
	/// 30/360M (Commonly used day count convention for US mortgage backed securities. Feb 28th (or 29th in a leap year) is always
	/// considered as a 30th for a start date. As a comparison, in the regular 30/360 day count as used by most US agency and corporate
	/// bonds, a start date of Feb 28th (or 29th in a leap year) is still considered as the 28th (or 29th) day of a month of 30 days.)
	#[serde(rename = "3")]
	N3,
	/// 30E/360 (Eurobond Basis) (Also known as 30/360.ISMA, 30S/360, or Special German. Date adjustment rules are: (1) If Date1 falls
	/// on the 31st, then change it to the 30th; (2) If Date2 falls on the 31st, then change it to the 30th.)
	#[serde(rename = "4")]
	N4,
	/// 30E/360 (ISDA) (Date adjustment rules are: (1) if Date1 is the last day of the month, then change Date1 to 30; (2) if D2 is
	/// the last day of the month (unless Date2 is the maturity date and Date2 is in February), then change Date2 to 30.)
	#[serde(rename = "5")]
	N5,
	/// Act/360 (The actual number of days between Date1 and Date2, divided by 360.)
	#[serde(rename = "6")]
	N6,
	/// Act/365 (FIXED) (The actual number of days between Date1 and Date2, divided by 365.)
	#[serde(rename = "7")]
	N7,
	/// Act/Act (AFB) (The actual number of days between Date1 and Date2, the denominator is either 365 (if the calculation period
	/// does not contain the 29th February) or 366 (if the calculation period includes 29th February).)
	#[serde(rename = "8")]
	N8,
	/// Act/Act ICMA (The denominator is the actual number of days in the coupon period multiplied by the number of coupon periods
	/// in the year. Assumes that regular coupons alwaysfall on the same day of the month where possible.)
	#[serde(rename = "9")]
	N9,
	/// Act/Act (ICMA Ultimo) (The Act/Act (ICMA Ultimo) differs from Act/Act (ICMA) method only that it assumes that regular coupons
	/// always fall on the last day of the month.)
	#[serde(rename = "10")]
	N10,
	/// Act/Act ISDA (The denominator varies depending on whether a portion of the relevant calculation period falls within a leap
	/// year. For the portion of the calculation period falling in a leap year, the denominator is 366 and for the portion falling
	/// outside a leap year, the denominator is 365.)
	#[serde(rename = "11")]
	N11,
	/// BUS/252 (Used for Brazilian Real swaps which is based on business days instead of calendar days. The number of business days
	/// divied by 252)
	#[serde(rename = "12")]
	N12,
	/// 30E+/360 (Variation on 30E/360. Date adjustment rules: (1) If Date1 falls on the 31st, then change it to the 30th; (2) If
	/// Date2 falls on the 31st, then change it to 1 and increase Month2 by one, i.e. next month)
	#[serde(rename = "13")]
	N13,
	/// Act/365L (The number of days in a period equal to the actual number of days. The number of days in a year is 365, or if the
	/// period ends in a leap year 366. Used for Sterling floating rate notes. May also be referred to as ISMA-Year.)
	#[serde(rename = "14")]
	N14,
	/// NL365 (The number of days in a period equal to the actual number of days, with the exception of leap days (29th February)
	/// which are ignored. The number of days in a year is 365, even in a leap year.)
	#[serde(rename = "15")]
	N15,
	/// NL360 (This is the same as Act/360, with the exception of leap days (29th February) which are ignored.)
	#[serde(rename = "16")]
	N16,
	/// Act/364 (The actual number of days between Date1 and Date2, divided by 364.)
	#[serde(rename = "17")]
	N17,
	/// 30/365
	#[serde(rename = "18")]
	N18,
	/// 30/Actual
	#[serde(rename = "19")]
	N19,
	/// 30/360 (ICMA or basis rule)
	#[serde(rename = "20")]
	N20,
	/// 30E2/360 (Eurobond basis model two)
	#[serde(rename = "21")]
	N21,
	/// 30E3/360 (Eurobond basis model three)
	#[serde(rename = "22")]
	N22,
	/// Other
	#[serde(rename = "99")]
	N99,
}

impl Default for LegPaymentStreamDiscountRateDayCount {
	fn default() -> Self {
		LegPaymentStreamDiscountRateDayCount::N0
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegPaymentStreamCompoundingMethod {
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

impl Default for LegPaymentStreamCompoundingMethod {
	fn default() -> Self {
		LegPaymentStreamCompoundingMethod::None
	}
}
