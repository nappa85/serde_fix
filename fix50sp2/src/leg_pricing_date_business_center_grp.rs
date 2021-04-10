
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPricingDateBusinessCenterGrp {
	/// NoLegPricingDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41607")]
	pub leg_pricing_date_business_centers: Option<fix_common::RepeatingValues<LegPricingDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPricingDateBusinessCenter {
	/// Required if NoLegPricingDateBusinessCenters(41607) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41608")]
	pub leg_pricing_date_business_center: Option<String>,
}
