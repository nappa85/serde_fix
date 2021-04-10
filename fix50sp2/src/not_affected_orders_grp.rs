
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NotAffectedOrdersGrp {
	/// Optional field used to indicate the number of order identifiers for orders not affected by the request. Must be followed with <a href="tag_1372_NotAffOrigClOrdID.html" target="bottom">NotAffOrigClOrdID (1372)&nbsp;(1372)</a> as the next field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1370")]
	pub not_affected_orders: Option<crate::entities::RepeatingValues<NotAffectedOrder>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NotAffectedOrder {
	/// Required if NoNotAffectedOrders(1370) &gt; 0 and must be the first repeating field in the group. Indicates the client order id
	/// of an order not affected by the request. If order(s) were manually delivered (or otherwise not delivered over FIX and not
	/// assigned a ClOrdID) this field should contain string "MANUAL".
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1372")]
	pub not_aff_orig_cl_ord_id: Option<String>,
	/// Contains the OrderID assigned by the counterparty of an unaffected order. Not required as part of the repeating group if <a href="tag_1372_NotAffOrigClOrdID.html" target="bottom">NotAffOrigClOrdID(1372)&nbsp;(1372)</a> has a value other than "MANUAL".
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1371")]
	pub not_affected_order_id: Option<String>,
}
