
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingProvisionCashSettlPaymentDateBusinessCenterGrp {
	/// NoUnderlyingProvisionCashSettlPaymentDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42180")]
	pub underlying_provision_cash_settl_payment_date_business_centers: Option<crate::entities::RepeatingValues<UnderlyingProvisionCashSettlPaymentDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingProvisionCashSettlPaymentDateBusinessCenter {
	/// Required if NoUnderlyingProvisionCashSettlPaymentDateBusinessCenters(42180) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42181")]
	pub underlying_provision_cash_settl_payment_date_business_center: Option<String>,
}
