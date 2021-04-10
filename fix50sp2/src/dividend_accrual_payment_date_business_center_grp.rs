
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DividendAccrualPaymentDateBusinessCenterGrp {
	/// NoDividendAccrualPaymentDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42236")]
	pub dividend_accrual_payment_date_business_centers: Option<fix_common::RepeatingValues<DividendAccrualPaymentDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DividendAccrualPaymentDateBusinessCenter {
	/// Required if NoDividendAccrualPaymentDateBusinessCenters(42236) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42237")]
	pub dividend_accrual_payment_date_business_center: Option<String>,
}
