
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegCashSettlDealerGrp {
	/// NoLegCashSettlDealers
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41342")]
	pub leg_cash_settl_dealers: Option<fix_common::RepeatingValues<LegCashSettlDealer>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegCashSettlDealer {
	/// Required if NoLegCashSettlDealers(41342) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41343")]
	pub leg_cash_settl_dealer: Option<String>,
}
