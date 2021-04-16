
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingProvisionCashSettlPaymentFixedDateGrp {
	/// NoUnderlyingProvisionCashSettlPaymentDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42099")]
	pub underlying_provision_cash_settl_payment_dates: Option<fix_common::RepeatingValues<UnderlyingProvisionCashSettlPaymentDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingProvisionCashSettlPaymentDate {
	/// Required if NoUnderlyingProvisionCashSettlPaymentDates (42099) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42100")]
	pub underlying_provision_cash_settl_payment_date: Option<fix_common::LocalMktDate>,
	/// UnderlyingProvisionCashSettlPaymentDateType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42101")]
	pub underlying_provision_cash_settl_payment_date_type: Option<UnderlyingProvisionCashSettlPaymentDateType>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingProvisionCashSettlPaymentDateType {
	/// Unadjusted
	#[serde(rename = "0")]
	Unadjusted,
	/// Adjusted
	#[serde(rename = "1")]
	Adjusted,
}

impl Default for UnderlyingProvisionCashSettlPaymentDateType {
	fn default() -> Self {
		UnderlyingProvisionCashSettlPaymentDateType::Unadjusted
	}
}
