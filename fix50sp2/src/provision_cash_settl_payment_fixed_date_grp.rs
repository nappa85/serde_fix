
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProvisionCashSettlPaymentFixedDateGrp {
	/// NoProvisionCashSettlPaymentDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40171")]
	pub provision_cash_settl_payment_dates: Option<fix_common::RepeatingValues<ProvisionCashSettlPaymentDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProvisionCashSettlPaymentDate {
	/// Required if NoProvisionCashSettlPaymentDates (40171) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40172")]
	pub provision_cash_settl_payment_date: Option<fix_common::LocalMktDate>,
	/// ProvisionCashSettlPaymentDateType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40173")]
	pub provision_cash_settl_payment_date_type: Option<ProvisionCashSettlPaymentDateType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ProvisionCashSettlPaymentDateType {
	/// Unadjusted
	#[serde(rename = "0")]
	Unadjusted,
	/// Adjusted
	#[serde(rename = "1")]
	Adjusted,
}
