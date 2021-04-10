
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentScheduleInterimExchangeDateBusinessCenterGrp {
	/// NoPaymentScheduleInterimExchangeDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40945")]
	pub payment_schedule_interim_exchange_date_business_centers: Option<crate::entities::RepeatingValues<PaymentScheduleInterimExchangeDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentScheduleInterimExchangeDateBusinessCenter {
	/// Required if NoPaymentScheduleInterimExchangeDateBusinessCenters(40945) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40863")]
	pub payment_schedule_interim_exchange_dates_business_center: Option<String>,
}
