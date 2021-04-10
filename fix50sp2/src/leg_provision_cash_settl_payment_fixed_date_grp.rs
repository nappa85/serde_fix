
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProvisionCashSettlPaymentFixedDateGrp {
	/// NoLegProvisionCashSettlPaymentDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40473")]
	pub leg_provision_cash_settl_payment_dates: Option<crate::entities::RepeatingValues<LegProvisionCashSettlPaymentDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProvisionCashSettlPaymentDate {
	/// Required if NoLegProvisionCashSettlPaymentDates (40473) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40474")]
	pub leg_provision_cash_settl_payment_date: Option<crate::entities::LocalMktDate>,
	/// LegProvisionCashSettlPaymentDateType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40475")]
	pub leg_provision_cash_settl_payment_date_type: Option<i32>,
}
