
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingCashSettlDealerGrp {
	/// NoUnderlyingCashSettlDealers
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42039")]
	pub underlying_cash_settl_dealers: Option<crate::entities::RepeatingValues<UnderlyingCashSettlDealer>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingCashSettlDealer {
	/// Required if NoUnderlyingCashSettlDealers(42039) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42040")]
	pub underlying_cash_settl_dealer: Option<String>,
}
