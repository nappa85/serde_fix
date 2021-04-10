
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegDividendConditions {
	/// LegDividendReinvestmentIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42337")]
	pub leg_dividend_reinvestment_indicator: Option<crate::entities::Boolean>,
	/// LegDividendEntitlementEvent
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42338")]
	pub leg_dividend_entitlement_event: Option<i32>,
	/// LegDividendAmountType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42339")]
	pub leg_dividend_amount_type: Option<i32>,
	/// LegDividendUnderlierRefID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42340")]
	pub leg_dividend_underlier_ref_id: Option<String>,
	/// LegDividendPeriodGrp
	#[serde(flatten)]
	pub leg_dividend_period_grp: Option<super::leg_dividend_period_grp::LegDividendPeriodGrp>,
	/// LegExtraordinaryDividendPartySide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42341")]
	pub leg_extraordinary_dividend_party_side: Option<i32>,
	/// LegExtraordinaryDividendAmountType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42342")]
	pub leg_extraordinary_dividend_amount_type: Option<i32>,
	/// LegExtraordinaryDividendCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42343")]
	pub leg_extraordinary_dividend_currency: Option<String>,
	/// LegExtraordinaryDividendDeterminationMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42344")]
	pub leg_extraordinary_dividend_determination_method: Option<String>,
	/// LegDividendFXTriggerDate
	#[serde(flatten)]
	pub leg_dividend_fx_trigger_date: Option<super::leg_dividend_fx_trigger_date::LegDividendFXTriggerDate>,
	/// LegDividendAccrualFloatingRate
	#[serde(flatten)]
	pub leg_dividend_accrual_floating_rate: Option<super::leg_dividend_accrual_floating_rate::LegDividendAccrualFloatingRate>,
	/// LegDividendAccrualFixedRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42345")]
	pub leg_dividend_accrual_fixed_rate: Option<f32>,
	/// LegDividendAccrualPaymentDate
	#[serde(flatten)]
	pub leg_dividend_accrual_payment_date: Option<super::leg_dividend_accrual_payment_date::LegDividendAccrualPaymentDate>,
	/// LegDividendCompoundingMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42346")]
	pub leg_dividend_compounding_method: Option<i32>,
	/// LegDividendNumOfIndexUnits
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42347")]
	pub leg_dividend_num_of_index_units: Option<i32>,
	/// LegDividendCashPercentage
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42348")]
	pub leg_dividend_cash_percentage: Option<f32>,
	/// LegDividendCashEquivalentPercentage
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42349")]
	pub leg_dividend_cash_equivalent_percentage: Option<f32>,
	/// LegNonCashDividendTreatment
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42350")]
	pub leg_non_cash_dividend_treatment: Option<i32>,
	/// LegDividendComposition
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42351")]
	pub leg_dividend_composition: Option<i32>,
	/// LegSpecialDividendsIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42352")]
	pub leg_special_dividends_indicator: Option<crate::entities::Boolean>,
	/// LegMaterialDividendsIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42353")]
	pub leg_material_dividends_indicator: Option<crate::entities::Boolean>,
	/// LegOptionsExchangeDividendsIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42354")]
	pub leg_options_exchange_dividends_indicator: Option<crate::entities::Boolean>,
	/// LegAdditionalDividendsIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42355")]
	pub leg_additional_dividends_indicator: Option<crate::entities::Boolean>,
	/// LegAllDividendsIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42356")]
	pub leg_all_dividends_indicator: Option<crate::entities::Boolean>,
}
