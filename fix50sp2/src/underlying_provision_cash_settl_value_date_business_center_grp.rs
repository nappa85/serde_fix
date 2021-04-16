
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingProvisionCashSettlValueDateBusinessCenterGrp {
	/// NoUnderlyingProvisionCashSettlValueDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42182")]
	pub underlying_provision_cash_settl_value_date_business_centers: Option<fix_common::RepeatingValues<UnderlyingProvisionCashSettlValueDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingProvisionCashSettlValueDateBusinessCenter {
	/// Required if NoUnderlyingProvisionCashSettlValueDateBusinessCenters(42182) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42183")]
	pub underlying_provision_cash_settl_value_date_business_center: Option<String>,
}
