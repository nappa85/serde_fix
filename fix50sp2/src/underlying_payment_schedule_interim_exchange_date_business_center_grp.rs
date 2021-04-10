
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentScheduleInterimExchangeDateBusinessCenterGrp {
	/// NoUnderlyingPaymentScheduleInterimExchangeDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40967")]
	pub underlying_payment_schedule_interim_exchange_date_business_centers: Option<crate::entities::RepeatingValues<UnderlyingPaymentScheduleInterimExchangeDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentScheduleInterimExchangeDateBusinessCenter {
	/// Required if NoUnderlyingPaymentScheduleInterimExchangeDateBusinessCenters(40967) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40699")]
	pub underlying_payment_schedule_interim_exchange_dates_business_center: Option<String>,
}
