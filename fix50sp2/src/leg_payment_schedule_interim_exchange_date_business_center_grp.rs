
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentScheduleInterimExchangeDateBusinessCenterGrp {
	/// NoLegPaymentScheduleInterimExchangeDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40928")]
	pub leg_payment_schedule_interim_exchange_date_business_centers: Option<crate::entities::RepeatingValues<LegPaymentScheduleInterimExchangeDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentScheduleInterimExchangeDateBusinessCenter {
	/// Required if NoLegPaymentScheduleInterimExchangeDateBusinessCenters(40928) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40409")]
	pub leg_payment_schedule_interim_exchange_dates_business_center: Option<String>,
}
