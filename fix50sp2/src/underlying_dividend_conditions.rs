
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingDividendConditions {
	/// UnderlyingDividendReinvestmentIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42826")]
	pub underlying_dividend_reinvestment_indicator: Option<crate::entities::Boolean>,
	/// UnderlyingDividendEntitlementEvent
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42827")]
	pub underlying_dividend_entitlement_event: Option<i32>,
	/// UnderlyingDividendAmountType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42828")]
	pub underlying_dividend_amount_type: Option<i32>,
	/// UnderlyingDividendUnderlierRefID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42829")]
	pub underlying_dividend_underlier_ref_id: Option<String>,
	/// UnderlyingDividendPeriodGrp
	#[serde(flatten)]
	pub underlying_dividend_period_grp: Option<super::underlying_dividend_period_grp::UnderlyingDividendPeriodGrp>,
	/// UnderlyingExtraordinaryDividendPartySide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42830")]
	pub underlying_extraordinary_dividend_party_side: Option<i32>,
	/// UnderlyingExtraordinaryDividendAmountType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42831")]
	pub underlying_extraordinary_dividend_amount_type: Option<i32>,
	/// UnderlyingExtraordinaryDividendCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42832")]
	pub underlying_extraordinary_dividend_currency: Option<String>,
	/// UnderlyingExtraordinaryDividendDeterminationMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42833")]
	pub underlying_extraordinary_dividend_determination_method: Option<String>,
	/// UnderlyingDividendFXTriggerDate
	#[serde(flatten)]
	pub underlying_dividend_fx_trigger_date: Option<super::underlying_dividend_fx_trigger_date::UnderlyingDividendFXTriggerDate>,
	/// UnderlyingDividendAccrualFloatingRate
	#[serde(flatten)]
	pub underlying_dividend_accrual_floating_rate: Option<super::underlying_dividend_accrual_floating_rate::UnderlyingDividendAccrualFloatingRate>,
	/// UnderlyingDividendAccrualFixedRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42834")]
	pub underlying_dividend_accrual_fixed_rate: Option<f32>,
	/// UnderlyingDividendAccrualPaymentDate
	#[serde(flatten)]
	pub underlying_dividend_accrual_payment_date: Option<super::underlying_dividend_accrual_payment_date::UnderlyingDividendAccrualPaymentDate>,
	/// UnderlyingDividendCompoundingMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42835")]
	pub underlying_dividend_compounding_method: Option<i32>,
	/// UnderlyingDividendNumOfIndexUnits
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42836")]
	pub underlying_dividend_num_of_index_units: Option<i32>,
	/// UnderlyingDividendCashPercentage
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42837")]
	pub underlying_dividend_cash_percentage: Option<f32>,
	/// UnderlyingDividendCashEquivalentPercentage
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42838")]
	pub underlying_dividend_cash_equivalent_percentage: Option<f32>,
	/// UnderlyingNonCashDividendTreatment
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42839")]
	pub underlying_non_cash_dividend_treatment: Option<i32>,
	/// UnderlyingDividendComposition
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42840")]
	pub underlying_dividend_composition: Option<i32>,
	/// UnderlyingSpecialDividendsIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42841")]
	pub underlying_special_dividends_indicator: Option<crate::entities::Boolean>,
	/// UnderlyingMaterialDividendsIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42842")]
	pub underlying_material_dividends_indicator: Option<crate::entities::Boolean>,
	/// UnderlyingOptionsExchangeDividendsIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42843")]
	pub underlying_options_exchange_dividends_indicator: Option<crate::entities::Boolean>,
	/// UnderlyingAdditionalDividendsIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42844")]
	pub underlying_additional_dividends_indicator: Option<crate::entities::Boolean>,
	/// UnderlyingAllDividendsIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42845")]
	pub underlying_all_dividends_indicator: Option<crate::entities::Boolean>,
}
