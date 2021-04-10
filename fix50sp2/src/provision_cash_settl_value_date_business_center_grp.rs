
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProvisionCashSettlValueDateBusinessCenterGrp {
	/// NoProvisionCashSettlValueDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40953")]
	pub provision_cash_settl_value_date_business_centers: Option<fix_common::RepeatingValues<ProvisionCashSettlValueDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProvisionCashSettlValueDateBusinessCenter {
	/// Required if NoProvisionCashSettlValueDatBusinessCenters(40953) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40117")]
	pub provision_cash_settl_value_date_business_center: Option<String>,
}
