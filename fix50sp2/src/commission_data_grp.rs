
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CommissionDataGrp {
	/// NoCommissions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2639")]
	pub commissions: Option<fix_common::RepeatingValues<Commission>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Commission {
	/// Required if NoCommissions(2639) &gt; 0. If the commission is based on a percentage of trade quantity or a factor of "unit of
	/// measure", CommissionRate(2646) and CommissionUnitOfMeasure(2644) may also be specified as appropriate.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2640")]
	pub commission_amount: Option<f64>,
	/// Required if NoCommissions(2639) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2641")]
	pub commission_amount_type: Option<CommissionAmountType>,
	/// Required if NoCommissions(2639) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2642")]
	pub commission_basis: Option<CommissionBasis>,
	/// CommissionCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2643")]
	pub commission_currency: Option<String>,
	/// CommissionUnitOfMeasure
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2644")]
	pub commission_unit_of_measure: Option<String>,
	/// CommissionUnitOfMeasureCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2645")]
	pub commission_unit_of_measure_currency: Option<String>,
	/// CommissionRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2646")]
	pub commission_rate: Option<f64>,
	/// CommissionSharedIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2647")]
	pub commission_shared_indicator: Option<fix_common::Boolean>,
	/// If specified, CommissionSharedIndicator(2647) must be set to "Y".
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2648")]
	pub commission_amount_shared: Option<f64>,
	/// This field may be used for multi-leg trades sent as a single message to indicate that the entry applies only to a specific
	/// leg.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2649")]
	pub commission_leg_ref_id: Option<String>,
	/// CommissionDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2650")]
	pub commission_desc: Option<String>,
	/// Must be set if EncodedCommissionDesc(2652) is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2651")]
	pub encoded_commission_desc_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the CommissionDesc(2650) field in the encoded format specified via the MessageEncoding(347)
	/// field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2652")]
	pub encoded_commission_desc: Option<String>,
	/// CommissionAmountSubType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2725")]
	pub commission_amount_sub_type: Option<CommissionAmountSubType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CommissionAmountType {
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
	/// Research payment
	#[serde(rename = "7")]
	ResearchPayment,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CommissionBasis {
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CommissionAmountSubType {
	/// Research payment account (RPA)
	#[serde(rename = "0")]
	ResearchPaymentAccount,
	/// Comission sharing agreement (CSA)
	#[serde(rename = "1")]
	ComissionSharingAgreement,
	/// Other type of research payment
	#[serde(rename = "2")]
	OtherTypeOfResearchPayment,
}
