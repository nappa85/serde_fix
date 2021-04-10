
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NotAffectedOrdGrp {
	/// NoNotAffectedOrders
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1370")]
	pub not_affected_orders: Option<crate::entities::RepeatingValues<NotAffectedOrder>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NotAffectedOrder {
	/// Required if NoNotAffectedOrders(1370) &gt; 0 and must be the first repeating field in the group. Indicates the client order identifier
	/// of an order not affected by the request. If order(s) were manually delivered (or otherwise not delivered over FIX and not
	/// assigned a ClOrdID(11)) this field should contain string "MANUAL"
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1372")]
	pub not_aff_orig_cl_ord_id: Option<String>,
	/// Contains the OrderID(37) assigned by the counterparty of an unaffected order. Not required as part of the repeating group
	/// if NotAffOrigClOrdID(1372) has a value other than "MANUAL"
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1371")]
	pub not_affected_order_id: Option<String>,
	/// Contains the SecondaryOrderID(198) assigned by the counterparty of an unaffected order. Not required as part of the repeating
	/// group
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1825")]
	pub not_aff_secondary_order_id: Option<String>,
	/// Can be used to provide a reason for excluding this order from the scope of the mass action.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2677")]
	pub not_affected_reason: Option<NotAffectedReason>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NotAffectedReason {
	/// Order suspended
	#[serde(rename = "0")]
	OrderSuspended,
	/// Instrument suspended
	#[serde(rename = "1")]
	InstrumentSuspended,
}
