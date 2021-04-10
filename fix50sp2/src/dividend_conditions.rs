
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DividendConditions {
	/// DividendReinvestmentIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42245")]
	pub dividend_reinvestment_indicator: Option<crate::entities::Boolean>,
	/// DividendEntitlementEvent
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42246")]
	pub dividend_entitlement_event: Option<DividendEntitlementEvent>,
	/// DividendAmountType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42247")]
	pub dividend_amount_type: Option<DividendAmountType>,
	/// DividendUnderlierRefID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42248")]
	pub dividend_underlier_ref_id: Option<String>,
	/// DividendPeriodGrp
	#[serde(flatten)]
	pub dividend_period_grp: Option<super::dividend_period_grp::DividendPeriodGrp>,
	/// ExtraordinaryDividendPartySide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42249")]
	pub extraordinary_dividend_party_side: Option<i32>,
	/// ExtraordinaryDividendAmountType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42250")]
	pub extraordinary_dividend_amount_type: Option<i32>,
	/// ExtraordinaryDividendCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42251")]
	pub extraordinary_dividend_currency: Option<String>,
	/// ExtraordinaryDividendDeterminationMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42252")]
	pub extraordinary_dividend_determination_method: Option<String>,
	/// DividendFXTriggerDate
	#[serde(flatten)]
	pub dividend_fx_trigger_date: Option<super::dividend_fx_trigger_date::DividendFXTriggerDate>,
	/// DividendAccrualFloatingRate
	#[serde(flatten)]
	pub dividend_accrual_floating_rate: Option<super::dividend_accrual_floating_rate::DividendAccrualFloatingRate>,
	/// DividendAccrualFixedRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42253")]
	pub dividend_accrual_fixed_rate: Option<f32>,
	/// DividendAccrualPaymentDate
	#[serde(flatten)]
	pub dividend_accrual_payment_date: Option<super::dividend_accrual_payment_date::DividendAccrualPaymentDate>,
	/// DividendCompoundingMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42254")]
	pub dividend_compounding_method: Option<i32>,
	/// DividendNumOfIndexUnits
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42255")]
	pub dividend_num_of_index_units: Option<i32>,
	/// DividendCashPercentage
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42256")]
	pub dividend_cash_percentage: Option<f32>,
	/// DividendCashEquivalentPercentage
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42257")]
	pub dividend_cash_equivalent_percentage: Option<f32>,
	/// NonCashDividendTreatment
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42258")]
	pub non_cash_dividend_treatment: Option<NonCashDividendTreatment>,
	/// DividendComposition
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42259")]
	pub dividend_composition: Option<DividendComposition>,
	/// SpecialDividendsIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42260")]
	pub special_dividends_indicator: Option<crate::entities::Boolean>,
	/// MaterialDividendsIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42261")]
	pub material_dividends_indicator: Option<crate::entities::Boolean>,
	/// OptionsExchangeDividendsIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42262")]
	pub options_exchange_dividends_indicator: Option<crate::entities::Boolean>,
	/// AdditionalDividendsIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42263")]
	pub additional_dividends_indicator: Option<crate::entities::Boolean>,
	/// AllDividendsIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42264")]
	pub all_dividends_indicator: Option<crate::entities::Boolean>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DividendEntitlementEvent {
	/// Ex-date
	#[serde(rename = "0")]
	ExDate,
	/// Record date
	#[serde(rename = "1")]
	RecordDate,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DividendAmountType {
	/// Record amount
	#[serde(rename = "0")]
	RecordAmount,
	/// Ex amount
	#[serde(rename = "1")]
	ExAmount,
	/// Paid amount
	#[serde(rename = "2")]
	PaidAmount,
	/// As specified in master confirmation
	#[serde(rename = "3")]
	AsSpecifiedInMasterConfirmation,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NonCashDividendTreatment {
	/// Potential adjustment event
	#[serde(rename = "0")]
	PotentialAdjustmentEvent,
	/// Cash equivalent
	#[serde(rename = "1")]
	CashEquivalent,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DividendComposition {
	/// Equity amount receiver election
	#[serde(rename = "0")]
	EquityAmountReceiverElection,
	/// Calculation agent election
	#[serde(rename = "1")]
	CalculationAgentElection,
}
