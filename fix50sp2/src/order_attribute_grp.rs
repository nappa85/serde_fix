
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OrderAttributeGrp {
	/// NoOrderAttributes
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2593")]
	pub order_attributes: Option<fix_common::RepeatingValues<OrderAttribute>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OrderAttribute {
	/// Required if NoOrderAttributes(2593) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2594")]
	pub order_attribute_type: Option<OrderAttributeType>,
	/// Required if NoOrderAttributes(2593) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2595")]
	pub order_attribute_value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OrderAttributeType {
	/// Aggregated order
	#[serde(rename = "0")]
	AggregatedOrder,
	/// Order pending allocation
	#[serde(rename = "1")]
	OrderPendingAllocation,
	/// Liquidity provision activity order
	#[serde(rename = "2")]
	LiquidityProvisionActivityOrder,
	/// Risk reduction order
	#[serde(rename = "3")]
	RiskReductionOrder,
	/// Algorithmic order
	#[serde(rename = "4")]
	AlgorithmicOrder,
	/// Systemic internaliser order
	#[serde(rename = "5")]
	SystemicInternaliserOrder,
	/// All executions for the order are to be submitted to an APA
	#[serde(rename = "6")]
	AllExecutionsForTheOrderAreToBeSubmittedToAnApa,
	/// Order execution instructed by client
	#[serde(rename = "7")]
	OrderExecutionInstructedByClient,
	/// Large in scale order
	#[serde(rename = "8")]
	LargeInScaleOrder,
	/// Hidden order
	#[serde(rename = "9")]
	HiddenOrder,
	/// Representative order
	#[serde(rename = "12")]
	RepresentativeOrder,
	/// Linkage type
	#[serde(rename = "13")]
	LinkageType,
	/// Subject to EU share trading obligation (STO)
	#[serde(rename = "10")]
	SubjectToEuShareTradingObligation,
	/// Subject to UK share trading obligation (STO)
	#[serde(rename = "11")]
	SubjectToUkShareTradingObligation,
	/// Exempt from share trading obligation (STO)
	#[serde(rename = "14")]
	ExemptFromShareTradingObligation,
}

impl Default for OrderAttributeType {
	fn default() -> Self {
		OrderAttributeType::AggregatedOrder
	}
}
