
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProvisionCashSettlValueDateBusinessCenterGrp {
	/// NoLegProvisionCashSettlValueDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40935")]
	pub leg_provision_cash_settl_value_date_business_centers: Option<crate::entities::RepeatingValues<LegProvisionCashSettlValueDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProvisionCashSettlValueDateBusinessCenter {
	/// Required if NoLegProvisionCashSettlValueDateBusinessCenters(40935) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40527")]
	pub leg_provision_cash_settl_value_date_business_center: Option<String>,
}
