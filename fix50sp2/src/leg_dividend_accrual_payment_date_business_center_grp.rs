
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegDividendAccrualPaymentDateBusinessCenterGrp {
	/// NoLegDividendAccrualPaymentDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42310")]
	pub leg_dividend_accrual_payment_date_business_centers: Option<fix_common::RepeatingValues<LegDividendAccrualPaymentDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegDividendAccrualPaymentDateBusinessCenter {
	/// Required if NoLegDividendAccrualPaymentDateBusinessCenters(42310) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42311")]
	pub leg_dividend_accrual_payment_date_business_center: Option<String>,
}
