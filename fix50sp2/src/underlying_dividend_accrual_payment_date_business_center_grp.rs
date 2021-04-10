
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingDividendAccrualPaymentDateBusinessCenterGrp {
	/// NoUnderlyingDividendAccrualPaymentDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42799")]
	pub underlying_dividend_accrual_payment_date_business_centers: Option<crate::entities::RepeatingValues<UnderlyingDividendAccrualPaymentDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingDividendAccrualPaymentDateBusinessCenter {
	/// Required if NoUnderlyingDividendAccrualPaymentDateBusinessCenters(42799) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42800")]
	pub underlying_dividend_accrual_payment_date_business_center: Option<String>,
}
