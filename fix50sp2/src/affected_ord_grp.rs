
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AffectedOrdGrp {
	/// NoAffectedOrders
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "534")]
	pub affected_orders: Option<crate::entities::RepeatingValues<AffectedOrder>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AffectedOrder {
	/// Required if NoAffectedOrders(534) &gt; 0. Indicates the client order id of an order affected by this request. If order(s) were
	/// manually delivered (or otherwise not delivered over FIX and not assigned a ClOrdID(11)) this field should contain string "MANUAL".
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1824")]
	pub affected_orig_cl_ord_id: Option<String>,
	/// Required if NoAffectedOrders &gt; 0 and must be the first repeating field in the group. Indicates the client order id of an order
	/// affected by this request. If order(s) were manually delivered (or otherwise not delivered over FIX and not assigned a ClOrdID)
	/// this field should contain string "MANUAL".
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41")]
	pub orig_cl_ord_id: Option<String>,
	/// Contains the OrderID(37) assigned by the counterparty of an affected order. Conditionally required when AffectedOrigClOrdID(1824)
	/// = "MANUAL".
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "535")]
	pub affected_order_id: Option<String>,
	/// Contains the SecondaryOrderID(198) assigned by the counterparty of an affected order. Not required as part of the repeating
	/// group.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "536")]
	pub affected_secondary_order_id: Option<String>,
}
