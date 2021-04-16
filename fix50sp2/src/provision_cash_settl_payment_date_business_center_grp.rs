
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProvisionCashSettlPaymentDateBusinessCenterGrp {
	/// NoProvisionCashSettlPaymentDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40952")]
	pub provision_cash_settl_payment_date_business_centers: Option<fix_common::RepeatingValues<ProvisionCashSettlPaymentDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProvisionCashSettlPaymentDateBusinessCenter {
	/// Required if NoProvisionCashSettlPaymentDateBusinessCenters(40952) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40164")]
	pub provision_cash_settl_payment_date_business_center: Option<String>,
}
