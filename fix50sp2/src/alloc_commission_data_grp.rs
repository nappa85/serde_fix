
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AllocCommissionDataGrp {
	/// NoAllocCommissions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2653")]
	pub alloc_commissions: Option<fix_common::RepeatingValues<AllocCommission>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AllocCommission {
	/// Required if NoAllocCommissions(2653) &gt; 0. If the commission is based on a percentage of trade quantity or a factor of "unit
	/// of measure", AllocCommissionRate(2660) and AllocCommissionUnitOfMeasure(2658) may also be specified as appropriate.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2654")]
	pub alloc_commission_amount: Option<f64>,
	/// Required if NoAllocCommissions(2653) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2655")]
	pub alloc_commission_amount_type: Option<AllocCommissionAmountType>,
	/// Required if NoAllocCommissions(2653) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2656")]
	pub alloc_commission_basis: Option<AllocCommissionBasis>,
	/// AllocCommissionCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2657")]
	pub alloc_commission_currency: Option<String>,
	/// AllocCommissionUnitOfMeasure
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2658")]
	pub alloc_commission_unit_of_measure: Option<String>,
	/// AllocCommissionUnitOfMeasureCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2659")]
	pub alloc_commission_unit_of_measure_currency: Option<String>,
	/// AllocCommissionRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2660")]
	pub alloc_commission_rate: Option<f64>,
	/// AllocCommissionSharedIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2661")]
	pub alloc_commission_shared_indicator: Option<fix_common::Boolean>,
	/// If specified, AllocCommissionSharedIndicator(2661) must be set to "Y".
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2662")]
	pub alloc_commission_amount_shared: Option<f64>,
	/// This field may be used for multi-leg trades sent as a single message to indicate that the entry applies only to a specific
	/// leg.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2663")]
	pub alloc_commission_leg_ref_id: Option<String>,
	/// AllocCommissionDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2664")]
	pub alloc_commission_desc: Option<String>,
	/// Must be set if EncodedAllocCommissionDesc(2666) is specified and must immediately precede it.
	#[serde(rename = "2665")]
	/// Encoded (non-ASCII characters) representation of the AllocCommissionDesc(2664) field in the encoded format specified via the
	/// MessageEncoding(347) field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "2666")]
	pub encoded_alloc_commission_desc: Option<fix_common::EncodedText<2666>>,
	/// AllocCommissionAmountSubType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2726")]
	pub alloc_commission_amount_sub_type: Option<AllocCommissionAmountSubType>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum AllocCommissionAmountType {
	/// Unspecified
	#[serde(rename = "0")]
	Unspecified,
	/// Acceptance
	#[serde(rename = "1")]
	Acceptance,
	/// Broker
	#[serde(rename = "2")]
	Broker,
	/// Clearing broker
	#[serde(rename = "3")]
	ClearingBroker,
	/// Retail
	#[serde(rename = "4")]
	Retail,
	/// Sales commission
	#[serde(rename = "5")]
	SalesCommission,
	/// Local commission
	#[serde(rename = "6")]
	LocalCommission,
	/// Research Payment
	#[serde(rename = "7")]
	ResearchPayment,
}

impl Default for AllocCommissionAmountType {
	fn default() -> Self {
		AllocCommissionAmountType::Unspecified
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum AllocCommissionBasis {
	/// Per unit
	#[serde(rename = "1")]
	PerUnit,
	/// Percent
	#[serde(rename = "2")]
	Percent,
	/// Absolute
	#[serde(rename = "3")]
	Absolute,
}

impl Default for AllocCommissionBasis {
	fn default() -> Self {
		AllocCommissionBasis::PerUnit
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum AllocCommissionAmountSubType {
	/// Research payment (RPA)
	#[serde(rename = "0")]
	ResearchPayment,
	/// Comission sharing agreement (CSA)
	#[serde(rename = "1")]
	ComissionSharingAgreement,
	/// Other
	#[serde(rename = "2")]
	Other,
}

impl Default for AllocCommissionAmountSubType {
	fn default() -> Self {
		AllocCommissionAmountSubType::ResearchPayment
	}
}
