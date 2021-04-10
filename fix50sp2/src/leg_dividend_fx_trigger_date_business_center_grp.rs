
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegDividendFXTriggerDateBusinessCenterGrp {
	/// NoLegDividendFXTriggerDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42364")]
	pub leg_dividend_fx_trigger_date_business_centers: Option<crate::entities::RepeatingValues<LegDividendFXTriggerDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegDividendFXTriggerDateBusinessCenter {
	/// Required if NoLegDividendFXTriggerDateBusinessCenters(42364) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42365")]
	pub leg_dividend_fx_trigger_date_business_center: Option<String>,
}
