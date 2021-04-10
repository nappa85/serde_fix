
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CashSettlDealerGrp {
	/// NoCashSettlDealers
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40277")]
	pub cash_settl_dealers: Option<crate::entities::RepeatingValues<CashSettlDealer>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CashSettlDealer {
	/// Required if NoCashSettlDealers(40277) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40032")]
	pub cash_settl_dealer: Option<String>,
}
