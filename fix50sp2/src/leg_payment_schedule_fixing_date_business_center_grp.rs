
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentScheduleFixingDateBusinessCenterGrp {
	/// NoLegPaymentScheduleFixingDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40927")]
	pub leg_payment_schedule_fixing_date_business_centers: Option<crate::entities::RepeatingValues<LegPaymentScheduleFixingDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentScheduleFixingDateBusinessCenter {
	/// Required if NoLegPaymentScheduleFixingDateBusinessCenters(40927) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40400")]
	pub leg_payment_schedule_fixing_date_business_center: Option<String>,
}
