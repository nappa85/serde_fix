
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProvisionCashSettlPaymentDateBusinessCenterGrp {
	/// NoLegProvisionCashSettlPaymentDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40934")]
	pub leg_provision_cash_settl_payment_date_business_centers: Option<crate::entities::RepeatingValues<LegProvisionCashSettlPaymentDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProvisionCashSettlPaymentDateBusinessCenter {
	/// Required if NoLegProvisionCashSettlPaymentDateBusinessCenters(40934) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40517")]
	pub leg_provision_cash_settl_payment_date_business_center: Option<String>,
}
